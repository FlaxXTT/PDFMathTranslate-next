use std::env;
use std::fs;
use std::io;
use std::net::{Ipv4Addr, TcpListener};
use std::path::PathBuf;
use std::process::Command;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::Duration;

use tokio::sync::oneshot;

use crate::APP_NAME;
use crate::workbench::{self, Workbench};

pub(crate) struct RuntimePaths {
    web: PathBuf,
    assets: PathBuf,
    data: PathBuf,
    legacy_profiles: Option<PathBuf>,
    legacy_settings: Option<PathBuf>,
    pub(crate) log: PathBuf,
}

impl RuntimePaths {
    pub(crate) fn discover() -> io::Result<Self> {
        let executable = env::current_exe()?.canonicalize()?;
        let workspace = env::var_os("HERMENEUEIN_WORKSPACE")
            .map(PathBuf::from)
            .or_else(|| {
                executable
                    .ancestors()
                    .find(|path| path.join("hermeneuein-rust/Cargo.toml").is_file())
                    .map(PathBuf::from)
            });
        let resources = env::var_os("HERMENEUEIN_RESOURCES")
            .map(PathBuf::from)
            .or_else(|| {
                let binary = executable.parent()?;
                (binary.file_name()? == "MacOS").then(|| binary.parent().unwrap().join("Resources"))
            })
            .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")));
        let web = resources.join("web");
        if !web.join("index.html").is_file() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Could not find the {APP_NAME} resources."),
            ));
        }
        let home = env::var_os("HOME").map(PathBuf::from);
        let data_override = env::var_os("HERMENEUEIN_DATA").map(PathBuf::from);
        let data = data_override
            .clone()
            .or_else(|| {
                workspace
                    .as_ref()
                    .map(|workspace| workspace.join("hermeneuein-data"))
            })
            .or_else(|| {
                home.as_ref()
                    .map(|home| home.join("Library/Application Support/Hermeneuein"))
            })
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find the application data directory."))?;
        Ok(Self {
            web,
            assets: resources,
            log: data.join("workbench.log"),
            legacy_profiles: if data_override.is_none() {
                workspace.map(|path| path.join("pdfmathtranslate/service_profiles.json"))
            } else {
                None
            },
            legacy_settings: if data_override.is_none() {
                home.map(|path| path.join(".config/pdf2zh/config.v3.toml"))
            } else {
                None
            },
            data,
        })
    }
}

pub(crate) enum BackendStatus {
    Starting,
    Ready,
    Exited(&'static str),
}

pub(crate) struct Backend {
    app: Arc<Workbench>,
    runtime: Option<tokio::runtime::Runtime>,
    task: tokio::task::JoinHandle<()>,
    stop: Option<oneshot::Sender<()>>,
    ready: Arc<AtomicBool>,
    url: String,
}

impl Backend {
    pub(crate) fn start(paths: &RuntimePaths, browser: bool) -> io::Result<Self> {
        let port = match env::var("PDFMATH_APP_PORT") {
            Ok(port) => port
                .parse::<u16>()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "The port must be a valid integer."))?,
            Err(env::VarError::NotPresent) => 0,
            Err(error) => return Err(io::Error::new(io::ErrorKind::InvalidInput, error)),
        };
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, port))?;
        listener.set_nonblocking(true)?;
        let host = listener.local_addr()?.to_string();
        let url = format!("http://{host}");
        let app = Workbench::new(
            &paths.data,
            &paths.assets,
            paths.legacy_profiles.as_deref(),
            paths.legacy_settings.as_deref(),
        )?;
        let router = workbench::router(app.clone(), paths.web.clone(), host);
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .thread_name("hermeneuein")
            .enable_all()
            .build()?;
        let listener = {
            let _scope = runtime.enter();
            tokio::net::TcpListener::from_std(listener)?
        };
        fs::write(&paths.log, format!("Rust 工作台 {url}\n"))?;
        let (stop, shutting_down) = oneshot::channel();
        let ready = Arc::new(AtomicBool::new(false));
        let started = ready.clone();
        let log = paths.log.clone();
        let task = runtime.spawn(async move {
            started.store(true, Ordering::Release);
            if let Err(error) = axum::serve(listener, router)
                .with_graceful_shutdown(async {
                    let _ = shutting_down.await;
                })
                .await
            {
                let _ = fs::write(log, format!("The workbench server exited ({error}).\n"));
            }
        });
        let backend = Self {
            app,
            runtime: Some(runtime),
            task,
            stop: Some(stop),
            ready,
            url,
        };
        if browser && env::var_os("HERMENEUEIN_NO_OPEN").is_none() {
            let _ = Command::new("/usr/bin/open").arg(backend.url()).spawn();
        }
        Ok(backend)
    }

    pub(crate) fn url(&self) -> &str {
        &self.url
    }

    pub(crate) fn poll(&mut self) -> io::Result<BackendStatus> {
        let status = if self.task.is_finished() {
            BackendStatus::Exited("the workbench stopped")
        } else if self.ready.load(Ordering::Acquire) {
            BackendStatus::Ready
        } else {
            BackendStatus::Starting
        };
        Ok(status)
    }
}

impl Drop for Backend {
    fn drop(&mut self) {
        self.app.shutdown();
        if let Some(stop) = self.stop.take() {
            let _ = stop.send(());
        }
        if let Some(runtime) = self.runtime.take() {
            runtime.shutdown_timeout(Duration::from_secs(2));
        }
    }
}
