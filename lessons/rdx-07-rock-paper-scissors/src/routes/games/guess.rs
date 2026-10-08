//! `/games/guess`: Number Guessing, version 2.
//!
//! Compare with lesson 05: the component no longer knows the RULES. It owns a
//! `GuessGame` (from `modules/games/guess`) and only does three things:
//!   1. forward what the player typed,
//!   2. turn the answer into a message,
//!   3. draw.
//!
//! ```text
//!   page (this file)                      rules (modules/games/guess)
//!   ------------------                    ----------------------------
//!   Signal<Option<GuessGame>>  ---------> GuessGame { secret, tries, won }
//!   on_submit: game.guess_text(&text) --> Result<Hint, GuessError>
//!   message.set(text for hint/error) <--- Display for GuessError
//! ```

use dioxus::prelude::*;

use crate::modules::base::random::random_seed;
use crate::modules::base::ui::card::Card;
use crate::modules::games::guess::{GuessGame, Hint, MAX, MIN};
use crate::routes::Route;

/// Turn a hint into a sentence for the player. A free function: it needs no state.
fn describe(hint: Hint, tries: u32) -> String {
    match hint {
        Hint::TooLow => String::from("Too low."),
        Hint::TooHigh => String::from("Too high."),
        Hint::Correct => format!("Correct! You needed {tries} tries."),
    }
}

#[component]
pub fn Guess() -> Element {
    // The whole game is ONE value inside ONE signal. `None` until the seed arrives.
    let mut game = use_signal(|| None::<GuessGame>);
    let mut input = use_signal(String::new);
    let mut message = use_signal(|| String::from("Starting..."));

    let new_game = move || {
        spawn(async move {
            let seed = random_seed().await;
            // Assigning a new GuessGame DROPS the previous one (if any).
            game.set(Some(GuessGame::from_seed(seed)));
            input.set(String::new());
            message.set(format!("I am thinking of a number from {MIN} to {MAX}."));
        });
    };
    use_hook(new_game);

    let on_submit = move |event: FormEvent| {
        event.prevent_default();

        // `input()` CLONES the String out of the signal: we now own a copy and
        // hold no guard on `input`.
        let text = input();

        // Ask the rules. The inner block makes the WRITE GUARD short-lived:
        // it is dropped at the closing brace, before we touch other signals.
        let result = {
            let mut slot = game.write(); // a &mut to the Option<GuessGame>
            let Some(current) = slot.as_mut() else {
                return; // not started yet
            };
            // `current` is `&mut GuessGame`, so `guess_text` may change the game.
            let result = current.guess_text(&text);
            let tries = current.tries(); // read while we still hold the guard
            result.map(|hint| (hint, tries)) // Result<Hint,_> -> Result<(Hint,u32),_>
        };

        // `match` on the two possible shapes of the answer.
        match result {
            Ok((hint, tries)) => message.set(describe(hint, tries)),
            // `error.to_string()` uses the Display impl written in the rules module.
            Err(error) => message.set(error.to_string()),
        }
        input.set(String::new());
    };

    // Reading the game for the view: `Option::is_some_and` asks a question about
    // the inside of an Option without unwrapping.
    let won = game.read().as_ref().is_some_and(|g| g.is_won());
    let tries = game.read().as_ref().map_or(0, |g| g.tries());
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

            p { class: "muted", "Tries: {tries}" }
            button { class: "btn secondary", onclick: move |_| new_game(), "New game" }
        }
    }
}
