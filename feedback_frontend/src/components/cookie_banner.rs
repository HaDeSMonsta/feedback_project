use yew::prelude::*;

#[cfg(not(feature = "cookie_banner"))]
#[function_component(CookieBanner)]
pub fn cookie_banner() -> Html {
    html! {}
}

#[cfg(feature = "cookie_banner")]
#[function_component(CookieBanner)]
pub fn cookie_banner() -> Html {
    let show_banner = use_state(|| true);

    if !*show_banner {
        return html! {};
    }

    html! {
        <div class={classes!(
            "fixed",
            "bottom-4",
            "left-1/2",
            "transform",
            "-translate-x-1/2",
            "bg-gray-100",
            "dark:bg-gray-800",
            "p-4",
            "flex",
            "justify-center",
            "items-center",
            "shadow-lg",
            "rounded-lg",
            "border",
            "border-gray-200",
            "dark:border-gray-700",
            "max-w-xl",
            "w-[95%]",
        )}>
            <div class={classes!("flex", "flex-col", "sm:flex-row", "items-center", "gap-4", "text-center")}>
                <p class={classes!("text-gray-800", "dark:text-gray-200")}>
                    { "We don't use any cookies." }
                </p>
                <button
                    onclick={
                        let show_banner = show_banner.clone();
                        Callback::from(move |_| show_banner.set(false))
                    }
                    class={classes!(
                        "px-4",
                        "py-2",
                        "bg-indigo-700",
                        "hover:bg-indigo-800",
                        "text-white",
                        "rounded",
                        "text-sm",
                        "whitespace-nowrap"
                    )}
                >
                    { "Got it, don't show again" }
                </button>
            </div>
        </div>
    }
}
