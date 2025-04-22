use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class={classes!("mt-8", "text-sm", "text-center", "text-gray-500", "dark:text-gray-400")}>
            { "Made with ❤ using Yew and Axum" }
        </footer>
    }
}
