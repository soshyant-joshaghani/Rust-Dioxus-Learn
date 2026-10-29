# Lesson 05 - Signals and closures: Number Guessing

> Goal: our first **playable game**, and our first **state that changes**. The central ideas: signals (Dioxus) and closures (Rust).

## What you build

`/games/guess`: the computer picks a secret number from 1 to 100; you guess; it says "too low", "too high" or "Correct!" and counts your tries. The catalogue (`/games`) now has a working **Play** button on this game; the other three say "coming soon".

## What you learn

| Rust | Dioxus |
|------|--------|
| `let mut`, mutation | `use_signal`: state that redraws the page |
| **closures**: capture, `move`, Copy closures | event handlers: `onclick`, `oninput`, `onsubmit` |
| `Result` + `match` + guards | forms and inputs bound to a signal |
| `Option` + `let ... else` | `spawn` and `async` (just enough) |
| `std::cmp::Ordering` (an enum) | `use_hook`: run once on first render |
| ranges `(1..=100).contains(&n)` | runtime borrow rules of signals (read guards) |

## Run

```bash
cd lessons/rdx-05-signals-and-closures
dx serve                         # open /games/guess
cargo run --example exp_closures
```

Try `abc`, `0`, `500`, then play for real. Binary search (always guess the middle) wins in at most 7 tries.

## Files

```
src/routes/games/mod.rs        catalogue; `route_for(slug) -> Option<Route>`
src/routes/games/guess.rs      THE GAME (state + handlers + view in one component)
src/modules/base/random.rs     `random_seed()`: random numbers via JavaScript
src/modules/games/mod.rs       GameInfo gets a `slug`
src/modules/games/game_card.rs card gets a `children` slot for the Play link
src/routes/mod.rs              + Route::Guess
examples/exp_closures.rs       closures lab
```

## Challenge

Limit the player to 7 tries: after the 7th wrong guess the game is lost and reveals the secret. Which signal do you add, and which `match` arm changes?

## Read next

`CHANGES.md` -> `KNOWLEDGE.md` -> [lesson 06](../rdx-06-structs-results-and-tests/README.md): move the rules out of the component, so they can be tested.
