use gloo::net::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;
use serde::{Deserialize, Serialize};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const BACKEND_URL: &str = include_str!("../backend_url.txt");

#[derive(Debug, Deserialize)]
struct FeedbackResponse {
    feedbacks: Option<String>,
}

#[derive(Debug, Serialize)]
struct FeedbackRequest {
    date: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FeedbackDates {
    dates: Option<Vec<String>>,
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
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::Date { date } => html! { <Date date={date}/>},
        Route::Version => html! { <Version/>},
        Route::NotFound => html! { <NotFound/> },
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
    let dates = use_state(|| None);

    {
        let dates = dates.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let result = get_all_dates().await;
                dates.set(Some(result));
            });
            || ()
        });
    }

    html! {
        <>
            <h1 class={classes!("text-3xl", "font-bold", "mb-6")}>{ "Available Feedback Dates" }</h1>
            {
                match &*dates {
                    None => html! { <p>{ "Loading..." }</p> },
                    Some(Ok(dates)) => html! {
                        <>
                            <ul class={classes!("space-y-4", "w-full", "max-w-3xl")}>
                                {
                                    for dates
                                        .iter()
                                        .map(|date| html! {
                                            <li>
                                                <Link<Route> to={Route::Date { date: date.clone() }}>
                                                    <a class={classes!("block", "w-full", "bg-gray-200", "hover:bg-gray-300", "text-gray-800", "dark:bg-gray-700", "dark:hover:bg-gray-600", "dark:text-gray-300", "font-bold", "py-3", "px-6", "rounded", "shadow", "text-center", "transition")}>
                                                        { date }
                                                    </a>
                                                </Link<Route>>
                                            </li>
                                        })
                                }
                            </ul>
                        </>
                    },
                    Some(Err(err)) => html! { <p class="error">{ format!("Error: {}", err) }</p> },
                }
            }
            <Footer/>
        </>
    }
}

#[function_component(Date)]
fn date(props: &DateProps) -> Html {
    html! {
        <>
            <p>{ props.date.clone() }</p>
            <Footer/>
        </>
    }
}

async fn get_all_dates() -> Result<Vec<String>, String> {
    let target_url = format!("{BACKEND_URL}/dates");
    let res = Request::get(&target_url)
        .send()
        .await
        .map_err(|e| format!("Failed to send request to {target_url}: {e}"))?;

    let dates = res
        .json::<FeedbackDates>()
        .await
        .map_err(|e| format!("Unable to parse response as JSON: {e}"))?;

    dates.dates.ok_or_else(|| "No dates found".to_string())
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

#[function_component(NotFound)]
fn not_found() -> Html {
    let location = use_location().unwrap();

    html! {
        <>
            <h1 class={classes!("text-3xl", "font-bold", "mb-4")}>{ "404" }</h1>
            <p>{ format!("Path {:?} not found", location.path()) }</p>
            <Footer/>
        </>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
