use yew::prelude::*;
use yew_router::prelude::*;
use serde::{Deserialize, Serialize};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Deserialize)]
struct FeedbackResponse {
    feedbacks: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
struct FeedbackRequest {
    date: Option<String>,
}

#[derive(Properties, PartialEq)]
struct DateProps {
    date: String,
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/version")]
    Version,
    #[at("/:date")]
    Date { date: String },
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::Date { date } => html! { <Date date={date}/>},
        Route::Version => html! { <Version/>},
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[function_component(Home)]
fn home() -> Html {
    html! {
        <>
            <p>{ "Moin" }</p>
            <Footer/>
        </>
    }
}

#[function_component(Date)]
fn date(props: &DateProps) -> Html {

    let feedbacks = use_state(|| Vec::<String>::new());

    html! {
        <>
            <p>{ props.date.clone() }</p>
            <Footer/>
        </>
    }
}

#[function_component(Version)]
fn version() -> Html {
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

#[function_component(Footer)]
fn footer() -> Html {
    html! {
        <footer class={classes!("mt-8", "text-sm", "text-center", "text-gray-500", "dark:text-gray-400")}>
            { "Made with ❤ using Yew and Axum" }
        </footer>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
