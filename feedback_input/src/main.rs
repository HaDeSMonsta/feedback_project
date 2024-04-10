extern crate logger_utc as logger;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use std::env;
use std::fs::create_dir_all;

use dotenv::dotenv;
use logger::log_to_dyn_file;
use rocket::{launch, response::Redirect, routes};
use rocket::form::Form;
use rocket::response::content;

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

    println!("User Arguments:\nWebport {web_port}\n\
    IP-config file path: {target_address}\nTarget Port: {target_port}");

    rocket::build()
        .configure(rocket::Config {
            address: "0.0.0.0".parse().unwrap(),
            port: web_port,
            ..Default::default()
        })
        .mount("/", routes![feedback_landing, feedback_landing_msg, print_feedback])
}

#[get("/", format = "html", rank = 2)]
fn feedback_landing() -> content::RawHtml<String> {
    content::RawHtml(get_html_form(None, None, ""))
}

#[get("/?<status_msg>&<colour>&<initial_msg>", format = "html", rank = 1)]
fn feedback_landing_msg(status_msg: &str, colour: &str, initial_msg: &str)
    -> content::RawHtml<String> {
    content::RawHtml(get_html_form(Some(status_msg), Some(colour), initial_msg))
}

#[post("/", data = "<feedback>")]
async fn print_feedback(feedback: Form<Feedback>) -> Redirect {
    let (_, ip_path, target_port, auth) = get_vars();

    let (status_msg, colour, initial_msg) = match client::send_msg(
        &feedback.textbox.to_string(), &ip_path, target_port, &auth,
    ).await {
        Ok(_) => { ("Thank you", "green", "") }
        Err(err) => {
            log(err.to_string());
            ("An error occurred while sending the data to the Server", "red",
             feedback.textbox.as_str())
        }
    };

    let status_msg = urlencoding::encode(status_msg);
    let initial_msg = urlencoding::encode(initial_msg.trim());

    Redirect::to(
        format!("{}?status_msg={status_msg}&colour={colour}&initial_msg={initial_msg}",
                uri!(feedback_landing))
    )
}

pub fn get_html_form(msg: Option<&str>, color: Option<&str>, initial_msg: &str) -> String {
    let thanks_msg = match msg {
        Some(thanks_message) => {
            format!(r#"<p class="thanks-message">{thanks_message}</p>"#)
        }
        None => { String::new() }
    };

    let colour = match color {
        Some(c) => {
            String::from(c)
        }
        None => { String::new() }
    };

    format!(r#"
        <!DOCTYPE html>
        <html style="background-color: #212121; color: white;">
        <head>
            <title>Feedback Tutorium</title>
            <style>
                .thanks-message {{
                    color: {colour};
                }}
            </style>
        </head>
        <body>

        <h1>Feedback</h1>
        {thanks_msg}
        <p>Please enter the Feedback here:</p>

        <form action="{uri}" method="post">
            <textarea id="textbox" name="textbox" rows="8" cols="50">{initial_msg}</textarea>
            <input type="submit" value="Submit">
        </form>

        </body>
        </html>
    "#, uri = uri!(print_feedback))
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
