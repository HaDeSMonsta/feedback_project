use yew::prelude::*;
use crate::toggleTheme;

#[function_component(Slider)]
pub fn slider() -> Html {
    html! {
        <div class={classes!("absolute", "top-4", "right-4")}>
            <label class={classes!("switch")}>
                <input type="checkbox" id="theme-switch" onclick={Callback::from(|_| toggleTheme())} checked={true}/>
                <span class={classes!("slider")}></span>
            </label>
        </div>
    }
}
