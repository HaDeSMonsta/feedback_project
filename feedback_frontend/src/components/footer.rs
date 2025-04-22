use yew::prelude::*;
use crate::LORIS_LINK;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <p class={classes!("text-sm", "italic", "mt-8")}>
            { "Thank you, " }
            <a href={LORIS_LINK} class={classes!("text-blue-500", "underline")} target="_blank">{ "Loris," }</a>
            { " for the design" }
        </p>
    }
}
