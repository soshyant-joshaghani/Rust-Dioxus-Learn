//! One game in the catalogue grid.

use dioxus::prelude::*;

// `crate::` means "start at the crate root (main.rs)". Absolute paths keep
// imports stable when files move around.
use crate::modules::base::ui::card::Card;

/// Props are owned Strings (see lesson 02, "Props and ownership").
#[component]
pub fn GameCard(title: String, blurb: String, concept: String) -> Element {
    rsx! {
        // Our own `Card` wraps the content. This is the `children` prop in action.
        Card {
            h3 { "{title}" }
            p { "{blurb}" }
            span { class: "tag", "{concept}" }
        }
    }
}
