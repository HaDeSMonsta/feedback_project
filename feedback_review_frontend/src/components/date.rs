use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::footer::Footer;
use crate::functions::{get_all_dates, get_feedback_for_date, parse_feedback};
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct DateProps {
    pub date: String,
}

#[function_component(Date)]
pub fn date(props: &DateProps) -> Html {
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

