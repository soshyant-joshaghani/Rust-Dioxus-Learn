# Lesson 06 - Structs, Results and tests

> Goal: the guessing game plays exactly as before, but its **rules move out of the component** into a plain Rust module with its own types, its own errors and its own tests. Central ideas: `struct` + `impl`, `Result<T, E>` with a custom error enum, and `cargo test`.

## What changes

```
lesson 05                         lesson 06
routes/games/guess.rs             routes/games/guess.rs          draws + forwards input only
  (rules + state + view)          modules/games/guess/mod.rs     GuessGame, Hint, GuessError + 7 tests
                                  modules/base/random.rs         + Rng (our own generator) + 4 tests
```

## What you learn

| Rust | Project |
|------|---------|
| `struct`, `impl`, associated functions (`new`) | `GuessGame { secret, tries, won }` |
| methods: `&self`, `&mut self`, `self` | `tries()`, `guess()` |
| private fields protect rules (invariants) | the page cannot cheat |
| custom error `enum` + `Display` + `From` | `GuessError` |
| the `?` operator | `guess_text` |
| `#[cfg(test)]`, `#[test]`, `assert_eq!`, `cargo test` | 11 tests, no browser needed |
| integer math: `wrapping_mul`, shifts, `as` | our `Rng` |

## Run

```bash
cd lessons/rdx-06-structs-results-and-tests
cargo test                        # rules tested without any UI
dx serve                          # play: behaves like lesson 05
cargo run --example exp_errors    # error-type lab
```

## Challenge

Add `GuessError::TooManyTries` after 7 valid guesses. First write the **test** that expects it and watch it fail, then make it pass. (That is test-driven development, in three lines.)

## Read next

`CHANGES.md` -> `KNOWLEDGE.md` -> [lesson 07](../rdx-07-rock-paper-scissors/README.md).
