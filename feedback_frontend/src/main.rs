mod components;

use std::fmt::Display;

use crate::components::cookie_banner::CookieBanner;
use crate::components::footer::Footer;
use crate::components::slider::Slider;
use crate::components::thanks_msg::ThanksMsg;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::components::input::Input;

const POST_URI: &str = include_str!("../target_uri.txt");
const LORIS_LINK: &str = "https://www.youtube.com/channel/UCe40qwYch8JcmBST_BWaYNA";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "toggleTheme")]
    fn toggleTheme();
}

#[derive(Serialize)]
struct Feedback {
    feedback: String,
}

#[derive(PartialEq, Clone)]
enum Colour {
    Green,
    Red,
    Orange,
}

impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Colour::Green => write!(f, "green"),
            Colour::Orange => write!(f, "orange"),
            Colour::Red => write!(f, "red"),
        }
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}

#[function_component(Main)]
fn app() -> Html {
    let thanks_message = use_state(|| None);
    let thanks_colour = use_state(|| Colour::Green);
    let feedback = use_state(String::new);

    html! {
        <>
            <h1 class={classes!("text-3xl", "font-bold", "mb-6")}>{ "Feedback" }</h1>
            <Slider/>
            <ThanksMsg thanks_msg={thanks_message.clone()} thanks_colour={thanks_colour.clone()}/>
            <p class={classes!("text-lg", "mb-4")}>{ "Please enter the Feedback here:" }</p>
            <Input feedback={feedback.clone()} thanks_msg={thanks_message.clone()} thanks_colour={thanks_colour.clone()}/>
            <Footer/>
            <CookieBanner/>
        </>
    }
}

