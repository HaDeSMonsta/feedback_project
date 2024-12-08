extern crate logger_utc as logger;
#[macro_use]
extern crate rocket;

use std::env;
use std::fs::create_dir_all;
use std::net::Ipv6Addr;
use std::sync::LazyLock;
use anyhow::{Context, Result};
use logger::log_to_dyn_file;
#[cfg(not(feature = "deutschland"))]
use rand::Rng;
use rocket::form::Form;
use rocket::response::content;
use rocket::{response::Redirect, routes};
use tonic::Status;
use crate::client::send_msg;

mod client;

const LOG_DIR: &'static str = "/logs/";
const LOG_FILE_NAME: &'static str = "err.log";

pub const PORT: LazyLock<u16> = LazyLock::new(|| {
    env::var("WEB_PORT")
        .expect("WEB_PORT must be set")
        .parse()
        .expect("WEB_PORT must be a valid u16")
});
pub const SERVER_ADDR: LazyLock<String> = LazyLock::new(|| {
    env::var("SERVER_ADDR")
        .expect("SERVER_ADDR must be set")
});
pub const AUTH: LazyLock<String> = LazyLock::new(|| {
    env::var("AUTH")
        .expect("AUTH must be set")
});

#[derive(FromForm)]
struct Feedback {
    textbox: String,
}

#[get("/?<status_msg>&<colour>&<initial_msg>", format = "html", rank = 1)]
fn feedback_landing(status_msg: Option<String>, colour: Option<String>, initial_msg: Option<String>)
    -> content::RawHtml<String> {
    content::RawHtml(get_html_form(status_msg, colour, initial_msg))
}

#[post("/", data = "<feedback>")]
async fn print_feedback(feedback: Form<Feedback>) -> Redirect {

    let (status_msg, colour, initial_msg) = match send_msg(
        &feedback.textbox.to_string()
    ).await {
        Ok(_) => (Some("Thank you"), None, None),
        Err(err) => {
            log(err.to_string());
            let err_msg = match err.downcast_ref::<Status>() {
                Some(status) if status.code() == tonic::Code::Internal
                => "Internal Server error, please contact the site administrator",
                _ => "An error occurred while sending data to the Server",
            };
            (Some(err_msg), Some("red"), Some(feedback.textbox.as_str()))
        }
    };

    Redirect::to(uri!(feedback_landing(status_msg, colour, initial_msg)))
}

fn get_html_form(thanks_msg: Option<String>, colour: Option<String>, initial_msg: Option<String>)
    -> String {
    let thanks_msg = thanks_msg.unwrap_or(String::new());
    let colour = colour.unwrap_or(String::from("green"));
    let initial_msg = initial_msg.unwrap_or(String::new());

    let uri = uri!(print_feedback).to_string();

    let replacements = [
        ("{thanks_msg}", thanks_msg),
        ("{colour}", colour),
        ("{initial_msg}", initial_msg),
        ("{uri}", uri),
    ];

    let res;

    #[cfg(not(feature = "deutschland"))]
    {
        let spinner = if rand::rng().random_range(0..1_000) == 0 {
            "animate-spin"
        } else { "" };

        let mut raw = String::from(include_str!("../html/index.html"));

        for (from, to) in replacements {
            raw = raw.replace(from, &to);
        }
        raw = raw.replace("{spin}", spinner);

        res = raw;
    }

    #[cfg(feature = "deutschland")]
    {
        let mut raw = String::from(include_str!("../html/DEUTSCHLAND.html"));

        for (from, to) in replacements {
            raw = raw.replace(from, &to);
        }

        res = raw
    }

    res
}

fn log(to_log: String) {
    log_to_dyn_file(to_log, Some(LOG_DIR), LOG_FILE_NAME).unwrap();
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv(); // If in docker, this must be allowed to fail
    create_dir_all(LOG_DIR).expect(&format!("Unable to create {LOG_DIR} dir"));

    let _ = AUTH.as_str();
    println!("Env Arguments (excluding auth, but auth is set):\n\
    Webport {}\n\
    Target Server Addr: {}", *PORT, *SERVER_ADDR);

    rocket::build()
        .configure(rocket::Config {
            address: Ipv6Addr::UNSPECIFIED.into(),
            port: *PORT,
            ..Default::default()
        })
        .mount("/", routes![feedback_landing, print_feedback])
        .launch()
        .await
        .with_context(|| "The server failed")?;

    Ok(())
}
