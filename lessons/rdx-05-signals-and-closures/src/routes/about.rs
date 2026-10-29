//! `/about`: what this project is.

use dioxus::prelude::*;

use crate::modules::base::ui::card::Card;

#[component]
pub fn About() -> Element {
    rsx! {
        h2 { class: "title", "About foxg-kit" }
        Card {
            p {
                "foxg-kit is a collection of small, readable starter kits. "
                "This one is a game hub written in Rust with Dioxus, built lesson by lesson."
            }
            p { "Every lesson adds one Rust idea and one piece of the hub." }
        }
    }
}
