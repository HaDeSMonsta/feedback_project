use crate::components::date::Date;
use crate::components::home::Home;
use crate::components::not_found::NotFound;
use crate::components::version::Version;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
pub mod functions;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const BACKEND_URL: &str = include_str!("../backend_url.txt");

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

fn main() {
    yew::Renderer::<Main>::new().render();
}
