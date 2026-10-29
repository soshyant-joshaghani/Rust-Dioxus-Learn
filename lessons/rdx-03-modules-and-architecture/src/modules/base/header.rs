//! The top bar of every page.

use dioxus::prelude::*;

use crate::config::{APP_NAME, LESSON_TAG};

#[component]
pub fn Header() -> Element {
    rsx! {
        div { class: "header",
            span { class: "brand", "{APP_NAME}" }
            span { class: "tag", "{LESSON_TAG}" }
        }
    }
}
