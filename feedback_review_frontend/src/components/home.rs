use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::footer::Footer;
use crate::functions::get_all_dates;
use crate::Route;

#[function_component(Home)]
pub fn home() -> Html {
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

