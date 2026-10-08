//! The root component: stylesheet + the router. Nothing else.

use dioxus::prelude::*;

use crate::config::APP_NAME;
use crate::routes::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Title { "{APP_NAME}" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        // `Router::<Route>` reads the current URL, finds the matching variant of
        // `Route`, and draws the page for it. The `::<Route>` is a generic
        // argument: "a router for THIS enum" (lesson 09 explains generics).
        Router::<Route> {}
    }
}
