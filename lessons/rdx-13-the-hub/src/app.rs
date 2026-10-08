//! The root component: stylesheet + the router. Nothing else.

use dioxus::prelude::*;

use crate::config::APP_NAME;
use crate::modules::base::theme::use_theme_provider;
use crate::modules::games::progress::use_progress_provider;
use crate::routes::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    // Create the shared progress store and load the saved data. Every page below can
    // reach it with `use_progress()`.
    use_progress_provider();
    // Same idea for the theme: load the saved choice, share it with every component.
    let theme = use_theme_provider();

    rsx! {
        document::Title { "{APP_NAME}" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        // `Router::<Route>` reads the current URL, finds the matching variant of
        // `Route`, and draws the page for it. The `::<Route>` is a generic
        // argument: "a router for THIS enum" (lesson 09 explains generics).
        // The wrapper's CLASS (`dark` / `light`) decides which colour variables are active:
        // see `.app.light` in assets/main.css. Changing the signal changes the class,
        // and CSS does the rest.
        div { class: "app {theme.get().css_class()}",
            Router::<Route> {}
        }
    }
}
