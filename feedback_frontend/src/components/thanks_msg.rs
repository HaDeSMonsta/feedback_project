use yew::prelude::*;
use crate::Colour;

#[derive(PartialEq, Properties)]
pub struct ThanksMsgProps {
    pub thanks_msg: UseStateHandle<Option<String>>,
    pub thanks_colour: UseStateHandle<Colour>,
}

#[function_component(ThanksMsg)]
pub fn thanks_msg(props: &ThanksMsgProps) -> Html {
    match &*props.thanks_msg {
        Some(msg) => html! {
            <div class={classes!("w-full", "max-w-lg", "text-center", "mb-4")} style={format!("color: {}", props.thanks_colour.to_string())}>
                { msg }
            </div>
        },
        None => html! {},
    }
}
