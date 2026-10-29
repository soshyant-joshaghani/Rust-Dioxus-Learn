//! The home page: header, hero, catalogue, footer.

use dioxus::prelude::*;

// `use` brings names into scope so we can write `Header` instead of the full path.
use crate::modules::base::{footer::Footer, header::Header};
use crate::modules::games::{game_card::GameCard, GAMES};

#[component]
pub fn Home() -> Element {
    let games_count = GAMES.len();

    rsx! {
        div { class: "page",
            Header {}

            div { class: "hero",
                h1 { "Learn Rust by playing" }
                p { "{games_count} small games, one Rust idea each." }
            }

            div { class: "grid",
                for game in GAMES.iter() {
                    GameCard {
                        title: game.title.to_string(),
                        blurb: game.blurb.to_string(),
                        concept: game.concept.to_string(),
                    }
                }
            }

            Footer {}
        }
    }
}
