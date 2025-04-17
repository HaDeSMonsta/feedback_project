use gloo::net::http::Request;
use regex::Regex;
use yew::prelude::*;
use yew_router::prelude::*;
use serde::Deserialize;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const BACKEND_URL: &str = include_str!("../backend_url.txt");

#[derive(Debug, Deserialize)]
struct FeedbackResponse {
    feedback: Option<String>,
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
    let title = format!("Feedback {}", props.date);
    gloo::utils::document().set_title(&title);

    let date = props.date.clone();
    let feedback = use_state(|| Err("Loading..".to_string()));

    {
        let feedback = feedback.clone();
        let date = date.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let dates = match get_all_dates().await {
                    Ok(dates) => dates,
                    Err(err) => {
                        feedback.set(Err(format!("Unable to get all dates: {err}")));
                        return;
                    }
                };

                if !dates.contains(&date) {
                    feedback.set(Err(format!("Date {date} not found")));
                    return;
                }

                let raw_feedbacks = match get_feedback_for_date(&date).await {
                    Ok(f) => f,
                    Err(e) => {
                        feedback.set(Err(format!("Unable to get feedback for date {date}: {e}")));
                        return;
                    }
                };

                match parse_feedback(&raw_feedbacks).await {
                    Ok(fs) => feedback.set(Ok(fs)),
                    Err(e) => feedback.set(Err(format!("Unable to parse feedback: {e}"))),
                }
            });
            || ()
        });
    }

    html! {
        <>
            {
                match &*feedback {
                    Ok(feedbacks) => html! {
                        <>
                            <Link<Route> to={Route::Home}>
                                <a class={classes!("absolute", "top-4", "left-4", "text-blue-500", "dark:text-blue-400", "hover:underline", "flex", "items-center")}>
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class={classes!("w-6", "h-6", "mr-1")}>
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 19.5L8.25 12l7.5-7.5" />
                                    </svg>
                                    { "Back to Home" }
                                </a>
                            </Link<Route>>
                            <h1 class={classes!("text-3xl", "font-bold", "mb-6")}>
                                { format!("Feedback for {date}") }
                            </h1>
                            <ul class={classes!("text-lg", "space-y-4", "w-full", "max-w-4xl")}>
                                {
                                    for feedbacks
                                        .iter()
                                        .map(|feedback| html! {
                                            <li class={classes!("flex", "items-center", "p4", "border", "border-gray-200", "rounded-lg", "dark:border-gray-600", "dark:bg-gray-700")}>
                                                <div class={classes!("flex-1", "feedback-container")}>
                                                    { 
                                                        feedback[1..]
                                                            .iter()
                                                            .map(|line| html! { <p>{ line }</p> })
                                                            .collect::<Html>() 
                                                    }
                                                </div>
                                                <div class={classes!("text-sm", "text-gray-500", "dark:text-gray-400", "ml-4")}>
                                                    { &feedback[0] }
                                                </div>
                                            </li>
                                        })
                                }
                            </ul>
                        </>
                },
                    Err(e) => html! {
                        <h1 class={classes!("text-3xl", "font-bold", "mb-4")}>
                            { e }
                        </h1> },
                }
            }
            <Link<Route> to={Route::Home}>
                 <a class={classes!("mt-6", "inline-block", "text-blue-500", "dark:text-blue-400", "hover:underline")}>
                    { "Back to Home" }
                </a>
            </Link<Route>>
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

async fn get_feedback_for_date(date: &str) -> Result<String, String> {
    let target_url = format!("{BACKEND_URL}/feedback/{date}");

    let res = Request::get(&target_url)
        .send()
        .await
        .map_err(|e| format!("Failed to send request to {target_url}: {e}"))?;

    let res = res
        .json::<FeedbackResponse>()
        .await
        .map_err(|e| format!("Unable to parse response {res:?} as JSON: {e}"))?;

    res.feedback.ok_or_else(|| format!("No feedback found for date {date}"))
}

async fn parse_feedback(feedback: &str) -> Result<Vec<Vec<String>>, String> {
    const DASH_CNT: usize = 50;
    let feedback_time_regex = Regex::new(r"^\[\d{4}-\d{2}-\d{2} - (\d{2}:\d{2}:\d{2})]z$").unwrap();

    let mut feedbacks = vec![];
    let mut curr_lines = vec![];
    let mut active = false;

    for line in feedback.lines() {
        if line == "-".repeat(DASH_CNT) {
            active = !active;
            if !active { // => Just turned inactive
                feedbacks.push(curr_lines);
                curr_lines = vec![];
            } else {
                continue;
            }
        }

        if !active { continue; }

        if let Some(capture) = feedback_time_regex.captures(line) {
            curr_lines.push(capture[1].to_string());
        } else {
            curr_lines.push(line.to_string());
        }
    }

    Ok(feedbacks)
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
