//! The bottom line of every page.

use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        div { class: "footer", "Built with Rust and Dioxus." }
    }
}
