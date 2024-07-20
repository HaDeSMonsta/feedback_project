extern crate logger_utc as logger;
#[macro_use]
extern crate rocket;

use std::env;
use std::fs::create_dir_all;

use dotenv::dotenv;
use logger::log_to_dyn_file;
#[cfg(not(feature = "deutschland"))]
use rand::Rng;
use rocket::{launch, response::Redirect, routes};
use rocket::form::Form;
use rocket::response::content;
use tonic::Status;

mod client;

const LOG_DIR: &'static str = "logs/";
const LOG_FILE_NAME: &'static str = "err.log";

#[derive(FromForm)]
struct Feedback {
    textbox: String,
}

#[launch]
fn rocket() -> _ {
    dotenv().expect("Failed to read .env file");
    create_dir_all(LOG_DIR).expect(&format!("Unable to create {LOG_DIR} dir"));

    let (web_port, target_address, target_port, _) = get_vars();

    println!("User Arguments:\n\
    Webport {web_port}\n\
    IP-config file path: {target_address}\n\
    Target Port: {target_port}");

    rocket::build()
        .configure(rocket::Config {
            address: "::".parse().unwrap(),
            port: web_port,
            ..Default::default()
        })
        .mount("/", routes![feedback_landing, print_feedback])
}

#[get("/?<status_msg>&<colour>&<initial_msg>", format = "html", rank = 1)]
fn feedback_landing(status_msg: Option<String>, colour: Option<String>, initial_msg: Option<String>)
    -> content::RawHtml<String> {
    content::RawHtml(get_html_form(status_msg, colour, initial_msg))
}

#[post("/", data = "<feedback>")]
async fn print_feedback(feedback: Form<Feedback>) -> Redirect {
    let (_, ip_path, target_port, auth) = get_vars();

    let (status_msg, colour, initial_msg) = match client::send_msg(
        &feedback.textbox.to_string(), &ip_path, target_port, &auth,
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

pub fn get_html_form(msg: Option<String>, colour: Option<String>, initial_msg: Option<String>) 
    -> String {
    let thanks_msg = msg.unwrap_or(
        String::from("")
    );
    let colour = colour.unwrap_or(
        String::from("green")
    );
    let initial_msg = initial_msg.unwrap_or(
        String::from("")
    );
    let uri = uri!(print_feedback).to_string();

    #[allow(unused_mut)]
    let mut replacements = vec![
        ("{colour}", colour),
        ("{thanks_msg}", thanks_msg),
        ("{uri}", uri),
        ("{initial_msg}", initial_msg),
    ];

    let res;

    #[cfg(not(feature = "deutschland"))]
    {
        let spinner = if rand::thread_rng().gen_range(0..1_000) == 0 {
            "animate-spin"
        } else { "" };

        replacements.extend([("", "")]);

        let mut raw = String::from(include_str!("../html/index.html"));

        for (from, to) in replacements {
            raw = raw.replace(from, &to);
        }

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

fn get_vars() -> (u16, String, u16, String) {
    let web_port: u16 = env::var("WEB_PORT")
        .expect("WEB_PORT must be set")
        .parse()
        .expect("WEB_PORT must be a valid u16");
    let target_address = env::var("IP_PATH")
        .expect("IP_PATH must be set");
    let target_port: u16 = env::var("TARGET_PORT")
        .expect("TARGET_PORT must be set")
        .parse()
        .expect("TARGET_PORT must be a valid u16");
    let auth = env::var("AUTH")
        .expect("AUTH must be set");

    (web_port, target_address, target_port, auth)
}

fn log(to_log: String) {
    log_to_dyn_file(&to_log, Some(LOG_DIR), LOG_FILE_NAME).unwrap();
}
