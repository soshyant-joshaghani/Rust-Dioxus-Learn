# Lesson 06 - KNOWLEDGE (the book part)

Read with `src/modules/games/guess/mod.rs` open.

---

## 1. Why pull the rules out?

In lesson 05 the rules were inside the component. That worked, but:

- To test a rule you had to open a browser and type.
- The rules were mixed with drawing, so reading either was harder.
- Any other front-end (a terminal version, a bot) could not reuse them.

Now there are two layers:

```
 routes/games/guess.rs   "how it looks"   uses ->   modules/games/guess   "how it works"
```

The lower layer is plain Rust. It does not import Dioxus (apart from `Rng`'s seed function in `base`). This is the most reusable architectural habit in this course: **keep the rules free of the UI.**

## 2. Structs and `impl`

```rust
pub struct GuessGame {
    secret: u32,
    tries: u32,
    won: bool,
}

impl GuessGame {
    pub fn new(secret: u32) -> GuessGame { GuessGame { secret, tries: 0, won: false } }
    pub fn tries(&self) -> u32 { self.tries }
    pub fn guess(&mut self, number: u32) -> Result<Hint, GuessError> { ... }
}
```

- `struct` defines the data. `impl` (implementation) block defines functions attached to the type.
- **Associated function** (no `self`): `GuessGame::new(7)`. Rust has no constructors; by convention `new` plays that role.
- **Method** (first parameter is `self` in some form): `game.guess(7)`.

| First parameter | Meaning | Caller needs |
|-----------------|---------|--------------|
| `&self` | read the value | any access |
| `&mut self` | change the value | a `mut` variable / `&mut` reference |
| `self` | consume the value (it is moved in) | gives up ownership |

`game.guess(7)` is sugar for `GuessGame::guess(&mut game, 7)`. Rust inserts the `&mut` borrow for you.

### Field init shorthand

`GuessGame { secret, tries: 0, won: false }`: when a variable has the same name as the field you can write just the name.

### Privacy as a guarantee

All fields are private; only the methods change them. So **no outside code can create the state "won = true, tries = 0"**. The module protects an *invariant* (a promise that always holds). This is the real reason for `pub` vs private: not secrecy, but making impossible states impossible.

## 3. Errors: separating "outcome" from "mistake"

```rust
pub enum Hint { TooLow, TooHigh, Correct }                  // game outcomes
pub enum GuessError { NotANumber, OutOfRange(u32), GameOver } // mistakes
fn guess(&mut self, n: u32) -> Result<Hint, GuessError>
```

Why `Result<Hint, GuessError>` and not just `Hint`?

| If `guess` returned only `Hint` | With `Result` |
|---------------------------------|---------------|
| you would need `Hint::NotANumber`, `Hint::OutOfRange`... mixing two ideas | outcomes and mistakes have separate types |
| callers could `match` only the "nice" variants and forget the rest silently via `_` | the compiler forces `Ok` and `Err` handling |
| tries counting must remember to skip bad hints | bad guesses return early; counting happens only on the `Ok` path |

Rule of thumb: **return `Result` when the caller can reasonably cause or recover from the failure.** Use `assert!`/`panic!` for *bugs* (e.g. `Rng::range(5, 1)`: the programmer broke a precondition). `GuessError::OutOfRange` is the player's fault, so it is a `Result`.

### `Display`: errors for humans

```rust
impl fmt::Display for GuessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GuessError::NotANumber => write!(f, "That is not a whole number."),
            ...
        }
    }
}
```

`impl Trait for Type` is how a type promises behavior (traits: lesson 09). `Display` gives `{}` and `.to_string()`. `Debug` (derived) is for programmers: `{:?}`. Keep *wording* in `Display`, keep *data* in the enum; then the page can show text, a log can show data.

The `'_` in `Formatter<'_>` is an anonymous **lifetime**: "some borrow, I do not need to name it". Lesson 13 explains lifetimes.

### `From` and `?`

```rust
impl From<ParseIntError> for GuessError {
    fn from(_: ParseIntError) -> GuessError { GuessError::NotANumber }
}

let number: u32 = text.trim().parse()?;
```

`expr?` expands (roughly) to:

```rust
match expr {
    Ok(value) => value,
    Err(e)    => return Err(GuessError::from(e)),   // converts, then returns early
}
```

So: **`?` = unwrap on success, return the error on failure, converting with `From`.** It only works in a function that returns `Result` (or `Option`). `exp_errors.rs` shows the same function written with and without `?` side by side.

## 4. Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn low_high_and_correct() {
        let mut game = GuessGame::new(50);
        assert_eq!(game.guess(10), Ok(Hint::TooLow));
    }
}
```

- `#[cfg(test)]`: this module is compiled only by `cargo test`. It adds nothing to the app.
- `use super::*;` pulls in the parent module, **including private items**, which is how unit tests inspect internals (`a.secret`).
- `#[test]` marks a function as a test. It passes if it does not panic.
- `assert!(cond)`, `assert_eq!(left, right)`, `assert_ne!`. `assert_eq!` needs `Debug + PartialEq` on both sides: that is why `Hint` and `GuessError` derive them. Failure output shows both values.
- Tests run in parallel, in separate threads; keep them independent.
- Rust also supports `tests/` (integration tests) and doc tests (code in `///` comments is run!). We use only unit tests.

What to test: each rule, each error, each boundary (0, 1, 100, 101), and "nothing changed on error" (`tries` stays 0). Because `GuessGame::new(secret)` takes the secret, tests are deterministic. **Designing for testability = passing in the random part.** That is why the game takes a *seed*, and does not call JavaScript itself.

## 5. Our own random numbers (`Rng`)

Read `modules/base/random.rs`. New Rust you meet there:

| Code | Idea |
|------|------|
| `self.state = x;` inside `&mut self` | mutating through a mutable borrow |
| `x ^= x >> 12` | bit shifts and xor: scramble bits |
| `a.wrapping_mul(b)` | overflow wraps around instead of panicking (in debug builds normal `*` panics on overflow; in release it wraps silently: so say what you mean) |
| `(x % span) as u32` | `%` remainder; `as` converts number types and **truncates** |
| `u64::from(n)` | lossless conversion: never fails, so it uses `From` |
| `assert!(min <= max, "message")` | precondition check |

Choosing between conversions:

- `u64::from(x_u32)`: lossless, cannot fail.
- `x as u32`: may lose data silently; use when you have already proven the range.
- `u32::try_from(x_u64)`: returns `Result`, for "maybe too big".

## 6. Ownership and borrowing in the new component

```rust
let text = input();                      // clone of the String: we OWN a copy
let result = {
    let mut slot = game.write();         // WRITE GUARD = a mutable borrow of the signal's content
    let Some(current) = slot.as_mut() else { return; };   // Option<&mut GuessGame>
    let result = current.guess_text(&text);                // &mut GuessGame, &String -> &str
    let tries = current.tries();
    result.map(|hint| (hint, tries))
};                                       // guard dropped here: the mutable borrow ends
message.set(...)                         // fine: nobody is borrowing now
```

- `slot.as_mut()` turns `&mut Option<T>` into `Option<&mut T>`: a mutable borrow *of the inside*.
- `&text` is `&String`; the parameter is `&str`. Rust converts automatically (deref coercion).
- The braces `{ ... }` limit **how long the guard lives**. Scopes are a tool for shortening borrows.
- `result.map(|hint| (hint, tries))` transforms the `Ok` value and leaves `Err` alone. (`Result<Hint, E>` -> `Result<(Hint, u32), E>`). We read `tries` *inside* the block, because after the block the guard (and our right to look at the game) is gone.

## 7. Option helpers you used

| Code | Meaning |
|------|---------|
| `game.read().as_ref()` | `&Option<T>` -> `Option<&T>` (look, don't take) |
| `.is_some_and(\|g\| g.is_won())` | true if Some and the test passes |
| `.map_or(0, \|g\| g.tries())` | value if Some, else default `0` |

## 8. Experiments

1. `cargo test -- --nocapture` and add a `println!` to a test.
2. Break `guess`: change `<` to `<=` in the comparison. Which tests fail? What do they tell you?
3. Add a test for `guess_text("100")` and `guess_text("101")`.
4. Make `GuessGame::secret` public and change it from the page. The compiler allows it; what promise did you lose?
5. Replace `?` in `guess_text` by a `match` (see `area_long` in `exp_errors.rs`).
6. Implement `Hint::describe(self) -> &'static str` as a method and delete the free function `describe` from the page. Which is better? Why?

## 9. Key vocabulary

struct, `impl`, associated function, method, `self` / `&self` / `&mut self`, invariant, error enum, `Display`, `From`, `?`, unit test, `assert_eq!`, `cfg(test)`, deterministic, seed, pseudo-random.

## 10. Self-check

- When does a function return `Result` instead of panicking?
- What does `?` do, step by step?
- Why are the fields private?
- Why does `GuessGame::new` take the secret instead of generating it?
- What is the difference between `x as u32` and `u32::try_from(x)`?
