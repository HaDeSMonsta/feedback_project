use gloo::net::http::Request;
use web_sys::HtmlTextAreaElement;
use yew::platform::spawn_local;
use yew::prelude::*;
use crate::{Colour, Feedback, POST_URI};

pub fn input(
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
            let feedback = feedback.clone();
            let thanks_msg = thanks_msg.clone();
            let thanks_colour = thanks_colour.clone();

            if feedback.is_empty() {
                thanks_colour.set(Colour::Red);
                thanks_msg.set(Some(String::from("Please enter feedback!")));
                return;
            }

            let feedback_data = Feedback { feedback: (*feedback).trim().to_string() };
            let parsed_feedback = serde_json::to_string(&feedback_data).unwrap();

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
                        feedback.set(String::new());
                    }
                    Ok(resp) => {
                        thanks_colour.set(Colour::Orange);
                        thanks_msg.set(Some(format!("Backend was unable to handle request: \
                            {resp:?}")))
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
                value={feedback.clone().to_string()}
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
