use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::footer::Footer;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    let location = use_location().unwrap();

    html! {
        <>
            <h1 class={classes!("text-3xl", "font-bold", "mb-4")}>{ "404" }</h1>
            <p>{ format!("Path {:?} not found", location.path()) }</p>
            <Footer/>
        </>
    }
}
