//! Reusable hooks (functions that use `use_signal`, `use_hook`...).
//!
//! THE DUPLICATION THIS REMOVES
//!   Lesson 06 (guess) and lesson 07 (rock paper scissors) both contained the
//!   same 8 lines: "keep an `Option<Game>` in a signal, ask JavaScript for a
//!   seed, build the game from it, and do it again on 'New game'".
//!   Snake (lesson 10) would be the third copy. Written once, here, as a GENERIC
//!   function over the game type.

use dioxus::prelude::*;

use crate::modules::base::random::random_seed;

/// Create a game that needs a random seed.
///
/// Returns TWO things (a tuple):
///   1. the signal holding `Option<G>`: `None` until the seed has arrived
///   2. a `restart` function: call it to start a fresh game with a new seed
///
/// THE GENERIC PARTS
///   `<G: 'static>` : any game type G. `'static` here means "G does not contain
///                    short-lived borrows" (a signal's storage needs that: it can
///                    outlive any function). All our games own their data, so OK.
///   `make: fn(u64) -> G` : a plain function POINTER. We pass `GuessGame::from_seed`
///                    itself: no closure needed.
///   `impl Fn() + Copy + 'static` (return type): "some closure that can be called
///                    many times and copied". We do not have to name its type.
pub fn use_seeded<G: 'static>(make: fn(u64) -> G) -> (Signal<Option<G>>, impl Fn() + Copy + 'static) {
    let mut slot = use_signal(|| None::<G>);

    let restart = move || {
        spawn(async move {
            let seed = random_seed().await;
            slot.set(Some(make(seed)));
        });
    };

    // First render only: start the first game.
    use_hook(restart);

    (slot, restart)
}
