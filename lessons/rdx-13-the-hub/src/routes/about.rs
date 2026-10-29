//! `/about`: what this project is, and facts about the running program.

use dioxus::prelude::*;

use crate::config::{platform_name, APP_NAME, VERSION};
use crate::modules::base::storage::location_hint;
use crate::modules::base::ui::card::Card;
use crate::modules::games::GAMES;

#[component]
pub fn About() -> Element {
    rsx! {
        h2 { class: "title", "About foxg-kit" }

        Card {
            p {
                "foxg-kit is a collection of small, readable starter kits. "
                "{APP_NAME} is the Rust and Dioxus one: a game hub built lesson by lesson, "
                "where every game teaches one Rust idea."
            }
            p { class: "muted",
                "Version {VERSION}  |  running on {platform_name()}  |  data: {location_hint()}"
            }
        }

        h3 { class: "title", "What each game teaches" }
        Card {
            ul { class: "stats",
                // The same GAMES table that feeds the catalogue: one source of truth.
                for game in GAMES.iter() {
                    li { "{game.title}: {game.concept}" }
                }
            }
        }

        h3 { class: "title", "Where to read more" }
        Card {
            p {
                "Every lesson lives in "
                code { "lessons/rdx-NN-name/" }
                " with a README (short), CHANGES (exact commands) and KNOWLEDGE (the book part)."
            }
            p { class: "muted",
                "Small experiments live in "
                code { "examples/" }
                ": run one with "
                code { "cargo run --example exp_ownership" }
                "."
            }
        }
    }
}
