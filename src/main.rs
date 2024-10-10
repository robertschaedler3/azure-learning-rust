#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use anyhow::{Context, Result};
use log::LevelFilter;
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        rolling_file::{
            policy::compound::{
                roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
            },
            RollingFileAppender,
        },
    },
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
};
use tokio::{
    net::UnixListener,
    signal::unix::{signal, SignalKind},
};
use tokio_stream::wrappers::UnixListenerStream;
use warp::Filter;

use platform::{handlers, routes};

#[tokio::main]
async fn main() -> Result<()> {
    init_logger();

    let path = std::path::Path::new("/tmp/osc-platform.sock");

    if path.exists() {
        std::fs::remove_file(path)?;
    } else {
        let parent = path.parent().unwrap();
        std::fs::create_dir_all(parent)?;
    }

    let listener = UnixListener::bind(path).context("Unable to bind to unix socket")?;
    let incoming = UnixListenerStream::new(listener);

    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigquit = signal(SignalKind::quit())?;
    let mut sigterm = signal(SignalKind::terminate())?;
    let mut sighup = signal(SignalKind::hangup())?;

    let platform = platform::init()?;
    {
        let platform = platform.clone();

        tokio::spawn(async move {
            while sighup.recv().await.is_some() {
                log::debug!("Received SIGHUP, reloading platform");

                let mut platform = platform.lock().unwrap();

                if let Err(e) = platform.reload() {
                    log::error!("Failed to reload platform: {}", e);
                }
            }
        });
    }

    let routes = routes::api(platform)
        .with(warp::log("platform"))
        .recover(handlers::handle_rejection);

    log::info!("Starting platform...");

    let server = warp::serve(routes).serve_incoming_with_graceful_shutdown(incoming, async move {
        tokio::select! {
            _ = sigint.recv() => {
                log::debug!("Received SIGINT, shutting down");
            }
            _ = sigquit.recv() => {
                log::debug!("Received SIGQUIT, shutting down");
            }
            _ = sigterm.recv() => {
                log::debug!("Received SIGTERM, shutting down");
            }
        }
    });

    log::info!("Listening on: {}", path.display());
    server.await;

    Ok(())
}

pub fn init_logger() {
    let _file_path = "/var/log/osconfig-platform.log";

    // Get the RUST_LOG environment variable and set the level filter accordingly.
    // If it is not set, default to "info"
    let rust_log = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "debug".to_string())
        .to_lowercase();
    let _level = match rust_log.as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        "off" => LevelFilter::Off,
        _ => LevelFilter::Info,
    };

    // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
    let _encoder = Box::new(PatternEncoder::new(
        "[{date(%Y-%m-%d %H:%M:%S)}] [{module}] [{highlight({level})}] {message}\n",
    ));
}
