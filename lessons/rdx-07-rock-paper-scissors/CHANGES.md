# Lesson 07 - CHANGES

Previous: lesson 06 (guessing game with a rules module and tests).

## Commands

```bash
cp -r rdx-06-structs-results-and-tests rdx-07-rock-paper-scissors
cd rdx-07-rock-paper-scissors
mkdir src/modules/games/rps
# add/edit files below, then
cargo clippy --all-targets
cargo test
dx serve
cargo run --example exp_copy_clone
```

## Files

| File | Change |
|------|--------|
| `Cargo.toml` | name only |
| `src/config/mod.rs` | `LESSON_TAG = "lesson 07"` |
| `src/modules/games/rps/mod.rs` | **new**: `Move`, `Outcome`, `judge`, `Round`, `Score`, `RpsGame`, 6 tests |
| `src/modules/games/mod.rs` | `pub mod rps;`; concept text "enums + match" |
| `src/routes/games/rps.rs` | **new**: the page |
| `src/routes/games/mod.rs` | `pub mod rps;` and `"rps" => Some(Route::Rps {})` in `route_for` |
| `src/routes/mod.rs` | `Route::Rps` (`/games/rps`); new method `Route::is_in_games()` |
| `src/routes/layout.rs` | `NavItem::Games => current.is_in_games()` |
| `assets/main.css` | `.row .btn { margin-top: 0; flex: 1; }` |
| `examples/exp_copy_clone.rs` | **new** |

## Why `is_in_games()`?

Lesson 05 listed every game route inside `layout.rs`'s `matches!`. A new game would need an edit in `routes/mod.rs` **and** `layout.rs`. Moving the question to the enum (`impl Route { fn is_in_games }`) leaves one list to maintain, right next to the variants. (Lesson 09 goes one step further.)

## The Route enum now

```rust
#[layout(AppLayout)]
    #[route("/")]           Home {},
    #[route("/games")]      Games {},
    #[route("/games/guess")] Guess {},
    #[route("/games/rps")]  Rps {},
    #[route("/about")]      About {},
#[end_layout]
```

## Compile errors that happened while writing this lesson

1. `cannot borrow play as mutable` (E0596). The closure `play` calls `game.write()`, a method that needs `&mut`, so the closure is `FnMut` and its variable must be `let mut play`. See KNOWLEDGE section 6.
2. `method rounds is never used` warning. A method used only by tests is dead code in the app. We fixed it by showing "Rounds N" on the page. Dead-code warnings are the compiler asking: "do you really need this?"

## Verify

1. `cargo test`: `17 passed` (6 new).
2. `/games` shows two Play buttons; `/games/rps` plays, scores add up, "Reset score" clears.
3. In `judge`, delete the line `(Rock, Paper) | ...`. The compiler lists the missing patterns: `(Rock, Paper)`, `(Paper, Scissors)`, `(Scissors, Rock)` not covered.
4. `cargo run --example exp_copy_clone`.
