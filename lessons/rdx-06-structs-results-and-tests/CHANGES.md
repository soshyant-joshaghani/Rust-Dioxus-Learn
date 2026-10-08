# Lesson 06 - CHANGES

Previous: lesson 05 (guessing game written inside the component).

## Commands

```bash
cp -r rdx-05-signals-and-closures rdx-06-structs-results-and-tests
cd rdx-06-structs-results-and-tests
mkdir src/modules/games/guess
# edit/add files below, then
cargo test
cargo clippy --all-targets
dx serve
cargo run --example exp_errors
```

Expected `cargo test`: `running 11 tests ... 11 passed`.

## Files

| File | Change |
|------|--------|
| `Cargo.toml` | name only |
| `src/config/mod.rs` | `LESSON_TAG = "lesson 06"` |
| `src/modules/base/random.rs` | **new content**: keeps `random_seed()`, adds `struct Rng` (`new`, `next_u64`, `range`) and 4 tests |
| `src/modules/games/guess/mod.rs` | **new**: `MIN`, `MAX`, `enum Hint`, `enum GuessError` (+ `Display`, `Error`, `From<ParseIntError>`), `struct GuessGame` (+ methods), 7 tests |
| `src/modules/games/mod.rs` | `pub mod guess;`; catalogue concept text "structs + Result" |
| `src/routes/games/guess.rs` | **rewritten**: one `Signal<Option<GuessGame>>` + handlers that call the rules |
| `examples/exp_errors.rs` | **new**: the same error handling written with and without `?` |

## The key before/after

Lesson 05 (inside the component):
```rust
match parsed {
    Err(_) => message.set(...),
    Ok(guess) if !(MIN..=MAX).contains(&guess) => message.set(...),
    Ok(guess) => { tries += 1; match guess.cmp(&secret_number) { ... } }
}
```

Lesson 06 (component):
```rust
let result = current.guess_text(&text);      // the rules decide
match result {
    Ok((hint, tries)) => message.set(describe(hint, tries)),
    Err(error)        => message.set(error.to_string()),
}
```

Lesson 06 (rules):
```rust
pub fn guess_text(&mut self, text: &str) -> Result<Hint, GuessError> {
    let number: u32 = text.trim().parse()?;   // ParseIntError -> GuessError via From
    self.guess(number)
}
```

## Signals changed

| Lesson 05 | Lesson 06 |
|-----------|-----------|
| `secret: Option<u32>`, `tries: u32`, `won: bool` | one `game: Option<GuessGame>` |
| `input`, `message` | `input`, `message` (unchanged) |

`tries` and `won` for the view are now *computed* from the game each render:
```rust
let won   = game.read().as_ref().is_some_and(|g| g.is_won());
let tries = game.read().as_ref().map_or(0, |g| g.tries());
```

## Verify

1. `cargo test`: 11 passed.
2. In the browser the game behaves as in lesson 05; messages come from `GuessError`'s `Display`: "That is not a whole number.", "500 is outside 1 to 100.", "You already won. Start a new game."
3. Break a rule on purpose: in `guess()`, remove the `self.tries += 1;` line. Which test fails? Read the failure message.
4. `cargo clippy --all-targets`: silent.
