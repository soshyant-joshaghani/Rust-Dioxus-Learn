//! `/games/rps`: Rock Paper Scissors.
//!
//! The page owns one `RpsGame` inside a signal and draws:
//! three buttons (one per `Move`), the last round, and the score.

use dioxus::prelude::*;

use crate::modules::base::random::random_seed;
use crate::modules::base::ui::card::Card;
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
    let mut game = use_signal(|| None::<RpsGame>);

    // Same start-up pattern as the guessing game: wait for a seed, then create the game.
    let new_game = move || {
        spawn(async move {
            let seed = random_seed().await;
            game.set(Some(RpsGame::new(seed)));
        });
    };
    use_hook(new_game);

    // `play` is a closure that takes the player's Move.
    // Move is Copy, so each button gets its own copy of its Move.
    let mut play = move |player: Move| {
        // The write guard from `game.write()` lives for the whole `if let` body (a
        // temporary in the condition). That is fine here: the body is ONE call and
        // touches no other signal. (In lesson 06 we needed an explicit block.)
        if let Some(g) = game.write().as_mut() {
            g.play(player);
        }
    };

    // Read what the view needs. `last_round()` and `score()` return COPIES
    // (Round and Score are Copy), so no guard is kept.
    let (score, last) = match game.read().as_ref() {
        Some(g) => (g.score(), g.last_round()),
        None => Default::default(), // (Score::default(), None)
    };

    rsx! {
        Link { to: Route::Games {}, class: "muted", "< All games" }
        h2 { class: "title", "Rock Paper Scissors" }

        Card {
            // `match` on an Option<Round>: no round yet, or show it.
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
                // Move::ALL is an array of Copy values: iterating it by value is free.
                for mv in Move::ALL {
                    button { class: "btn", onclick: move |_| play(mv), "{mv}" }
                }
            }

            p { class: "muted",
                "Rounds {score.rounds()}  |  Wins {score.wins}  |  Losses {score.losses}  |  Draws {score.draws}"
            }
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
