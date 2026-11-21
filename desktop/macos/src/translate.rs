use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use futures_util::{StreamExt, stream};
use reqwest::{Client, Response, StatusCode, redirect::Policy};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::sync::OnceCell;

use crate::workbench::Options;

#[path = "translation_recovery.rs"]
mod recovery;

const ENDPOINTS: [&str; 2] = [
    "https://api1.pdf2zh-next.com/chatproxy",
    "https://api2.pdf2zh-next.com/chatproxy",
];
const MAX_SPLIT_DEPTH: u8 = 3;

#[derive(Clone, Default, Deserialize, Serialize)]
pub(crate) enum Service {
    #[default]
    SiliconFlowFree,
    Custom {
        name: String,
        base_url: String,
        model: String,
        key: String,
    },
}

impl Service {
    pub(crate) fn describe(&self) -> Value {
        match self {
            Self::SiliconFlowFree => {
                json!({"name":"SiliconFlowFree", "base_url":"", "model":"", "has_key":false})
            }
            Self::Custom {
                name,
                base_url,
                model,
                key,
            } => {
                json!({"name":name, "base_url":base_url, "model":model, "has_key":!key.is_empty()})
            }
        }
    }
}

pub(crate) struct Translator {
    service: Service,
    client: Client,
    free_endpoints: OnceCell<[&'static str; 2]>,
}

pub(crate) struct SegmentTranslation {
    pub(crate) text: String,
    pub(crate) kept_source: bool,
}

enum CompletionError {
    Incomplete,
    Failed(String),
}

impl Translator {
    pub(crate) fn new(service: Service) -> Result<Self, String> {
        let client = Client::builder()
            .user_agent("Hermeneuein")
            .connect_timeout(Duration::from_secs(8))
            .timeout(Duration::from_secs(60))
            .redirect(Policy::none())
            .build()
            .map_err(|_| "Could not initialize the translation service.")?;
        Ok(Self {
            service,
            client,
            free_endpoints: OnceCell::new(),
        })
    }

    pub(crate) async fn batch(
        &self,
        texts: &[String],
        terms: &[(String, String)],
        opts: &Options,
        cancelled: &AtomicBool,
    ) -> Result<Vec<SegmentTranslation>, String> {
        let glossary = terms
            .iter()
            .map(|(source, target)| format!("{source} → {target}"))
            .collect::<Vec<_>>()
            .join("\n");
        let parallelism = if matches!(self.service, Service::SiliconFlowFree) {
            2
        } else {
            8
        };
        let mut requests = std::pin::pin!(
            stream::iter(
                texts
                    .iter()
                    .map(|text| self.translate(text, &glossary, opts, cancelled, 0))
            )
            .buffered(parallelism)
        );
        let mut translated = Vec::with_capacity(texts.len());
        while let Some(next) = requests.next().await {
            translated.push(next?);
        }
        Ok(translated)
    }

    async fn translate(
        &self,
        text: &str,
        glossary: &str,
        opts: &Options,
        cancelled: &AtomicBool,
        depth: u8,
    ) -> Result<SegmentTranslation, String> {
        if cancelled.load(Ordering::Relaxed) {
            return Err("Canceled".into());
        }
        if text.trim().is_empty() {
            return Ok(SegmentTranslation {
                text: text.to_owned(),
                kept_source: false,
            });
        }
        match self.translate_once(text, glossary, opts, cancelled).await {
            Ok(reply) if !recovery::looks_incomplete(text, &reply, depth > 0) => {
                return Ok(SegmentTranslation {
                    text: reply,
                    kept_source: false,
                });
            }
            Ok(_) | Err(CompletionError::Incomplete) => {}
            Err(CompletionError::Failed(error)) => return Err(error),
        }

        // Never export a partial reply. Preserve the original only for a piece
        // that still fails after bounded, paragraph-local recovery.
        if depth < MAX_SPLIT_DEPTH
            && let Some((left, separator, right)) = recovery::split(text)
        {
            let left = Box::pin(self.translate(left, glossary, opts, cancelled, depth + 1)).await?;
            let right =
                Box::pin(self.translate(right, glossary, opts, cancelled, depth + 1)).await?;
            return Ok(SegmentTranslation {
                text: [left.text.as_str(), separator, right.text.as_str()].concat(),
                kept_source: left.kept_source || right.kept_source,
            });
        }
        Ok(SegmentTranslation {
            text: text.to_owned(),
            kept_source: true,
        })
    }

    async fn translate_once(
        &self,
        text: &str,
        glossary: &str,
        opts: &Options,
        cancelled: &AtomicBool,
    ) -> Result<String, CompletionError> {
        if matches!(self.service, Service::SiliconFlowFree) {
            let target = if opts.to == "zh-CN" { "zh" } else { &opts.to };
            let prompt = format!(
                "You are a professional,authentic machine translation engine.\n\n;; Treat next line as plain text input and translate it into {target}, output translation ONLY. If translation is unnecessary (e.g. proper nouns, codes), return the original text. NO explanations. NO notes. Input:\n\n{text}"
            );
            return self.complete(&prompt, "", false, cancelled).await;
        }
        let from = language(&opts.from);
        let to = language(&opts.to);
        let mut prompt = format!(
            "Translate this PDF text from {from} into {to}. Output only the translated text. Preserve mathematical formulas and every [[BD_F_N]], [[BD_L_N]], [[BD_R_N]] placeholder exactly."
        );
        if !glossary.is_empty() {
            prompt.push_str("\nUse these terminology translations where appropriate:\n");
            prompt.push_str(glossary);
        }
        prompt.push_str("\nInput:\n\n");
        prompt.push_str(text);
        self.complete(&prompt, opts.prompt.trim(), false, cancelled)
            .await
    }

    pub(crate) async fn extract_terms(
        &self,
        prompt: &str,
        opts: &Options,
        cancelled: &AtomicBool,
    ) -> Result<String, String> {
        self.complete(prompt, "", opts.json_mode, cancelled)
            .await
            .map_err(|error| match error {
                CompletionError::Incomplete => "The translation service returned an incomplete glossary.".into(),
                CompletionError::Failed(error) => error,
            })
    }

    async fn complete(
        &self,
        prompt: &str,
        system: &str,
        structured: bool,
        cancelled: &AtomicBool,
    ) -> Result<String, CompletionError> {
        let request = async {
            match &self.service {
                Service::SiliconFlowFree => {
                    let combined;
                    let prompt = if system.is_empty() {
                        prompt
                    } else {
                        combined =
                            format!("{prompt}\n\nAdditional translation instructions: {system}");
                        &combined
                    };
                    self.free(prompt, structured).await
                }
                Service::Custom {
                    base_url,
                    model,
                    key,
                    ..
                } => {
                    let messages = if system.is_empty() {
                        json!([{"role":"user", "content":prompt}])
                    } else {
                        json!([{"role":"system", "content":system}, {"role":"user", "content":prompt}])
                    };
                    let response = self
                        .client
                        .post(format!("{base_url}/chat/completions"))
                        .bearer_auth(key)
                        .json(&json!({"model": model, "messages":messages}))
                        .send()
                        .await
                        .map_err(|error| CompletionError::Failed(network_error(error)))?;
                    if !response.status().is_success() {
                        return Err(request_failure(response).await);
                    }
                    let reply: Value = response
                        .json()
                        .await
                        .map_err(|error| CompletionError::Failed(network_error(error)))?;
                    reply_text(&reply, &reply["choices"][0]["message"]["content"])
                }
            }
        };
        tokio::select! {
            value = request => value,
            _ = wait_for_cancellation(cancelled) => Err(CompletionError::Failed("Canceled".into())),
        }
    }

    async fn free(&self, prompt: &str, json_mode: bool) -> Result<String, CompletionError> {
        let endpoints = self
            .free_endpoints
            .get_or_init(|| async {
                let (first, second) =
                    tokio::join!(self.health(ENDPOINTS[0]), self.health(ENDPOINTS[1]));
                if second.is_some() && (first.is_none() || second < first) {
                    [ENDPOINTS[1], ENDPOINTS[0]]
                } else {
                    ENDPOINTS
                }
            })
            .await;
        let mut last_error = "SiliconFlow Free is temporarily unavailable.".to_owned();
        for (attempt, endpoint) in endpoints.iter().cycle().take(4).enumerate() {
            if attempt >= 2 {
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
            let request = if json_mode {
                json!({"text": prompt, "requestJsonMode": true})
            } else {
                json!({"text": prompt})
            };
            let response = match self
                .client
                .post(*endpoint)
                .timeout(Duration::from_secs(42))
                .json(&request)
                .send()
                .await
            {
                Ok(response) => response,
                Err(error) => {
                    last_error = network_error(error);
                    continue;
                }
            };
            let status = response.status();
            if !status.is_success() {
                let CompletionError::Failed(error) = request_failure(response).await else {
                    return Err(CompletionError::Incomplete);
                };
                last_error = error;
                if status.as_u16() == 400
                    || status == StatusCode::TOO_MANY_REQUESTS
                    || status.is_server_error()
                {
                    continue;
                }
                return Err(CompletionError::Failed(last_error));
            }
            let reply: Value = response
                .json()
                .await
                .map_err(|error| CompletionError::Failed(network_error(error)))?;
            return reply_text(&reply, &reply["content"]);
        }
        Err(CompletionError::Failed(last_error))
    }

    async fn health(&self, endpoint: &str) -> Option<Duration> {
        let start = std::time::Instant::now();
        let response = self
            .client
            .post(format!("{endpoint}/check"))
            .timeout(Duration::from_secs(5))
            .send()
            .await
            .ok()?;
        if !response.status().is_success() {
            return None;
        }
        let status: Value = response.json().await.ok()?;
        (status["status"].as_str() == Some("ok")).then(|| start.elapsed())
    }
}

fn reply_text(reply: &Value, content: &Value) -> Result<String, CompletionError> {
    let choice = &reply["choices"][0];
    let reasons = [
        &choice["finish_reason"],
        &choice["finishReason"],
        &reply["finish_reason"],
        &reply["finishReason"],
        &reply["stop_reason"],
        &reply["stopReason"],
        &reply["incomplete_details"]["reason"],
    ];
    if reasons
        .iter()
        .filter_map(|value| value.as_str())
        .any(|reason| {
            ["length", "max_tokens", "max_output_tokens"]
                .iter()
                .any(|limit| reason.eq_ignore_ascii_case(limit))
        })
        || [&reply["error"]["code"], &reply["error"]["message"]]
            .into_iter()
            .filter_map(Value::as_str)
            .any(is_context_limit)
    {
        return Err(CompletionError::Incomplete);
    }
    content
        .as_str()
        .map(clean_reply)
        .filter(|text| !text.is_empty())
        .ok_or_else(|| CompletionError::Failed("The translation service did not return usable text.".into()))
}

async fn request_failure(response: Response) -> CompletionError {
    let status = response.status();
    if status == StatusCode::PAYLOAD_TOO_LARGE
        || (matches!(
            status,
            StatusCode::BAD_REQUEST | StatusCode::UNPROCESSABLE_ENTITY
        ) && response
            .text()
            .await
            .is_ok_and(|body| is_context_limit(&body)))
    {
        return CompletionError::Incomplete;
    }
    CompletionError::Failed(service_error(status))
}

fn is_context_limit(body: &str) -> bool {
    let body = body.to_ascii_lowercase();
    [
        "context_length_exceeded",
        "context_window_exceeded",
        "maximum context length",
        "exceeds the context window",
        "context length exceeded",
        "input is too long",
        "input too long",
        "prompt is too long",
        "maximum input length",
    ]
    .iter()
    .any(|signal| body.contains(signal))
}

pub(crate) async fn wait_for_cancellation(cancelled: &AtomicBool) {
    while !cancelled.load(Ordering::Relaxed) {
        tokio::time::sleep(Duration::from_millis(70)).await;
    }
}

fn service_error(status: StatusCode) -> String {
    match status.as_u16() {
        401 | 403 => "Authentication failed. Check your API key and model access.".into(),
        429 => "The translation service is rate limiting requests. Try again later.".into(),
        code => format!("The translation service returned HTTP {code}."),
    }
}

fn network_error(error: reqwest::Error) -> String {
    if error.is_timeout() {
        "The translation service timed out."
    } else {
        "Could not connect to the translation service."
    }
    .into()
}

fn clean_reply(raw: &str) -> String {
    let text = raw.rsplit("</think>").next().unwrap_or(raw).trim();
    text.strip_prefix("```text\n")
        .or_else(|| text.strip_prefix("```json\n"))
        .or_else(|| text.strip_prefix("```\n"))
        .and_then(|text| text.strip_suffix("```"))
        .unwrap_or(text)
        .trim()
        .into()
}

fn language(code: &str) -> &str {
    match code {
        "auto" => "the detected source language",
        "zh" | "zh-CN" => "Simplified Chinese",
        "zh-TW" => "Traditional Chinese (Taiwan)",
        "zh-HK" => "Traditional Chinese (Hong Kong)",
        "en" => "English",
        "ja" => "Japanese",
        "ko" => "Korean",
        "fr" => "French",
        "de" => "German",
        "es" => "Spanish",
        "it" => "Italian",
        "pt" => "Portuguese",
        "ru" => "Russian",
        code => code,
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::{CompletionError, Service, Translator, is_context_limit, reply_text};
    use crate::workbench::Options;
    use serde_json::{Value, json};

    #[test]
    fn reads_length_limits_from_openai_choices_and_proxy_replies() {
        let standard = json!({"choices": [{
            "message": {"content": "A nonempty prefix"},
            "finish_reason": "length",
        }]});
        assert!(matches!(
            reply_text(&standard, &standard["choices"][0]["message"]["content"]),
            Err(CompletionError::Incomplete)
        ));
        let proxy = json!({"content": "一段译文的开头", "finishReason": "MAX_TOKENS"});
        assert!(matches!(
            reply_text(&proxy, &proxy["content"]),
            Err(CompletionError::Incomplete)
        ));
        let stopped = json!({"content": "完整译文。"});
        assert_eq!(
            reply_text(&stopped, &stopped["content"]).ok().as_deref(),
            Some("完整译文。")
        );
    }

    #[test]
    fn classifies_context_errors_without_folding_other_bad_requests_into_recovery() {
        assert!(is_context_limit(
            r#"{"error":{"code":"context_length_exceeded"}}"#
        ));
        assert!(is_context_limit(
            "This model's maximum context length is 4096 tokens."
        ));
        assert!(!is_context_limit(r#"{"error":{"code":"invalid_model"}}"#));
        assert!(!is_context_limit(
            "The max_tokens parameter must be positive."
        ));
    }

    #[test]
    fn free_gateway_recovers_a_short_reply_without_replaying_healthy_paragraphs() {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let source =
                    "This original sentence must remain present in the recovered paragraph. "
                        .repeat(12)
                        .trim_end()
                        .to_owned();
                let sibling = "This nearby paragraph already has a complete reply.".to_owned();
                let calls = Arc::new(Mutex::new(Vec::new()));
                let received = Arc::clone(&calls);
                let incomplete_input = source.clone();
                let route = axum::routing::post(move |axum::Json(request): axum::Json<Value>| {
                    let received = Arc::clone(&received);
                    let incomplete_input = incomplete_input.clone();
                    async move {
                        let (_, input) = request["text"]
                            .as_str()
                            .unwrap()
                            .split_once("Input:\n\n")
                            .unwrap();
                        received.lock().unwrap().push(input.to_owned());
                        axum::Json(json!({
                            "content": if input == incomplete_input { "译文尚未" } else { input },
                        }))
                    }
                });
                let listener = tokio::net::TcpListener::bind(("127.0.0.1", 0))
                    .await
                    .unwrap();
                let endpoint: &'static str =
                    format!("http://{}/chatproxy", listener.local_addr().unwrap()).leak();
                let server = tokio::spawn(async move {
                    axum::serve(listener, axum::Router::new().route("/chatproxy", route))
                        .await
                        .unwrap();
                });
                let mut translator = Translator::new(Service::SiliconFlowFree).unwrap();
                translator.client = reqwest::Client::builder().no_proxy().build().unwrap();
                translator.free_endpoints.set([endpoint, endpoint]).unwrap();
                let results = translator
                    .batch(
                        &[source.clone(), sibling.clone()],
                        &[],
                        &Options::default(),
                        &std::sync::atomic::AtomicBool::new(false),
                    )
                    .await
                    .unwrap();
                assert_eq!(results[0].text, source);
                assert!(!results[0].kept_source);
                assert_eq!(results[1].text, sibling);
                let calls = calls.lock().unwrap();
                assert_eq!(calls.len(), 4);
                assert_eq!(calls.iter().filter(|call| **call == source).count(), 1);
                assert_eq!(calls.iter().filter(|call| **call == sibling).count(), 1);
                server.abort();
            });
    }
}
