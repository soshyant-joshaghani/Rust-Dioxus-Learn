# Lesson 07 - Rock Paper Scissors

> Goal: a second game, built the lesson-06 way (rules in `modules`, page in `routes`), to dig into **enums, `match` on tuples, and the difference between `Copy`, `Clone` and move**.

## What you build

`/games/rps`: three buttons (Rock / Paper / Scissors). The computer picks a move with our `Rng`; the page shows "Rock vs Scissors: **You win!**" and keeps a running score.

## What you learn

| Rust | Project |
|------|---------|
| enums with plain variants: `Move`, `Outcome` | the vocabulary of the game |
| `match` on a **tuple** `(player, computer)`, exhaustive, with `\|` | `judge()` |
| `impl` on enums, associated `const ALL` | `Move::label`, `Move::ALL` |
| `Copy` / `Clone` / move, `derive(Default)` | `Round`, `Score` (Copy) vs `RpsGame` (not Copy) |
| arrays and `usize` indexing | `Move::ALL[index]` |
| pure functions | `judge` is testable with a table |
| `FnMut` closures that are `Copy` | `play` called from three buttons |

## Run

```bash
cd lessons/rdx-07-rock-paper-scissors
cargo test                      # 17 tests
dx serve                        # /games/rps
cargo run --example exp_copy_clone
```

## Files

```
src/modules/games/rps/mod.rs   Move, Outcome, judge, Round, Score, RpsGame (+6 tests)
src/routes/games/rps.rs        the page
src/routes/mod.rs              + Route::Rps, + Route::is_in_games()
src/routes/layout.rs           uses is_in_games() (one place to edit per new game)
examples/exp_copy_clone.rs     Copy / Clone / move lab
```

## Challenge

Add a fourth move, `Lizard`, that beats Paper and loses to Rock and Scissors... (Spock is optional). Follow the compiler errors: every `match` that forgets `Lizard` fails. Count how many places you had to edit, and decide if `judge` stays a good design for 5 moves.

## Read next

`CHANGES.md` -> `KNOWLEDGE.md` -> [lesson 08](../rdx-08-tic-tac-toe/README.md).
