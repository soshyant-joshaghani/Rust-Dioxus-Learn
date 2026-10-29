//! The root component: the only place that knows about the stylesheet and
//! which page is shown. Lesson 04 replaces the hard-coded `Home {}` with a router.

use dioxus::prelude::*;

use crate::config::APP_NAME;
use crate::routes::home::Home;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

/// `pub` makes `App` visible outside this file, so that `main.rs` can use it.
/// Without `pub` the item is private to the module (`app`) it is defined in.
#[component]
pub fn App() -> Element {
    rsx! {
        document::Title { "{APP_NAME}" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Home {}
    }
}
