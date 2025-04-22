use yew::prelude::*;
use crate::Colour;

#[derive(PartialEq, Properties)]
pub struct ThanksMsgProps {
    pub msg: UseStateHandle<Option<String>>,
    pub colour: UseStateHandle<Colour>,
}

#[function_component(ThanksMsg)]
pub fn thanks_msg(props: &ThanksMsgProps) -> Html {
    match &*props.msg {
        Some(msg) => html! {
            <div class={classes!("w-full", "max-w-lg", "text-center", "mb-4")} style={format!("color: {}", props.colour.to_string())}>
                { msg }
            </div>
        },
        None => html! {},
    }
}
