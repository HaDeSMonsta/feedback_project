use anyhow::{Context, Result};
use axum::http::{header, HeaderValue, Method, StatusCode};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use chrono::Utc;
use serde::Deserialize;
use std::cell::LazyCell;
use std::env;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Mutex;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::{debug, error, info, subscriber, Level};
use tracing_subscriber::FmtSubscriber;

static WRITE_MUTEX: Mutex<()> = Mutex::new(());
const FILE_PATH: &str = "/feedback/";
const FILE_NAME: &str = "feedback.txt";

const PORT: u16 = 8080; // This only runs in docker, so 8080 works
const LOG_LEVEL: LazyCell<Level> = LazyCell::new(|| {
    const ENV_KEY: &str = "LOG_LEVEL";
    #[cfg(debug_assertions)]
    const DEFAULT_LEVEL: Level = Level::DEBUG;
    #[cfg(not(debug_assertions))]
    const DEFAULT_LEVEL: Level = Level::INFO;

    match env::var(ENV_KEY) {
        Ok(lvl) => {
            lvl.parse()
               .unwrap_or_else(|_| {
                   println!("WARNING: {ENV_KEY} is set, but the value is invalid, \
                       using default ({DEFAULT_LEVEL})");
                   DEFAULT_LEVEL
               })
        }
        Err(_) => DEFAULT_LEVEL,
    }
});

#[derive(Debug, Deserialize)]
struct Feedback {
    feedback: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(*LOG_LEVEL)
            .finish()
    ).with_context(|| format!("Failed to set up logging with level {}", *LOG_LEVEL))?;

    let cors = CorsLayer::new()
        .allow_origin(
            env::var("ALLOW_ORIGIN")
                .context("ALLOW_ORIGIN env var is not set")?
                .parse::<HeaderValue>()
                .context("Failed to parse origin")?
        )
        .allow_methods([Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/feedback", post(handle_feedback))
        .layer(cors);

    let listener = TcpListener::bind(
        SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), PORT)
    )
        .await
        .with_context(|| format!("Failed to bind port {PORT}"))?;

    info!("Bound port {PORT}");

    axum::serve(listener, app)
        .await
        .with_context(|| format!("Failed to start server on port {PORT}"))?;

    Ok(())
}

async fn handle_feedback(Json(feedback): Json<Feedback>) -> impl IntoResponse {
    const LINE_SEP_LEN: usize = 50;
    let Ok(_lock) = WRITE_MUTEX.lock() else {
        error!("Failed to acquire write lock");
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to acquire write lock");
    };

    info!(?feedback);

    let now = Utc::now();
    let current_date = now.format("%Y-%m-%d");
    let current_date_time = Utc::now().format("[%Y-%m-%d - %H:%M:%S]z");
    let file_name = format!("{FILE_PATH}{current_date}-{FILE_NAME}");
    debug!(file_name);

    let Ok(file) = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&file_name) else {
        error!("Failed to open file {file_name} \
            (you probably didn't bind the correct port in docker)");
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to open file");
    };

    debug!("Opened file");

    let mut writer = BufWriter::new(file);

    debug!("Created writer");

    if let Err(e) = writeln!(writer, "{}", "-".repeat(LINE_SEP_LEN)) {
        error!("Failed to write initial lines to file {file_name}: {e}");
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to write to file");
    };
    if let Err(e) = writeln!(writer, "{current_date_time}") {
        error!("Failed to write time to file {file_name}: {e}");
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to write to file");
    };
    if let Err(e) = writeln!(writer, "{}", feedback.feedback) {
        error!("Failed to write feedback to file {file_name}: {e}");
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to write to file");
    };
    if let Err(e) = writeln!(writer, "{}\n", "-".repeat(LINE_SEP_LEN)) {
        error!("Failed to write ending lines to file {file_name}: {e}");
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to write to file");
    };

    debug!("Finished writing, flushing writer");

    if let Err(e) = writer.flush() {
        error!("Failed to flush file {file_name}: {e}");
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to flush file");
    };

    debug!("Exiting");

    (StatusCode::OK, "Feedback Received")
}
