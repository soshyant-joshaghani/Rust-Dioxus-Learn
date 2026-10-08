//! `/games`: the catalogue. Each game has its own page in this folder.

use dioxus::prelude::*;

pub mod guess;
pub mod rps;
pub mod tictactoe;

use crate::modules::games::{game_card::GameCard, GAMES};
use crate::routes::Route;

/// Which page belongs to which game slug?
///
/// Returns `Option<Route>`: `Some(route)` for games that exist, `None` for
/// games that are listed but not written yet. Returning a fake route or an
/// empty string would hide that case; `Option` makes the caller handle it.
fn route_for(slug: &str) -> Option<Route> {
    match slug {
        "guess" => Some(Route::Guess {}),
        "rps" => Some(Route::Rps {}),
        "tictactoe" => Some(Route::TicTacToe {}),
        _ => None,
    }
}

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
                    // `match` is an expression, so it can produce the Element to show.
                    match route_for(game.slug) {
                        Some(route) => rsx! { Link { to: route, class: "btn", "Play" } },
                        None => rsx! { span { class: "muted", "coming soon" } },
                    }
                }
            }
        }
    }
}
