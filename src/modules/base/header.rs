//! The top bar of every page: brand on the left, whatever you pass as
//! `children` (the navigation links) on the right.
//!
//! This component does not know the routes. That is deliberate: it stays
//! reusable and `modules` does not depend on `routes`.

use dioxus::prelude::*;

use crate::config::{APP_NAME, VERSION};
use crate::modules::base::theme_toggle::ThemeToggle;

#[component]
pub fn Header(children: Element) -> Element {
    rsx! {
        div { class: "header",
            span { class: "brand", "{APP_NAME}" }
            span { class: "tag", "v{VERSION}" }
            nav { class: "nav", {children} }
            ThemeToggle {}
        }
    }
}
