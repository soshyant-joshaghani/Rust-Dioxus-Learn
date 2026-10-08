//! `/games`: the catalogue of games (it was the home page in lesson 03).

use dioxus::prelude::*;

use crate::modules::games::{game_card::GameCard, GAMES};

#[component]
pub fn Games() -> Element {
    rsx! {
        h2 { class: "title", "Games" }
        div { class: "grid",
            for game in GAMES.iter() {
                GameCard {
                    title: game.title.to_string(),
                    blurb: game.blurb.to_string(),
                    concept: game.concept.to_string(),
                }
            }
        }
    }
}
