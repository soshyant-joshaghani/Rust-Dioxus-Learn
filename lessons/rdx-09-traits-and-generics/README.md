# Lesson 09 - Traits and generics

> Goal: the three games have grown similar. Instead of copy-pasting, we introduce **a trait** (`Game`) and **generic code** (`describe<G: Game>`, `use_seeded<G>`). Nothing new appears on screen except status lines; the win is that a fourth game will cost far less code.

## What changes

```
before (lesson 08)                         after (lesson 09)
guess.rs: 12 lines of seed/async start     use_seeded(GuessGame::from_seed)      (1 line)
rps.rs:   same 8 lines again               use_seeded(RpsGame::new)              (1 line)
each page: own status wording              trait Game { is_over, summary, headline }
                                           fn describe<G: Game>(&G) -> String
```

## What you learn

| Rust | Project |
|------|---------|
| `trait`: required + default methods | `Game` |
| `impl Trait for Type` | `impl Game for GuessGame / RpsGame / Board` |
| generic functions `<G: Game>` and **trait bounds** | `describe`, `use_seeded<G: 'static>` |
| static dispatch (monomorphization) vs `dyn Trait` | `[&dyn Game; 2]` in a test |
| function pointers `fn(u64) -> G`, `impl Fn() + Copy` | `use_seeded(GuessGame::from_seed)` |
| generic structs, `impl<T>` | `Stack<T>` experiment |
| a trait must be **in scope** to call its methods | `use crate::modules::games::Game` |
| dead-code warnings after a refactor | getters now used by the trait impls |

## Run

```bash
cd lessons/rdx-09-traits-and-generics
cargo test                          # 30 tests (3 new, about the trait)
dx serve                            # all three games still work
cargo run --example exp_traits_generics
```

## Files

```
src/modules/games/mod.rs          trait Game, fn describe, tests with a fake game
src/modules/games/*/mod.rs        each game: impl Game for ...
src/modules/base/hooks.rs         NEW: use_seeded<G>
src/routes/games/{guess,rps,tictactoe}.rs   shorter, use the shared pieces
examples/exp_traits_generics.rs   traits / generics / dyn lab
```

## Challenge

Write `struct Coin { heads: bool }` (a coin-flip "game"), implement `Game` for it in a test module, and call `describe(&Coin { heads: true })`. You touched no existing code: that is the point of a trait.

## Read next

`CHANGES.md` -> `KNOWLEDGE.md` -> [lesson 10](../rdx-10-snake-rules/README.md): the fourth game, and our first `Vec` / `VecDeque` collections.
