use crate::components::footer::Footer;
use crate::VERSION;
use yew::prelude::*;

#[function_component(Version)]
pub fn version() -> Html {
    gloo::utils::document().set_title("Feedback Review - Version");

    html! {
        <>
            <h1 class={classes!("text-3xl", "font-bold", "mb-4")}>{ "Application Version" }</h1>
            <p class={classes!("text-lg")}>
                { "The current version of this application is: " }
                <span id="app-version" class={classes!("font-mono")}>
                    { VERSION }
                </span>
            </p>
            <Footer/>
        </>
    }
}
