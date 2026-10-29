//! `/games/rps`: Rock Paper Scissors, using the shared `use_seeded` hook and the `Game` trait.

use dioxus::prelude::*;

use crate::modules::base::hooks::use_seeded;
use crate::modules::base::ui::card::Card;
use crate::modules::games::describe;
use crate::modules::games::progress::use_progress;
use crate::modules::games::rps::{Move, Outcome, RpsGame};
use crate::routes::Route;

/// One sentence about a finished round. A `match` on the Outcome enum.
fn headline(outcome: Outcome) -> &'static str {
    match outcome {
        Outcome::Win => "You win!",
        Outcome::Lose => "You lose.",
        Outcome::Draw => "Draw.",
    }
}

#[component]
pub fn Rps() -> Element {
    let (mut game, _restart) = use_seeded(RpsGame::new);
    let progress = use_progress();

    let mut play = move |player: Move| {
        // The write guard lives for this one call; no other signal is touched.
        if let Some(g) = game.write().as_mut() {
            g.play(player);
            progress.update(|p| p.record_rps_round());
        }
    };

    // `last_round()` returns a Copy value, `describe` builds an owned String:
    // nothing borrowed from the signal survives this statement.
    let (status, last) = match game.read().as_ref() {
        Some(g) => (describe(g), g.last_round()),
        None => (String::from("Starting..."), None),
    };

    rsx! {
        Link { to: Route::Games {}, class: "muted", "< All games" }
        h2 { class: "title", "Rock Paper Scissors" }

        Card {
            match last {
                None => rsx! { p { class: "message", "Pick your move." } },
                Some(round) => rsx! {
                    p { class: "message",
                        "{round.player} vs {round.computer}: "
                        strong { "{headline(round.outcome)}" }
                    }
                },
            }

            div { class: "row",
                for mv in Move::ALL {
                    button { class: "btn", onclick: move |_| play(mv), "{mv}" }
                }
            }

            p { class: "muted", "{status}" }
            button {
                class: "btn secondary",
                onclick: move |_| {
                    if let Some(g) = game.write().as_mut() {
                        g.reset_score();
                    }
                },
                "Reset score"
            }
        }
    }
}
