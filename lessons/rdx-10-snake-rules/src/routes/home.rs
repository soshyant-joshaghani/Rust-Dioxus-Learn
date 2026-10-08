//! `/`: the landing page.

use dioxus::prelude::*;

use crate::modules::games::GAMES;
use crate::routes::Route;

#[component]
pub fn Home() -> Element {
    let games_count = GAMES.len();

    rsx! {
        div { class: "hero",
            h1 { "Learn Rust by playing" }
            p { "{games_count} small games, one Rust idea each." }
            // A Link takes a `Route` VALUE. Typos are impossible: `Route::Gamez {}` does not compile.
            Link { to: Route::Games {}, class: "btn", "See the games" }
        }
    }
}
