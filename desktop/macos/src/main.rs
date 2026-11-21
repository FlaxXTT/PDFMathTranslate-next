mod archives;
mod desktop;
mod font_subset;
mod layout_engine;
mod network;
mod pdf_engine;
mod profiles;
mod runtime;
mod translate;
mod workbench;

use std::error::Error;
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use runtime::{Backend, BackendStatus, RuntimePaths};

pub(crate) const APP_NAME: &str = "PDF Translator";

static INTERRUPTED: AtomicBool = AtomicBool::new(false);

extern "C" fn interrupt(_: libc::c_int) {
    INTERRUPTED.store(true, Ordering::Relaxed);
}

fn run_browser(paths: &RuntimePaths) -> Result<(), Box<dyn Error>> {
    // Ctrl-C reaches this process and drops the HTTP server and active jobs.
    unsafe {
        let handler = interrupt as *const () as libc::sighandler_t;
        libc::signal(libc::SIGINT, handler);
        libc::signal(libc::SIGTERM, handler);
    }
    let mut backend = Backend::start(paths, true)?;
    let mut started = false;
    while !INTERRUPTED.load(Ordering::Relaxed) {
        match backend.poll()? {
            BackendStatus::Ready if !started => {
                println!("{APP_NAME} · {}", backend.url());
                started = true;
            }
            BackendStatus::Exited(status) => {
                return Err(
                    format!("The local service exited ({status}). See the log at {}.", paths.log.display()).into(),
                );
            }
            _ => {}
        }
        thread::sleep(Duration::from_millis(160));
    }
    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args_os().skip(1);
    match args.next() {
        None => desktop::run(&RuntimePaths::discover()?),
        Some(mode) if mode == "--browser" => run_browser(&RuntimePaths::discover()?),
        Some(_) => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Launch {APP_NAME} with --browser or without arguments."),
        )
        .into()),
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
