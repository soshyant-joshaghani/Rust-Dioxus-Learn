//! One game in the catalogue grid.

use dioxus::prelude::*;

use crate::modules::base::ui::card::Card;

/// `children` is a slot for the action area (a "Play" link or "coming soon").
/// The card does not know where that link goes: that is the page's job.
#[component]
pub fn GameCard(title: String, blurb: String, concept: String, children: Element) -> Element {
    rsx! {
        Card {
            h3 { "{title}" }
            p { "{blurb}" }
            span { class: "tag", "{concept}" }
            div { class: "card-action", {children} }
        }
    }
}
