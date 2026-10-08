# Lesson 09 - CHANGES

Previous: lesson 08 (three games, each page with its own start-up and status code).

## Commands

```bash
cp -r rdx-08-tic-tac-toe rdx-09-traits-and-generics
cd rdx-09-traits-and-generics
# edit/add files below, then
cargo clippy --all-targets
cargo test
dx serve
cargo run --example exp_traits_generics
```

## Files

| File | Change |
|------|--------|
| `Cargo.toml` | name only |
| `src/config/mod.rs` | `LESSON_TAG = "lesson 09"` |
| `src/modules/games/mod.rs` | **new**: `trait Game`, `fn describe<G: Game>`, 3 tests (incl. a fake `Dummy` game and `&dyn Game`) |
| `src/modules/games/guess/mod.rs` | `impl Game for GuessGame` |
| `src/modules/games/rps/mod.rs` | `impl Game for RpsGame` (never "over") |
| `src/modules/games/tictactoe/mod.rs` | `impl Game for Board` |
| `src/modules/base/hooks.rs` | **new**: `use_seeded<G: 'static>(make: fn(u64) -> G)` |
| `src/modules/base/mod.rs` | `pub mod hooks;` |
| `src/routes/games/guess.rs` | uses `use_seeded` and `describe`; start-up code deleted |
| `src/routes/games/rps.rs` | same; "Rounds N | Wins..." now comes from `Game::summary` |
| `src/routes/games/tictactoe.rs` | `status_text` deleted; `describe(&*board.read())` |
| `examples/exp_traits_generics.rs` | **new** |

## The trait

```rust
pub trait Game {
    fn is_over(&self) -> bool;
    fn summary(&self) -> String;
    fn headline(&self) -> String {           // default method
        if self.is_over() { format!("Finished. {}", self.summary()) } else { self.summary() }
    }
}
pub fn describe<G: Game>(game: &G) -> String { game.headline() }
```

## The generic hook

```rust
pub fn use_seeded<G: 'static>(make: fn(u64) -> G)
    -> (Signal<Option<G>>, impl Fn() + Copy + 'static)
```
Usage: `let (mut game, restart) = use_seeded(GuessGame::from_seed);`

## What happened to the old getters?

After the refactor the compiler warned that `is_won`, `score`, `status`, `next_player` were **never used**: the pages no longer call them. The `Game` impls now call them (`self.is_won()`, `self.score()`...), so each accessor has one clear reason to exist. Dead-code warnings are the compiler telling you which refactor leftovers to delete or re-use.

## Behaviour changes visible to the player

- Guess: the line under the input says `Tries: N`, and after a win `Finished. Solved in N tries.` (from `headline`).
- Rock Paper Scissors: `Rounds: N  Wins: N  Losses: N  Draws: N`.
- Tic-Tac-Toe: the status text is the same, now produced by the trait.

## Verify

1. `cargo test`: 30 passed.
2. All three game pages work in the browser (checked: binary search wins at the guessing game, six RPS rounds add up, reset clears).
3. `cargo run --example exp_traits_generics`: five numbered sections.
4. Try the error from the challenge: call `describe(&5)` anywhere. Read: "the trait bound `i32: Game` is not satisfied".
