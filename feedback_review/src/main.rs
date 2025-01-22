use anyhow::{Context, Result};
use regex::Regex;
use rocket::response::content::RawHtml;
use rocket::{get, routes};
use std::cell::LazyCell;
use std::net::Ipv6Addr;
use std::{env, fs};
use std::process::exit;
use std::time::Duration;
use tokio::time;
use tracing::{error, subscriber, warn, Level};
use tracing_subscriber::FmtSubscriber;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PORT: LazyCell<u16> = LazyCell::new(|| {
    const PORT_STR: &str = "PORT";
    env::var(&PORT_STR)
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect(&format!("{PORT_STR} is set, but not a u16"))
});

const LOG_LEVEL: LazyCell<Level> = LazyCell::new(|| {
    const LEVEL_STR: &str = "LOG_LEVEL";
    #[cfg(debug_assertions)]
    let default_log_level = Level::DEBUG;
    #[cfg(not(debug_assertions))]
    let default_log_level = Level::INFO;

    match env::var(&LEVEL_STR) {
        Ok(level) => level
            .parse()
            .expect(&format!("{LEVEL_STR} is set, but not a valid log level")),
        Err(_) => default_log_level,
    }
});

#[cfg(debug_assertions)]
const FEEDBACK_DIR: &str = "./feedback/";
#[cfg(not(debug_assertions))]
const FEEDBACK_DIR: &str = "/feedback/";
const FEEDBACK_FN_SUFFIX: &str = "-feedback.txt";
const FEEDBACK_DATE_REGEX: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new(r"^\d{4}-\d{2}-\d{2}").unwrap()
});
const FEEDBACK_TIME_REGEX: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new(r"^\[\d{4}-\d{2}-\d{2} - (\d{2}:\d{2}:\d{2})]z$").unwrap()
});

#[get("/")]
fn index() -> RawHtml<String> {
    const LI_TEMPLATE: &str = include_str!("../html/index_li_template.html");
    const RAW_HTML: &str = include_str!("../html/index.html");

    let dates = get_available_dates()
        .expect("Unable to get available dates")
        .into_iter()
        .map(|date| {
            LI_TEMPLATE.replace("DATE", &date)
        })
        .reduce(|acc, curr| {
            if acc.is_empty() {
                curr
            } else {
                format!("{acc}\n{curr}")
            }
        })
        .expect("No dates provided");

    RawHtml(RAW_HTML.replace("DATES", &dates))
}

#[get("/<date>")]
fn feedback(date: String) -> RawHtml<String> {
    const LI_TEMPLATE: &str = include_str!("../html/feedback_li_template.html");
    const RAW_HTML: &str = include_str!("../html/feedback.html");

    let feedbacks = get_feedback_for_date(&date)
        .expect(&format!("Unable to get feedback for date {date}"))
        .into_iter()
        .rev()
        .map(|feedback| {
            let mut li = LI_TEMPLATE.replace("TIME", &feedback[0]);
            li = li.replace("FEEDBACK", &feedback[1..].join("\n"));
            li
        })
        .reduce(|acc, curr| {
            if acc.is_empty() {
                curr
            } else {
                format!("{acc}\n{curr}")
            }
        })
        .unwrap_or(String::new());

    let html = RAW_HTML
        .replace("FEEDBACK", &feedbacks)
        .replace("DATE", &date);

    RawHtml(html)
}

#[get("/version")]
fn version() -> RawHtml<String> {
    RawHtml(include_str!("../html/version.html").replace("VERSION", VERSION))
}

fn get_available_dates() -> Result<Vec<String>> {
    let mut dates = vec![];

    for feedback_file in fs::read_dir(FEEDBACK_DIR)
        .with_context(|| format!("Unable to read feedback dir {FEEDBACK_DIR}"))? {
        let feedback_file_name = feedback_file
            .context("Unable to read feedback file")?
            .file_name();
        // Need to shadow, because else the OsString would be dropped
        let feedback_file_name = feedback_file_name
            .to_str()
            .context("Unable to convert feedback file name to &str")?;

        let Some(capture) = FEEDBACK_DATE_REGEX.captures(&feedback_file_name) else {
            error!("Unable to capture feedback file date {feedback_file_name}");
            continue;
        };

        let date = capture[0].to_string();
        dates.push(date);
    }

    dates.sort();
    dates.reverse();
    Ok(dates)
}

/// Returns an error on fs error and empty vec if no feedback exists
fn get_feedback_for_date(date: &str) -> Result<Vec<Vec<String>>> {
    let feedback_file_name = format!("{FEEDBACK_DIR}{date}{FEEDBACK_FN_SUFFIX}");

    if !fs::exists(&feedback_file_name)
        .with_context(|| format!("Unable to validate if {feedback_file_name} exists"))? {
        return Ok(vec![]);
    };

    let mut feedbacks = vec![];
    let mut curr_lines = vec![];
    let mut active = false;

    for line in fs::read_to_string(&feedback_file_name)
        .with_context(|| format!("Unable to read lines in existing file {feedback_file_name}"))?
        .lines() {
        if line == "-".repeat(50) {
            active = !active;
            if !active { // Just turned inactive
                feedbacks.push(curr_lines);
                curr_lines = vec![];
            } else { // Skip lines line
                continue;
            }
        }
        if !active { continue; }

        if let Some(capture) = FEEDBACK_TIME_REGEX.captures(&line) {
            let time = capture[1].to_string();
            curr_lines.push(time);
            continue;
        }
        curr_lines.push(line.to_string());
    }

    Ok(feedbacks)
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv();

    subscriber::set_global_default(FmtSubscriber::builder().with_max_level(*LOG_LEVEL).finish())
        .with_context(|| format!("Unable to set subscriber with log level {}", *LOG_LEVEL))?;

    if fs::metadata(FEEDBACK_DIR).is_err() {
        error!("Feedback dir {FEEDBACK_DIR} does not exist");
        // Give user time to see it in the logs
        for i in (1..=10).rev() {
            warn!("Shutting down in {i} second{}", if i == 1 { "" } else { "s" });
            time::sleep(Duration::from_secs(1)).await;
        }
        exit(1);
    }

    rocket::build()
        .configure(rocket::Config {
            address: Ipv6Addr::UNSPECIFIED.into(),
            port: *PORT,
            ..Default::default()
        })
        .mount("/", routes![index, version, feedback,])
        .launch()
        .await
        .context("The server failed")?;

    Ok(())
}
