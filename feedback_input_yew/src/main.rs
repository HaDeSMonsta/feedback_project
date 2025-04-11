use gloo::net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::HtmlTextAreaElement;
use yew::platform::spawn_local;
use yew::prelude::*;

const POST_URI: &str = "http://127.0.0.1:8088/";
const LORIS_LINK: &str = "https://www.youtube.com/channel/UCe40qwYch8JcmBST_BWaYNA";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "toggleTheme")]
    fn toggleTheme();
}

#[derive(Serialize, Deserialize)]
struct Feedback {
    feedback: String,
}

enum Colour {
    Green,
    Red,
    Orange,
}

impl Colour {
    fn to_string(&self) -> String {
        match self {
            Colour::Green => String::from("green"),
            Colour::Red => String::from("red"),
            Colour::Orange => String::from("orange"),
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    let thanks_message = use_state(|| None);
    let thanks_colour = use_state(|| Colour::Green);
    let feedback = use_state(|| String::new());

    html! {
        <>
            <h1 class={classes!("text-3xl", "font-bold", "mb-6", "test-class")}>{ "Feedback" }</h1>
            { slider() }
            { format_thanks(&thanks_message, &thanks_colour) }
            <p class={classes!("text-lg", "mb-4")}>{ "Please enter the Feedback here:" }</p>
            { input(&feedback, &thanks_message, &thanks_colour) }
            { loris_footer() }
        </>
    }
}

fn slider() -> Html {
    html! {
        <div class={classes!("absolute", "top-4", "right-4")}>
            <label class={classes!("switch")}>
                <input type="checkbox" id="theme-switch" onclick={Callback::from(|_| toggleTheme())} checked={true}/>
                <span class={classes!("slider")}></span>
            </label>
        </div>
    }
}

fn format_thanks(
    thanks_message: &UseStateHandle<Option<String>>,
    thanks_colour: &UseStateHandle<Colour>,
) -> Html {
    match &**thanks_message {
        Some(msg) => html! {
            <div class={classes!("w-full", "max-w-lg", "text-center", "mb-4")} style={format!("color: {}", thanks_colour.to_string())}>
                { msg }
            </div>
        },
        None => html! {},
    }
}

fn input(
    feedback: &UseStateHandle<String>,
    thanks_msg: &UseStateHandle<Option<String>>,
    thanks_colour: &UseStateHandle<Colour>,
) -> Html {
    let on_feedback_input = {
        let feedback = feedback.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlTextAreaElement>() {
                feedback.set(input.value());
            }
        })
    };

    let on_click = {
        let feedback = feedback.clone();
        let thanks_msg = thanks_msg.clone();
        let thanks_colour = thanks_colour.clone();

        Callback::from(move |_| {
            let thanks_msg = thanks_msg.clone();
            let thanks_colour = thanks_colour.clone();

            let feedback = Feedback { feedback: (*feedback).trim().to_string() };
            let parsed_feedback = serde_json::to_string(&feedback).unwrap();

            spawn_local(async move {
                let response = Request::post(&POST_URI)
                    .header("Content-Type", "application/json")
                    .body(&parsed_feedback)
                    .expect("Failed to create request")
                    .send()
                    .await;

                match response {
                    Ok(resp) if resp.ok() => {
                        thanks_colour.set(Colour::Green);
                        thanks_msg.set(Some(String::from("Thank you for your feedback!")));
                    }
                    Ok(resp) => {
                        thanks_colour.set(Colour::Orange);
                        thanks_msg.set(Some(format!("Request was not successful: {resp:?}")))
                    }
                    Err(e) => {
                        thanks_colour.set(Colour::Red);
                        thanks_msg.set(Some(
                            format!("Unable to send request: {e}.")
                        ));
                    }
                }
            });
        })
    };

    html! {
        <div class={classes!("w-full", "max-w-lg")}>
            <textarea
                id="textbox"
                name="textbox"
                rows="8"
                class={classes!("w-full", "p-2", "bg-gray-100", "dark:bg-gray-800", "border", "border-gray-300", "dark:border-gray-700", "rounded", "mb-4", "resize-none")}
                oninput={on_feedback_input}
            ></textarea>
            <button
                type="submit"
                onclick={on_click}
                class={classes!("w-full", "bg-indigo-700", "hover:bg-indigo-800", "text-white", "font-bold", "py-2", "px-4", "rounded")}
            >
                { "Submit" }
            </button>
        </div>
    }
}

fn loris_footer() -> Html {
    html! {
        <p class={classes!("text-sm", "italic", "mt-8")}>
            { "Thank you, " }
            <a href={LORIS_LINK} class={classes!("text-blue-500", "underline")} target="_blank">{ "Loris," }</a>
            { " for the front end" }
        </p>
    }
}
