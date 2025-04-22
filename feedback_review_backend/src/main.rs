use std::cell::LazyCell;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use anyhow::{Context, Result};
use axum::http::{header, HeaderValue, Method, StatusCode};
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::extract::Path;
use axum::routing::get;
use serde::Serialize;
use tokio::fs;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::{debug, error, info, subscriber, Level};
use tracing_subscriber::FmtSubscriber;

const FILE_ROOT: &str = "/feedback/";
const FILE_SUFFIX: &str = "-feedback.txt";
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

#[derive(Debug, Serialize)]
struct FeedbackDates {
    dates: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
struct FeedbackResponse {
    feedback: Option<String>,
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
        .route("/dates", get(get_available_feedbacks))
        .route("/feedback/{date}", get(get_feedback_for_date))
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

async fn get_available_feedbacks() -> impl IntoResponse {
    debug!("Getting available feedbacks");

    let mut dates = vec![];

    match fs::read_dir(FILE_ROOT).await {
        Ok(mut dir) => {
            while let Ok(Some(dir)) = dir.next_entry().await {
                if let Ok(file_name) = dir.file_name().into_string() {
                    dates.push(file_name);
                } else {
                    error!("Failed to convert file name to string");
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(FeedbackDates { dates: None }));
                }
            }
        }
        Err(e) => {
            error!("Failed to read directory {FILE_ROOT}: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(FeedbackDates { dates: None }));
        }
    }

    let dates = dates.into_iter()
                     .map(|date| date.replace(FILE_SUFFIX, ""))
                     .collect();

    debug!(?dates);

    (StatusCode::OK, Json(FeedbackDates { dates: Some(dates) }))
}

async fn get_feedback_for_date(Path(date): Path<String>) -> impl IntoResponse {
    debug!(date);
    let f_name = format!("{FILE_ROOT}{date}{FILE_SUFFIX}");
    debug!("Checking for file: {f_name}");
    let Ok(feedback) = fs::read_to_string(&f_name)
        .await else {
        error!("No feedback for date {date}");
        return (StatusCode::NOT_FOUND, Json(FeedbackResponse { feedback: None }));
    };

    (StatusCode::OK, Json(FeedbackResponse { feedback: Some(feedback) }))
}
