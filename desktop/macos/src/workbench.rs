use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{
    Arc, Mutex, MutexGuard,
    atomic::{AtomicBool, Ordering},
};

use axum::body::Body;
use axum::extract::{Path as Route, Query, Request, State};
use axum::http::{HeaderMap, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use http_body_util::BodyExt;
use percent_encoding::{NON_ALPHANUMERIC, percent_decode_str, utf8_percent_encode};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tempfile::TempDir;
use tokio::io::AsyncWriteExt;
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};
use uuid::Uuid;

use crate::archives::{self, ArchiveDocument};
use crate::network;
use crate::pdf_engine::{self, PdfOptions, PdfResult};
use crate::profiles;
use crate::translate::{Service, Translator, wait_for_cancellation};

#[derive(Clone, Deserialize, Serialize)]
#[serde(default)]
pub(crate) struct Options {
    pub from: String,
    pub to: String,
    pub pages: String,
    pub format: String,
    pub translated_only: bool,
    pub alternate: bool,
    pub translated_first: bool,
    pub json_mode: bool,
    pub prompt: String,
    pub glossary: bool,
    pub min_text_length: usize,
    pub font_family: Option<String>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            from: "en".into(),
            to: "zh-CN".into(),
            pages: String::new(),
            format: "both".into(),
            translated_only: false,
            alternate: false,
            translated_first: false,
            json_mode: false,
            prompt: String::new(),
            glossary: false,
            min_text_length: 5,
            font_family: None,
        }
    }
}

impl Options {
    fn validate(self) -> Result<Self, ApiError> {
        if !matches!(self.format.as_str(), "both" | "mono" | "dual") {
            return Err(ApiError::bad("Select an output format."));
        }
        if self.to.trim().is_empty() || self.to == "auto" {
            return Err(ApiError::bad("Select a target language."));
        }
        Ok(self)
    }

    fn pdf_options(&self) -> PdfOptions {
        PdfOptions {
            pages: self.pages.clone(),
            lang_out: self.to.clone(),
            mono: self.format != "dual",
            dual: self.format != "mono",
            alternating: self.alternate,
            translated_first: self.translated_first,
            only_selected: self.translated_only,
            min_text_length: self.min_text_length,
            font_family: self.font_family.clone(),
            glossary: self.glossary,
        }
    }
}

#[derive(Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum Source {
    File,
    Link,
}

struct Document {
    id: String,
    name: String,
    original: PathBuf,
    source: Source,
    status: &'static str,
    result: Option<PdfResult>,
    retained_paragraphs: usize,
    _directory: TempDir,
}

impl Document {
    fn file(&self, kind: &str) -> Option<&Path> {
        if kind == "original" {
            return Some(&self.original);
        }
        let result = self.result.as_ref()?;
        match kind {
            "mono" => result.mono.as_deref(),
            "dual" => result.dual.as_deref(),
            "glossary" => result.glossary.as_deref(),
            _ => None,
        }
    }

    fn public(&self) -> Value {
        let versions: BTreeMap<_, _> = ["original", "mono", "dual", "glossary"]
            .into_iter()
            .filter(|kind| self.file(kind).is_some())
            .map(|kind| (kind, format!("/api/files/{}/{kind}", self.id)))
            .collect();
        json!({
            "id": self.id, "name": self.name, "source": self.source, "status": self.status,
            "versions": versions, "retained_paragraphs": self.retained_paragraphs,
        })
    }
}

#[derive(Clone, Serialize)]
struct JobStatus {
    status: &'static str,
    done: usize,
    total: usize,
    percent: u8,
    message: String,
    error: Option<String>,
    retained_paragraphs: usize,
}

struct Job {
    info: JobStatus,
    flag: Arc<AtomicBool>,
    pending: Vec<String>,
    options: Options,
    source: Source,
}

impl Job {
    fn busy(&self) -> bool {
        matches!(self.info.status, "translating" | "cancelling")
    }
}

struct Batch {
    files: BTreeMap<String, PathBuf>,
    _directory: TempDir,
}

struct Store {
    docs: Vec<Document>,
    options: Options,
    service: Service,
    job: Option<Job>,
    import: Option<Arc<AtomicBool>>,
    archives: Option<Batch>,
}

pub(crate) struct Workbench {
    store: Mutex<Store>,
    scratch: TempDir,
    assets: PathBuf,
    profiles: PathBuf,
    settings: PathBuf,
}

impl Workbench {
    pub(crate) fn new(
        data: &Path,
        assets: &Path,
        legacy_profiles: Option<&Path>,
        legacy_settings: Option<&Path>,
    ) -> std::io::Result<Arc<Self>> {
        fs::create_dir_all(data)?;
        let settings = data.join("settings.json");
        let profile_path = data.join("service_profiles.json");
        if !profile_path.exists() {
            if let Some(legacy) = legacy_profiles.filter(|path| path.is_file()) {
                if let Ok(saved) = profiles::load(legacy) {
                    let mut file = tempfile::NamedTempFile::new_in(data)?;
                    serde_json::to_writer_pretty(&mut file, &saved)?;
                    let _ = file.persist_noclobber(&profile_path);
                }
            }
        }
        let saved = fs::read(&settings)
            .ok()
            .and_then(|bytes| serde_json::from_slice::<Preferences>(&bytes).ok())
            .or_else(|| legacy_settings.and_then(|path| previous_settings(path, &profile_path)));
        let (options, service) = saved
            .map(|saved| (saved.options, saved.service))
            .unwrap_or_default();
        Ok(Arc::new(Self {
            store: Mutex::new(Store {
                docs: Vec::new(),
                options,
                service,
                job: None,
                import: None,
                archives: None,
            }),
            scratch: tempfile::Builder::new().prefix("hermeneuein-").tempdir()?,
            assets: assets.to_owned(),
            profiles: profile_path,
            settings,
        }))
    }

    fn lock(&self) -> MutexGuard<'_, Store> {
        self.store
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    pub(crate) fn shutdown(&self) {
        let store = self.lock();
        if let Some(job) = &store.job {
            job.flag.store(true, Ordering::Relaxed);
        }
        if let Some(import) = &store.import {
            import.store(true, Ordering::Relaxed);
        }
    }
}

#[derive(Debug)]
struct ApiError(StatusCode, String);

impl ApiError {
    fn bad(message: impl Into<String>) -> Self {
        Self(StatusCode::BAD_REQUEST, message.into())
    }
    fn busy() -> Self {
        Self(StatusCode::CONFLICT, "Wait for the current task to finish.".into())
    }
    fn missing() -> Self {
        Self(StatusCode::NOT_FOUND, "Could not find this file.".into())
    }
    fn disk() -> Self {
        Self(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not read or write a local file.".into(),
        )
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.0, Json(json!({"error": self.1}))).into_response()
    }
}

pub(crate) fn router(state: Arc<Workbench>, web: PathBuf, host: String) -> Router {
    Router::new()
        .route("/api/state", get(state_response))
        .route("/api/documents/upload", post(upload_pdf))
        .route("/api/documents/link", post(link_pdf))
        .route("/api/documents/{id}", delete(delete_pdf))
        .route("/api/jobs", post(start_job))
        .route("/api/jobs/retry", post(retry_job))
        .route("/api/jobs/cancel", post(cancel_job))
        .route("/api/profiles", post(save_profile))
        .route("/api/profiles/test", post(test_profile))
        .route("/api/profiles/{name}", delete(delete_profile))
        .route("/api/service", post(select_service))
        .route("/api/settings", post(save_settings))
        .route("/api/files/{id}/{kind}", get(pdf_file))
        .route("/api/archives/{kind}", get(archive_file))
        .with_state(state)
        .fallback_service(ServeDir::new(web).append_index_html_on_directories(true))
        .layer(middleware::from_fn(move |request, next| {
            guard(request, next, host.clone())
        }))
}

async fn guard(request: Request, next: Next, host: String) -> Response {
    let valid_host = request
        .headers()
        .get(header::HOST)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|header_host| header_host == host);
    let valid_action = request.method().is_safe()
        || request
            .headers()
            .get("x-hermeneuein")
            .is_some_and(|value| value == "1");
    if !valid_host || !valid_action {
        return StatusCode::FORBIDDEN.into_response();
    }
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        header::X_CONTENT_TYPE_OPTIONS,
        header::HeaderValue::from_static("nosniff"),
    );
    response
}

async fn state_response(State(app): State<Arc<Workbench>>) -> Result<Json<Value>, ApiError> {
    let profiles = profiles::load(&app.profiles).map_err(ApiError::bad)?;
    let store = app.lock();
    let documents: Vec<_> = store.docs.iter().map(Document::public).collect();
    let archives: BTreeMap<_, _> = store
        .archives
        .iter()
        .flat_map(|batch| batch.files.keys())
        .map(|kind| (kind, format!("/api/archives/{kind}?download=1")))
        .collect();
    let job = store.job.as_ref().map(|job| &job.info);
    let service = store.service.describe();
    Ok(Json(json!({
        "documents":documents, "profiles":profiles, "service":service,
        "active_service":service["name"], "job":job, "archives":archives,
        "settings":store.options, "importing":store.import.is_some(),
    })))
}

fn ensure_idle(store: &Store) -> Result<(), ApiError> {
    if store.job.as_ref().is_some_and(Job::busy) || store.import.is_some() {
        Err(ApiError::busy())
    } else {
        Ok(())
    }
}

fn cleaned_name(raw: &str) -> String {
    let name = raw.rsplit(['/', '\\']).next().unwrap_or_default().trim();
    let stem = name
        .get(..name.len().saturating_sub(4))
        .filter(|_| name.to_ascii_lowercase().ends_with(".pdf"))
        .unwrap_or(name);
    let stem: String = stem
        .chars()
        .filter(|value| !value.is_control())
        .take(120)
        .collect();
    let stem = stem.trim_matches(['.', ' ']);
    format!("{}.pdf", if stem.is_empty() { "document" } else { stem })
}

fn unique_name(store: &Store, name: String, source: Source) -> String {
    let exists = |name: &str| {
        store
            .docs
            .iter()
            .any(|doc| doc.source == source && doc.name.to_lowercase() == name.to_lowercase())
    };
    if !exists(&name) {
        return name;
    }
    let stem = name.trim_end_matches(".pdf");
    (2..)
        .map(|index| format!("{stem} ({index}).pdf"))
        .find(|candidate| !exists(candidate))
        .expect("finite document list")
}

async fn validate_pdf(path: PathBuf) -> Result<(), ApiError> {
    tokio::task::spawn_blocking(move || pdf_engine::check_pdf(&path))
        .await
        .map_err(|_| ApiError::bad("Could not open this PDF."))?
        .map_err(ApiError::bad)
}

async fn upload_pdf(
    State(app): State<Arc<Workbench>>,
    headers: HeaderMap,
    mut body: Body,
) -> Result<Json<Value>, ApiError> {
    ensure_idle(&app.lock())?;
    let raw = headers
        .get("x-filename")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("document.pdf");
    let name = cleaned_name(&percent_decode_str(raw).decode_utf8_lossy());
    let directory = tempfile::tempdir_in(app.scratch.path()).map_err(|_| ApiError::disk())?;
    let original = directory.path().join("original.pdf");
    let mut file = tokio::fs::File::create(&original)
        .await
        .map_err(|_| ApiError::disk())?;
    while let Some(frame) = body.frame().await {
        let frame = frame.map_err(|_| ApiError::bad("File upload was interrupted."))?;
        if let Ok(data) = frame.into_data() {
            file.write_all(&data).await.map_err(|_| ApiError::disk())?;
        }
    }
    file.flush().await.map_err(|_| ApiError::disk())?;
    drop(file);
    validate_pdf(original.clone()).await?;
    let mut store = app.lock();
    ensure_idle(&store)?;
    Ok(Json(add_document(
        &mut store,
        name,
        Source::File,
        original,
        directory,
    )))
}

#[derive(Deserialize)]
struct LinkRequest {
    url: String,
}

async fn link_pdf(
    State(app): State<Arc<Workbench>>,
    Json(request): Json<LinkRequest>,
) -> Result<Json<Value>, ApiError> {
    let flag = Arc::new(AtomicBool::new(false));
    {
        let mut store = app.lock();
        ensure_idle(&store)?;
        store.import = Some(flag.clone());
    }
    // The guard also resets the import state if the HTTP request is dropped.
    struct ImportGuard(Arc<Workbench>);
    impl Drop for ImportGuard {
        fn drop(&mut self) {
            self.0.lock().import = None;
        }
    }
    let _guard = ImportGuard(app.clone());
    let directory = tempfile::tempdir_in(app.scratch.path()).map_err(|_| ApiError::disk())?;
    let original = directory.path().join("original.pdf");
    let metadata = tokio::select! {
        result = network::download_pdf(&request.url, &original) => result.map_err(ApiError::bad)?,
        _ = wait_for_cancellation(&flag) => return Err(ApiError::bad("Canceled")),
    };
    validate_pdf(original.clone()).await?;
    if flag.load(Ordering::Relaxed) {
        return Err(ApiError::bad("Canceled"));
    }
    let name = link_name(&metadata);
    let mut store = app.lock();
    store.docs.retain(|doc| doc.source != Source::Link);
    let document = add_document(&mut store, name, Source::Link, original, directory);
    drop(store);
    Ok(Json(document))
}

fn link_name(metadata: &network::RemotePdf) -> String {
    if let Some(disposition) = metadata.content_disposition.as_deref() {
        let mut plain = None;
        for part in disposition.split(';').skip(1) {
            if let Some((name, value)) = part.trim().split_once('=') {
                let value = value.trim_matches('"');
                if name.eq_ignore_ascii_case("filename*") {
                    if let Some(value) = value.splitn(3, '\'').nth(2) {
                        return cleaned_name(&percent_decode_str(value).decode_utf8_lossy());
                    }
                } else if name.eq_ignore_ascii_case("filename") {
                    plain = Some(value);
                }
            }
        }
        if let Some(plain) = plain {
            return cleaned_name(plain);
        }
    }
    let name = network::http_url(&metadata.final_url)
        .and_then(|url| {
            url.path_segments()?
                .rfind(|path| !path.is_empty())
                .map(str::to_owned)
        })
        .unwrap_or_else(|| "linked-document.pdf".into());
    cleaned_name(&percent_decode_str(&name).decode_utf8_lossy())
}

fn add_document(
    store: &mut Store,
    name: String,
    source: Source,
    original: PathBuf,
    directory: TempDir,
) -> Value {
    let document = Document {
        id: Uuid::new_v4().simple().to_string(),
        name: unique_name(store, name, source),
        original,
        source,
        status: "ready",
        result: None,
        retained_paragraphs: 0,
        _directory: directory,
    };
    let public = document.public();
    store.docs.push(document);
    public
}

async fn delete_pdf(
    State(app): State<Arc<Workbench>>,
    Route(id): Route<String>,
) -> Result<StatusCode, ApiError> {
    let mut store = app.lock();
    ensure_idle(&store)?;
    let index = store
        .docs
        .iter()
        .position(|doc| doc.id == id)
        .ok_or_else(ApiError::missing)?;
    store.docs.remove(index);
    if let Some(job) = &mut store.job {
        job.pending.retain(|pending| pending != &id);
    }
    store.archives = None;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
struct StartJob {
    document_ids: Vec<String>,
    options: Options,
}

#[derive(Clone)]
struct JobDocument {
    id: String,
    name: String,
    original: PathBuf,
    source: Source,
}

async fn start_job(
    State(app): State<Arc<Workbench>>,
    Json(request): Json<StartJob>,
) -> Result<StatusCode, ApiError> {
    schedule_job(app, request.document_ids, request.options.validate()?)
}

async fn retry_job(State(app): State<Arc<Workbench>>) -> Result<StatusCode, ApiError> {
    let (ids, options) = {
        let store = app.lock();
        ensure_idle(&store)?;
        let job = store
            .job
            .as_ref()
            .ok_or_else(|| ApiError::bad("There is no task to retry."))?;
        (job.pending.clone(), job.options.clone())
    };
    schedule_job(app, ids, options)
}

fn schedule_job(
    app: Arc<Workbench>,
    ids: Vec<String>,
    options: Options,
) -> Result<StatusCode, ApiError> {
    let (documents, translator, flag) = {
        let mut store = app.lock();
        ensure_idle(&store)?;
        let mut seen = HashSet::new();
        let documents: Vec<_> = ids
            .iter()
            .filter(|id| seen.insert(*id))
            .map(|id| {
                let doc = store
                    .docs
                    .iter()
                    .find(|doc| &doc.id == id)
                    .ok_or_else(ApiError::missing)?;
                Ok(JobDocument {
                    id: doc.id.clone(),
                    name: doc.name.clone(),
                    original: doc.original.clone(),
                    source: doc.source,
                })
            })
            .collect::<Result<_, ApiError>>()?;
        if documents.is_empty() {
            return Err(ApiError::bad("Add a PDF first."));
        }
        let source = documents[0].source;
        if documents.iter().any(|doc| doc.source != source) {
            return Err(ApiError::bad("Select files from the same source."));
        }
        let translator = Translator::new(store.service.clone()).map_err(ApiError::bad)?;
        store.archives = None;
        for document in &mut store.docs {
            if seen.contains(&document.id) {
                document.result = None;
                document.retained_paragraphs = 0;
                document.status = "ready";
            }
        }
        let flag = Arc::new(AtomicBool::new(false));
        store.options = options.clone();
        store.job = Some(Job {
            info: JobStatus {
                status: "translating",
                done: 0,
                total: documents.len(),
                percent: 0,
                message: "Preparing".into(),
                error: None,
                retained_paragraphs: 0,
            },
            flag: flag.clone(),
            pending: ids,
            options: options.clone(),
            source,
        });
        (documents, translator, flag)
    };
    let handle = tokio::runtime::Handle::current();
    tokio::task::spawn_blocking(move || {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            run_job(&app, &documents, &translator, &options, &flag, &handle)
        }))
        .unwrap_or_else(|_| Err("A typesetting error stopped the task.".into()));
        finish_job(&app, result, &flag);
    });
    Ok(StatusCode::ACCEPTED)
}

fn run_job(
    app: &Workbench,
    documents: &[JobDocument],
    translator: &Translator,
    options: &Options,
    flag: &AtomicBool,
    handle: &tokio::runtime::Handle,
) -> Result<(), String> {
    let opts = options.pdf_options();
    for target in documents {
        if flag.load(Ordering::Relaxed) {
            return Ok(());
        }
        {
            let mut store = app.lock();
            if let Some(doc) = store.docs.iter_mut().find(|doc| doc.id == target.id) {
                doc.status = "translating";
            }
            if let Some(job) = &mut store.job {
                job.info.percent = 0;
                job.info.message = target.name.clone();
            }
        }
        let output = target
            .original
            .parent()
            .expect("staged file directory")
            .join(format!("translation-{}", Uuid::new_v4().simple()));
        fs::create_dir(&output).map_err(|_| "Could not create the output directory.")?;
        let result = pdf_engine::run(
            &target.original,
            &output,
            &app.assets,
            &opts,
            flag,
            |percent, message| {
                if flag.load(Ordering::Relaxed) {
                    return;
                }
                if let Some(job) = &mut app.lock().job {
                    job.info.percent = percent;
                    job.info.message = message.to_owned();
                }
            },
            |texts, terms| {
                handle
                    .block_on(translator.batch(texts, terms, options, flag))
                    .map(|values| {
                        values
                            .into_iter()
                            .map(|value| (value.text, value.kept_source))
                            .collect()
                    })
            },
            |prompt| handle.block_on(translator.extract_terms(prompt, options, flag)),
        );
        let (result, retained_paragraphs) = match result {
            Ok(result) if !flag.load(Ordering::Relaxed) => result,
            Ok(_) => {
                let _ = fs::remove_dir_all(output);
                return Ok(());
            }
            Err(error) => {
                let _ = fs::remove_dir_all(output);
                return Err(error);
            }
        };
        let mut store = app.lock();
        if let Some(doc) = store.docs.iter_mut().find(|doc| doc.id == target.id) {
            doc.result = Some(result);
            doc.retained_paragraphs = retained_paragraphs;
            doc.status = "done";
        }
        if let Some(job) = &mut store.job {
            job.info.done += 1;
            job.info.retained_paragraphs += retained_paragraphs;
            job.pending.retain(|id| id != &target.id);
        }
    }
    Ok(())
}

fn finish_job(app: &Workbench, result: Result<(), String>, flag: &AtomicBool) {
    let source = app.lock().job.as_ref().map(|job| job.source);
    let batch = source.and_then(|source| create_batch(app, source).ok().flatten());
    let mut store = app.lock();
    store.archives = batch;
    let cancelled = flag.load(Ordering::Relaxed);
    for doc in &mut store.docs {
        if doc.status == "translating" {
            doc.status = if cancelled { "ready" } else { "failed" };
        }
    }
    if let Some(job) = &mut store.job {
        if cancelled {
            job.info.status = "cancelled";
            job.info.message = "Canceled".into();
        } else if let Err(error) = result {
            job.info.status = "failed";
            job.info.message = "Translation failed".into();
            job.info.error = Some(error);
        } else {
            job.info.percent = 100;
            if job.info.retained_paragraphs == 0 {
                job.info.status = "completed";
                job.info.message = "Completed".into();
            } else {
                job.info.status = "completed_with_warnings";
                let paragraphs = job.info.retained_paragraphs;
                let unit = if paragraphs == 1 { "paragraph" } else { "paragraphs" };
                job.info.message = format!("Exported with original text in {paragraphs} {unit}.");
            }
        }
    }
}

fn create_batch(app: &Workbench, source: Source) -> Result<Option<Batch>, String> {
    let mut entries = Vec::new();
    {
        let store = app.lock();
        for doc in &store.docs {
            if doc.source == source {
                if let Some(result) = &doc.result {
                    entries.push(ArchiveDocument {
                        name: doc.name.clone(),
                        files: [
                            result.mono.clone(),
                            result.dual.clone(),
                            result.glossary.clone(),
                        ],
                    });
                }
            }
        }
    }
    if entries.len() < 2 {
        return Ok(None);
    }
    let directory = tempfile::tempdir_in(app.scratch.path()).map_err(|error| error.to_string())?;
    let files = archives::write(&directory.path().join("exports"), entries)?;
    Ok(Some(Batch {
        files,
        _directory: directory,
    }))
}

async fn cancel_job(State(app): State<Arc<Workbench>>) -> StatusCode {
    let mut store = app.lock();
    if let Some(import) = &store.import {
        import.store(true, Ordering::Relaxed);
    }
    if let Some(job) = &mut store.job {
        if job.busy() {
            job.flag.store(true, Ordering::Relaxed);
            job.info.status = "cancelling";
            job.info.message = "Canceling…".into();
        }
    }
    StatusCode::ACCEPTED
}

#[derive(Deserialize)]
struct ProfileRequest {
    #[serde(default)]
    name: String,
    #[serde(default, alias = "base_url")]
    url: String,
    #[serde(default)]
    model: String,
    #[serde(default)]
    key: String,
}

async fn save_profile(
    State(app): State<Arc<Workbench>>,
    Json(profile): Json<ProfileRequest>,
) -> Result<Json<Value>, ApiError> {
    let name = profiles::save(&app.profiles, &profile.name, &profile.url, &profile.model)
        .map_err(ApiError::bad)?;
    Ok(Json(json!({"name":name})))
}

async fn delete_profile(
    State(app): State<Arc<Workbench>>,
    Route(name): Route<String>,
) -> Result<StatusCode, ApiError> {
    profiles::delete(&app.profiles, &name).map_err(ApiError::bad)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn test_profile(Json(profile): Json<ProfileRequest>) -> Result<Json<Value>, ApiError> {
    let message = network::test_provider(&profile.url, &profile.model, &profile.key)
        .await
        .map_err(ApiError::bad)?;
    Ok(Json(json!({"message":message})))
}

async fn select_service(
    State(app): State<Arc<Workbench>>,
    Json(mut profile): Json<ProfileRequest>,
) -> Result<Json<Value>, ApiError> {
    let service = if profile.name == "SiliconFlowFree" {
        Service::SiliconFlowFree
    } else {
        if profile.url.is_empty() && profile.model.is_empty() {
            if let Some(saved) = profiles::load(&app.profiles)
                .map_err(ApiError::bad)?
                .remove(&profile.name)
            {
                profile.url = saved.base_url;
                profile.model = saved.model;
            }
        }
        let name = if profile.name.trim().is_empty() {
            "OpenAI-compatible"
        } else {
            &profile.name
        };
        let (name, base_url, model) =
            profiles::validate(name, &profile.url, &profile.model).map_err(ApiError::bad)?;
        let key = profile.key.trim();
        if key.is_empty() {
            return Err(ApiError::bad("Enter an API key."));
        }
        Service::Custom {
            name,
            base_url,
            model,
            key: key.into(),
        }
    };
    let mut store = app.lock();
    ensure_idle(&store)?;
    let description = service.describe();
    store.service = service;
    Ok(Json(description))
}

#[derive(Serialize, Deserialize)]
struct Preferences {
    options: Options,
    #[serde(default)]
    service: Service,
}

fn previous_settings(path: &Path, profile_path: &Path) -> Option<Preferences> {
    let config: toml::Value = toml::from_str(&fs::read_to_string(path).ok()?).ok()?;
    let mut options = Options::default();
    if let Some(settings) = config.get("translation") {
        let string = |name: &str| settings.get(name).and_then(toml::Value::as_str);
        if let Some(value) = string("lang_in") {
            options.from = value.into();
        }
        if let Some(value) = string("lang_out") {
            options.to = if value == "zh" { "zh-CN" } else { value }.into();
        }
        if let Some(value) = string("custom_system_prompt").filter(|value| *value != "null") {
            options.prompt = value.into();
        }
    }
    if let Some(settings) = config.get("pdf") {
        let enabled = |name: &str| {
            settings
                .get(name)
                .and_then(toml::Value::as_bool)
                .unwrap_or(false)
        };
        if let Some(pages) = settings
            .get("pages")
            .and_then(toml::Value::as_str)
            .filter(|pages| *pages != "null")
        {
            options.pages = pages.into();
        }
        options.format = match (enabled("no_mono"), enabled("no_dual")) {
            (true, false) => "dual",
            (false, true) => "mono",
            _ => "both",
        }
        .into();
        options.translated_only = enabled("only_include_translated_page");
        options.alternate = enabled("use_alternating_pages_dual");
        options.translated_first = enabled("dual_translate_first");
    }
    let service = if config
        .get("openaicompatible")
        .and_then(toml::Value::as_bool)
        == Some(true)
    {
        config
            .get("openaicompatible_detail")
            .and_then(|detail| {
                let field = |name: &str| {
                    detail
                        .get(name)
                        .and_then(toml::Value::as_str)
                        .filter(|value| !value.is_empty() && *value != "null")
                };
                let (base_url, model) = profiles::validate_endpoint(
                    field("openai_compatible_base_url")?,
                    field("openai_compatible_model")?,
                )
                .ok()?;
                let key = field("openai_compatible_api_key")?.into();
                let name = profiles::load(profile_path)
                    .unwrap_or_default()
                    .into_iter()
                    .find(|(_, profile)| profile.base_url == base_url && profile.model == model)
                    .map(|(name, _)| name)
                    .unwrap_or_else(|| "OpenAI-compatible".into());
                Some(Service::Custom {
                    name,
                    base_url,
                    model,
                    key,
                })
            })
            .unwrap_or_default()
    } else {
        Service::SiliconFlowFree
    };
    Some(Preferences { options, service })
}

async fn save_settings(
    State(app): State<Arc<Workbench>>,
    Json(options): Json<Options>,
) -> Result<Json<Value>, ApiError> {
    let options = options.validate()?;
    let mut store = app.lock();
    ensure_idle(&store)?;
    let saved = Preferences {
        options: options.clone(),
        service: store.service.clone(),
    };
    let parent = app.settings.parent().ok_or_else(ApiError::disk)?;
    let mut pending = tempfile::NamedTempFile::new_in(parent).map_err(|_| ApiError::disk())?;
    serde_json::to_writer_pretty(&mut pending, &saved).map_err(|_| ApiError::disk())?;
    pending.as_file().sync_all().map_err(|_| ApiError::disk())?;
    pending
        .persist(&app.settings)
        .map_err(|_| ApiError::disk())?;
    store.options = options;
    Ok(Json(json!({"message":"Settings saved."})))
}

#[derive(Deserialize, Default)]
struct DownloadQuery {
    #[serde(default)]
    download: Option<u8>,
}

async fn pdf_file(
    State(app): State<Arc<Workbench>>,
    Route((id, kind)): Route<(String, String)>,
    Query(query): Query<DownloadQuery>,
    request: Request,
) -> Result<Response, ApiError> {
    let (path, name) = {
        let store = app.lock();
        let doc = store
            .docs
            .iter()
            .find(|doc| doc.id == id)
            .ok_or_else(ApiError::missing)?;
        let path = doc.file(&kind).ok_or_else(ApiError::missing)?.to_owned();
        let stem = doc.name.trim_end_matches(".pdf");
        let name = match kind.as_str() {
            "original" => doc.name.clone(),
            "glossary" => format!("{stem}.glossary.csv"),
            _ => format!("{stem}.{kind}.pdf"),
        };
        (path, name)
    };
    file_response(path, name, query.download == Some(1), request).await
}

async fn archive_file(
    State(app): State<Arc<Workbench>>,
    Route(kind): Route<String>,
    request: Request,
) -> Result<Response, ApiError> {
    let path = app
        .lock()
        .archives
        .as_ref()
        .and_then(|batch| batch.files.get(&kind))
        .cloned()
        .ok_or_else(ApiError::missing)?;
    let name = path
        .file_name()
        .ok_or_else(ApiError::missing)?
        .to_string_lossy()
        .into_owned();
    file_response(path, name, true, request).await
}

async fn file_response(
    path: PathBuf,
    name: String,
    download: bool,
    request: Request,
) -> Result<Response, ApiError> {
    let mut response = ServeFile::new(&path)
        .oneshot(request)
        .await
        .map_err(|_| ApiError::disk())?
        .into_response();
    if download && response.status().is_success() {
        let value = format!(
            "attachment; filename*=UTF-8''{}",
            utf8_percent_encode(&name, NON_ALPHANUMERIC)
        );
        if let Ok(value) = header::HeaderValue::from_str(&value) {
            response
                .headers_mut()
                .insert(header::CONTENT_DISPOSITION, value);
        }
    }
    Ok(response)
}
