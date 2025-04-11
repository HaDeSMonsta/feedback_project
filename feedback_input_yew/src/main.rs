use yew::prelude::*;
use wasm_bindgen::prelude::*;

const POST_URI: &str = "";
const LORIS_LINK: &str = "https://www.youtube.com/channel/UCe40qwYch8JcmBST_BWaYNA";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "toggleTheme")]
    fn toggleTheme();
}

enum Colour {
    Green,
    Red,
}

impl Colour {
    fn to_string(&self) -> String {
        match self {
            Colour::Green => String::from("green"),
            Colour::Red => String::from("red"),
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    let thanks_message = use_state(|| None);
    // let thanks_message = use_state(|| Some(String::from("Test")));
    let thanks_colour = use_state(|| Colour::Green);

    html! {
        <>
            <h1 class={classes!("text-3xl", "font-bold", "mb-6", "test-class")}>{ "Feedback" }</h1>
            { slider() }
            { format_thanks(&thanks_message, &thanks_colour) }
            <p class={classes!("text-lg", "mb-4")}>{ "Please enter the Feedback here:" }</p>
            { input() }
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

fn input() -> Html {
    html! {
        <form action={POST_URI} method="post" class={classes!("w-full", "max-w-lg")}>
        <textarea id="textbox" name="textbox" rows="8" class={classes!("w-full", "p-2", "bg-gray-100", "dark:bg-gray-800", "border", "border-gray-300", "dark:border-gray-700", "rounded", "mb-4", "resize-none")}>
        </textarea>
        <button type="submit" class={classes!("w-full", "bg-indigo-700", "hover:bg-indigo-800", "text-white", "font-bold", "py-2", "px-4", "rounded")}>
        { "Submit" }
        </button>
        </form>
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
