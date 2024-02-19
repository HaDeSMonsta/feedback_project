extern crate logger_utc as logger;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use std::borrow::Cow;
use std::env;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;

use chrono::Local;
use dotenv::dotenv;
use logger::log_file;
use rocket::{launch, response::Redirect, routes};
use rocket::form::Form;
use rocket::response::content;

mod client;

#[derive(FromForm)]
struct Feedback {
    textbox: String,
}

#[launch]
fn rocket() -> _ {
    dotenv().expect("Failed to read .env file");

    let (web_port, target_address, target_port) = get_vars();

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
fn print_feedback(feedback: Form<Feedback>) -> Redirect {
    let (_, ip_path, target_port) = get_vars();

    let (status_msg, colour, initial_msg) = match client::send_msg(
        feedback.textbox.to_string(), ip_path.as_str(), target_port,
    ) {
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
            Cow::Owned(format!(r#"<p class="thanks-message">{thanks_message}</p>"#))
        }
        None => { Cow::Borrowed("") }
    };

    let colour = match color {
        Some(c) => {
            Cow::Borrowed(c)
        }
        None => { Cow::Borrowed("white") }
    };

    format!(r#"
        <!DOCTYPE html>
        <html style="background-color: #212121; color: white;">
        <head>
            <title>Feedback Prog1</title>
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

fn get_vars() -> (u16, String, u16) {
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

    (web_port, target_address, target_port)
}

fn log(to_log: String) {
    let now = Local::now();
    let date = now.format("%Y-%d-%d");
    let dir = "logs";
    let file_name = format!("{dir}/{date}-err.log");

    create_dir_all(dir).expect(&format!("Unable to create {dir} dir"));

    log_file(to_log.as_str(), file_name.as_str()).expect("Unable to open log file");
}