//! `/`: the landing page, with the player's saved progress.

use dioxus::prelude::*;

use crate::modules::base::storage::location_hint;
use crate::modules::base::ui::card::Card;
use crate::modules::games::progress::{use_progress, Progress};
use crate::modules::games::GAMES;
use crate::routes::Route;

#[component]
pub fn Home() -> Element {
    let games_count = GAMES.len();

    // The store was created in `App`. Reading `get()` subscribes this component:
    // when a game records something, this card updates.
    let store = use_progress();
    let progress = store.get();

    // `Option<u32>` -> text. `map_or_else`: one closure for Some, one for None.
    let best_guess = progress
        .guess_best_tries
        .map_or_else(|| String::from("not yet"), |tries| format!("{tries} tries"));

    rsx! {
        div { class: "hero",
            h1 { "Learn Rust by playing" }
            p { "{games_count} small games, one Rust idea each." }
            Link { to: Route::Games {}, class: "btn", "See the games" }
        }

        Card {
            h3 { "Your progress" }
            ul { class: "stats",
                li { "Number Guessing, best: {best_guess}" }
                li { "Rock Paper Scissors, rounds played: {progress.rps_rounds}" }
                li { "Tic-Tac-Toe, games finished: {progress.tictactoe_games}" }
                li { "Snake, best score: {progress.snake_best}" }
            }
            // A storage problem is shown, not hidden.
            if let Some(problem) = store.notice() {
                p { class: "message error", "{problem}" }
            }
            p { class: "muted", "Saved in: {location_hint()}" }
            button {
                class: "btn secondary",
                onclick: move |_| store.update(|p| *p = Progress::default()),
                "Reset progress"
            }
        }
    }
}
