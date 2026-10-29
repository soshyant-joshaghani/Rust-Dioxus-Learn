//! `/games/guess`: Number Guessing, version 1.
//!
//! Everything is written directly inside the component so you can see the raw
//! ingredients: SIGNALS (state), CLOSURES (event handlers), `match`, `Result`.
//! Lesson 06 will move the rules out of the component into their own module.
//!
//! HOW THE GAME FLOWS
//!
//! ```text
//!   page opens ----> new_game() ----> asks JS for a seed ----> secret = Some(n)
//!                                                                  |
//!   type "42" ---- oninput ----> `input` signal                    |
//!   press Guess -- onsubmit ---> parse "42"                        |
//!        |  Err -> message "not a number"                          |
//!        |  Ok(42) -> compare with secret --> message "higher/lower/correct"
//!        v
//!   any signal changed -> Dioxus draws this component again
//! ```

use std::cmp::Ordering;

use dioxus::prelude::*;

use crate::modules::base::random::random_seed;
use crate::modules::base::ui::card::Card;
use crate::routes::Route;

const MIN: u32 = 1;
const MAX: u32 = 100;

#[component]
pub fn Guess() -> Element {
    // ---- state: SIGNALS ----------------------------------------------------
    // `use_signal(initial)` creates a value that Dioxus WATCHES. Reading it in
    // the markup subscribes this component; writing to it re-draws the component.
    //
    // `let mut`: the binding is `mut` because `.set(...)` and `+=` need `&mut self`.
    // `Signal<T>` is `Copy` (it is a small handle to the real data, which Dioxus
    // keeps elsewhere), so it can be given to many closures without clone().
    //
    // The secret is `Option<u32>`: `None` until the random seed arrives.
    let mut secret = use_signal(|| None::<u32>);
    let mut input = use_signal(String::new);
    let mut message = use_signal(|| String::from("Starting..."));
    let mut tries = use_signal(|| 0u32);
    let mut won = use_signal(|| false);

    // ---- a CLOSURE that starts a new game ---------------------------------
    // A closure is an anonymous function that CAPTURES variables around it.
    // `move` = "take the captured variables with you" (here they are Copy
    // signals, so each use gets its own cheap copy; the data is still shared).
    //
    // Because it only captures Copy things, the closure itself is `Copy`. That
    // is why we may call it from `use_hook` and from the button below.
    let new_game = move || {
        // `spawn` starts an async task. We need it because `random_seed()` must be
        // awaited and a component function cannot wait while it is drawing.
        spawn(async move {
            let seed = random_seed().await;
            // Map the big seed into MIN..=MAX with the remainder operator `%`.
            let number = (seed % u64::from(MAX - MIN + 1)) as u32 + MIN;

            secret.set(Some(number));
            input.set(String::new());
            tries.set(0);
            won.set(false);
            message.set(format!("I am thinking of a number from {MIN} to {MAX}."));
        });
    };

    // `use_hook` runs its closure only on the FIRST render of the component.
    use_hook(new_game);

    // ---- the guess handler -------------------------------------------------
    let on_submit = move |event: FormEvent| {
        // A <form> would reload the page by default. Stop that.
        event.prevent_default();

        // let-else: unwrap the Option or leave the function early.
        // `*secret.read()` copies the u32 out of the signal.
        let Some(secret_number) = *secret.read() else {
            return; // the game has not started yet
        };
        if won() {
            return; // finished: the button does nothing until "New game"
        }

        // IMPORTANT, this is a borrow rule at RUN time:
        // `input.read()` gives a temporary READ GUARD. While a guard exists you must
        // not WRITE the same signal, or Dioxus panics. So we finish with the guard
        // in this one statement and keep only the parsed OWNED result.
        let parsed = input.read().trim().parse::<u32>();

        // `parse` returns Result<u32, ParseIntError>. `match` handles both cases.
        match parsed {
            Err(_) => message.set(String::from("That is not a whole number. Try again.")),
            Ok(guess) if !(MIN..=MAX).contains(&guess) => {
                // a "match guard": the arm applies only when the `if` is true.
                // `(MIN..=MAX)` is a RANGE value; `contains` asks "is it inside?".
                message.set(format!("Pick a number from {MIN} to {MAX}."));
            }
            Ok(guess) => {
                tries += 1;

                // `cmp` returns the enum `Ordering { Less, Equal, Greater }`.
                match guess.cmp(&secret_number) {
                    Ordering::Less => message.set(format!("{guess} is too low.")),
                    Ordering::Greater => message.set(format!("{guess} is too high.")),
                    Ordering::Equal => {
                        won.set(true);
                        message.set(format!("Correct! {guess} in {} tries.", tries()));
                    }
                }
            }
        }

        input.set(String::new());
    };

    // ---- the view ------------------------------------------------------------
    let message_class = if won() { "message win" } else { "message" };

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
                    // The input SHOWS the signal and WRITES to it: a two-way link.
                    value: "{input}",
                    oninput: move |event| input.set(event.value()),
                }
                button { class: "btn", r#type: "submit", "Guess" }
            }

            p { class: "muted", "Tries: {tries}" }
            // `new_game` is Copy, so using it here does not "use it up".
            button { class: "btn secondary", onclick: move |_| new_game(), "New game" }
        }
    }
}
