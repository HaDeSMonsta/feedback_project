use crate::BACKEND_URL;
use gloo::net::http::Request;
use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FeedbackResponse {
    feedback: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FeedbackDates {
    dates: Option<Vec<String>>,
}
pub async fn get_all_dates() -> Result<Vec<String>, String> {
    let target_url = format!("{BACKEND_URL}/dates");
    let res = Request::get(&target_url)
        .send()
        .await
        .map_err(|e| format!("Failed to send request to {target_url}: {e}"))?;

    let dates = res
        .json::<FeedbackDates>()
        .await
        .map_err(|e| format!("Unable to parse response as JSON: {e}"))?;

    dates.dates.ok_or_else(|| "No dates found".to_string())
}

pub async fn get_feedback_for_date(date: &str) -> Result<String, String> {
    let target_url = format!("{BACKEND_URL}/feedback/{date}");

    let res = Request::get(&target_url)
        .send()
        .await
        .map_err(|e| format!("Failed to send request to {target_url}: {e}"))?;

    let res = res
        .json::<FeedbackResponse>()
        .await
        .map_err(|e| format!("Unable to parse response {res:?} as JSON: {e}"))?;

    res.feedback.ok_or_else(|| format!("No feedback found for date {date}"))
}

pub async fn parse_feedback(feedback: &str) -> Result<Vec<Vec<String>>, String> {
    const DASH_CNT: usize = 50;
    let feedback_time_regex = Regex::new(r"^\[\d{4}-\d{2}-\d{2} - (\d{2}:\d{2}:\d{2})]z$").unwrap();

    let mut feedbacks = vec![];
    let mut curr_lines = vec![];
    let mut active = false;

    for line in feedback.lines() {
        if line == "-".repeat(DASH_CNT) {
            active = !active;
            if !active { // => Just turned inactive
                feedbacks.push(curr_lines);
                curr_lines = vec![];
            } else {
                continue;
            }
        }

        if !active { continue; }

        if let Some(capture) = feedback_time_regex.captures(line) {
            curr_lines.push(capture[1].to_string());
        } else {
            curr_lines.push(line.to_string());
        }
    }

    Ok(feedbacks)
}

