#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use std::borrow::Cow;
use std::env;

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
    let (port, _, _) = get_args();

    rocket::build()
        .configure(rocket::Config {
            address: "0.0.0.0".parse().unwrap(),
            port,
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
    let (_, ip_path, target_port) = get_args();

    let (status_msg, colour, initial_msg) = match client::send_msg(
        feedback.textbox.to_string(), ip_path.as_str(), target_port,
    ) {
        Ok(_) => { ("Thank you", "green", "") }
        Err(_) => {
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

fn get_args() -> (u16, String, u16) {
    let args: Vec<String> = env::args().collect();
    let web_port = if args.len() > 1 { Some(args[1].as_str()) } else { None };
    let ip_path = if args.len() > 2 { Some(args[2].as_str()) } else { None };
    let target_port = if args.len() > 3 { Some(args[3].as_str()) } else { None };

    let web_port: u16 = web_port
        .expect("First argument (Web port) is missing")
        .parse()
        .expect("First argument (Web port) was not a u32");

    let ip_path = ip_path
        .expect("Second argument (IP-config path) not set")
        .to_string();

    let target_port: u16 = target_port
        .expect("Third argument (Target port) not set")
        .parse()
        .expect("Third argument (Target port) was not a valid int");
    (web_port, ip_path, target_port)
}