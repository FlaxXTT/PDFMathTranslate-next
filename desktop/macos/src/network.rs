use std::path::Path;
use std::time::Duration;

use reqwest::{Client, Url, header::CONTENT_DISPOSITION, redirect::Policy};
use serde::Serialize;
use serde_json::{Value, json};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

use crate::profiles::validate_endpoint;

const INVALID_LINK: &str = "Enter a valid HTTP or HTTPS URL.";
const LINK_TIMEOUT: &str = "The URL request timed out. Try again.";
const LINK_FAILED: &str = "Could not load the URL. Try again.";

pub(crate) fn http_url(input: &str) -> Option<Url> {
    let parsed = Url::parse(input).ok()?;
    let (_, address) = input.split_once("://")?;
    let authority = address.split(['/', '?', '#']).next()?;
    if !matches!(parsed.scheme(), "http" | "https")
        || !parsed.has_host()
        || authority.is_empty()
        || authority.contains(['@', '\\'])
        || parsed.port() == Some(0)
        || input
            .chars()
            .any(|character| character.is_whitespace() || character.is_control())
    {
        return None;
    }
    Some(parsed)
}

#[derive(Serialize)]
pub(crate) struct RemotePdf {
    pub(crate) final_url: String,
    pub(crate) content_disposition: Option<String>,
}

pub(crate) async fn download_pdf(url: &str, path: &Path) -> Result<RemotePdf, String> {
    let url = http_url(url.trim()).ok_or(INVALID_LINK)?;
    let client = Client::builder()
        .user_agent("Hermeneuein")
        .connect_timeout(Duration::from_secs(6))
        .read_timeout(Duration::from_secs(8))
        .redirect(Policy::limited(5))
        .build()
        .map_err(link_error)?;
    let mut response = client
        .get(url)
        .send()
        .await
        .map_err(link_error)?
        .error_for_status()
        .map_err(link_error)?;
    let metadata = RemotePdf {
        final_url: response.url().to_string(),
        content_disposition: response
            .headers()
            .get(CONTENT_DISPOSITION)
            .map(|value| String::from_utf8_lossy(value.as_bytes()).into_owned()),
    };
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .await
        .map_err(|_| "Could not save the imported file.")?;
    while let Some(chunk) = response.chunk().await.map_err(link_error)? {
        file.write_all(&chunk)
            .await
            .map_err(|_| "Could not save the imported file.")?;
    }
    file.flush().await.map_err(|_| "Could not save the imported file.")?;
    Ok(metadata)
}

fn link_error(error: reqwest::Error) -> &'static str {
    if error.is_timeout() {
        LINK_TIMEOUT
    } else {
        LINK_FAILED
    }
}

pub(crate) async fn test_provider(url: &str, model: &str, key: &str) -> Result<String, String> {
    let (url, model) = validate_endpoint(url, model)?;
    let key = key.trim();
    if key.is_empty()
        || !key.is_ascii()
        || key
            .bytes()
            .any(|byte| byte.is_ascii_whitespace() || byte.is_ascii_control())
    {
        return Err("Enter a valid API key.".into());
    }
    let client = Client::builder()
        .user_agent("Hermeneuein")
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(15))
        .redirect(Policy::none())
        .build()
        .map_err(provider_error)?;
    let response = client
        .post(format!("{url}/chat/completions"))
        .bearer_auth(key)
        .json(&json!({"model": model, "messages": [{"role": "user", "content": "Reply with OK."}]}))
        .send()
        .await
        .map_err(provider_error)?;
    let status = response.status();
    if !status.is_success() {
        return Err(match status.as_u16() {
            401 | 403 => "Authentication failed. Check your API key and model access.".into(),
            400 | 404 | 405 | 422 => "The request failed. Check the API base URL and model name.".into(),
            429 => "Request limit reached. Check your quota or try again later.".into(),
            code => format!("The service returned HTTP {code}."),
        });
    }
    let reply: Value = response.json().await.map_err(|error| {
        if error.is_decode() {
            "The service did not return a usable response."
        } else {
            provider_error(error)
        }
    })?;
    if reply["choices"][0]["message"]["content"]
        .as_str()
        .is_none_or(|text| text.trim().is_empty())
    {
        return Err("The service did not return a usable response.".into());
    }
    Ok("Connection successful.".into())
}

fn provider_error(error: reqwest::Error) -> &'static str {
    if error.is_timeout() {
        "Connection timed out. Try again."
    } else {
        "Could not connect. Check the API URL and your connection."
    }
}
