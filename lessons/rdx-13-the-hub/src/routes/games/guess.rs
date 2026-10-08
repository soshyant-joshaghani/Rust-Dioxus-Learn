//! `/games/guess`: Number Guessing, version 3.
//!
//! Compared with lesson 06 the start-up code (seed, `Option<GuessGame>`, restart)
//! is gone: `use_seeded` does it for every game. The status line comes from the
//! `Game` trait through `describe`.

use dioxus::prelude::*;

use crate::modules::base::hooks::use_seeded;
use crate::modules::base::ui::card::Card;
use crate::modules::games::guess::{GuessGame, Hint, MAX, MIN};
use crate::modules::games::progress::use_progress;
use crate::modules::games::{describe, Game};
use crate::routes::Route;

/// Turn a hint into a sentence for the player.
fn describe_hint(hint: Hint, tries: u32) -> String {
    match hint {
        Hint::TooLow => String::from("Too low."),
        Hint::TooHigh => String::from("Too high."),
        Hint::Correct => format!("Correct! You needed {tries} tries."),
    }
}

#[component]
pub fn Guess() -> Element {
    // One line replaces the signal + async + spawn + use_hook of lesson 06.
    // `GuessGame::from_seed` is passed as a function pointer: `fn(u64) -> GuessGame`.
    let (mut game, restart) = use_seeded(GuessGame::from_seed);
    let progress = use_progress(); // the shared store from `App`

    let mut input = use_signal(String::new);
    let mut message = use_signal(|| format!("I am thinking of a number from {MIN} to {MAX}."));

    let on_submit = move |event: FormEvent| {
        event.prevent_default();
        let text = input();

        // Short-lived write guard (see lesson 06).
        let result = {
            let mut slot = game.write();
            let Some(current) = slot.as_mut() else {
                return;
            };
            let result = current.guess_text(&text);
            let tries = current.tries();
            result.map(|hint| (hint, tries))
        };

        match result {
            Ok((hint, tries)) => {
                if hint == Hint::Correct {
                    // Saved to disk / browser storage by the store.
                    progress.update(|p| p.record_guess(tries));
                }
                message.set(describe_hint(hint, tries));
            }
            Err(error) => message.set(error.to_string()),
        }
        input.set(String::new());
    };

    // `is_over()` and `describe()` come from the `Game` trait. A trait's methods
    // can only be called when the trait is IN SCOPE: that is why `Game` is imported.
    let won = game.read().as_ref().is_some_and(|g| g.is_over());
    let status = game.read().as_ref().map_or(String::from("Starting..."), describe);
    let message_class = if won { "message win" } else { "message" };

    rsx! {
        Link { to: Route::Games {}, class: "muted", "< All games" }
        h2 { class: "title", "Number Guessing" }

        Card {
            p { class: "{message_class}", "{message}" }

            form { class: "row", onsubmit: on_submit,
                input {
                    r#type: "text",
                    inputmode: "numeric",
                    placeholder: "your guess",
                    value: "{input}",
                    oninput: move |event| input.set(event.value()),
                }
                button { class: "btn", r#type: "submit", "Guess" }
            }

            p { class: "muted", "{status}" }
            button {
                class: "btn secondary",
                onclick: move |_| {
                    restart();
                    input.set(String::new());
                    message.set(format!("I am thinking of a number from {MIN} to {MAX}."));
                },
                "New game"
            }
        }
    }
}
