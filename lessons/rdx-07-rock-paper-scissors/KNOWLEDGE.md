# Lesson 07 - KNOWLEDGE (the book part)

Read with `src/modules/games/rps/mod.rs` and `examples/exp_copy_clone.rs` open.

---

## 1. Model the problem with types

Before writing any logic we wrote down the *vocabulary*:

```rust
enum Move    { Rock, Paper, Scissors }
enum Outcome { Win, Lose, Draw }
struct Round { player: Move, computer: Move, outcome: Outcome }
```

Compare with strings or integers (`"rock"`, `0`, `1`): a typo (`"rok"`) or an impossible number (`7`) compiles fine and fails later. With enums the invalid values **cannot be written**. Good Rust starts with the types; the functions often almost write themselves.

## 2. `match` on a tuple

```rust
match (player, computer) {
    (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Outcome::Draw,
    (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Outcome::Win,
    (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Outcome::Lose,
}
```

- `(a, b)` builds a tuple: two values bundled temporarily. Patterns can take it apart in the same shape.
- `|` means "or" between patterns. All alternatives yield the same result.
- 3 x 3 = 9 combinations are covered: the compiler checks. Delete one pattern and it names the missing case.
- No `_` wildcard on purpose. A wildcard would silently swallow future moves (say you add `Lizard`) and give wrong answers instead of a compile error. **Prefer explicit patterns for domain enums; use `_` for "open-ended" things like text.**

`use Move::{Paper, Rock, Scissors};` inside the function brings the variant names into scope so you do not repeat `Move::`. A `use` can appear in any block.

## 3. Methods and constants on enums

```rust
impl Move {
    pub const ALL: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];
    pub fn label(self) -> &'static str { match self { ... } }
}
```

- An **associated constant** (`Move::ALL`) belongs to the type. It is the usual way to get "all the variants" (Rust has no built-in `Move::values()`).
- Used twice: to draw the buttons (`for mv in Move::ALL`) and to choose the computer's move by index.
- `fn label(self)` takes `self` by value: allowed because `Move` is `Copy`.

## 4. Copy, Clone, move: the decision

| Question | Answer | Result |
|----------|--------|--------|
| Is the value tiny and made only of Copy data? | yes | `derive(Clone, Copy)`: pass freely |
| Does it own heap memory (`String`, `Vec`) or has identity/state? | yes | not `Copy`; moves by default; `Clone` if you need explicit duplicates |
| Would two copies be wrong? | yes (a random generator!) | not even `Clone` unless duplicating makes sense |

In this lesson:

| Type | Traits | Why |
|------|--------|-----|
| `Move`, `Outcome` | `Clone, Copy` | one byte, no meaning to "move" it |
| `Round`, `Score` | `Clone, Copy` | only Copy fields; handing one to the page leaves ours intact |
| `RpsGame` | `Clone` | holds an `Rng`; duplicating is possible but must be deliberate |

`derive(Copy)` only compiles when **all fields are Copy**. Put a `String` in `Round` and the compiler rejects `Copy`. See `exp_copy_clone.rs`.

### Why this matters in the page

```rust
pub fn score(&self) -> Score { self.score }          // returns a COPY
pub fn last_round(&self) -> Option<Round> { self.last } // Option<Copy> is Copy
```

The page reads these through a signal guard. Because the results are copies, the guard can be dropped immediately; the page never holds a borrow of the game while drawing. If `Score` were not `Copy` we would return `&Score` or clone it, and the guard would have to stay alive.

## 5. `derive(Default)`

```rust
#[derive(Default)]
struct Score { wins: u32, losses: u32, draws: u32 }
let s = Score::default();   // all zeros
```

`Default` builds the "empty" value; numbers default to 0, `String` to `""`, `Option` to `None`, `Vec` to empty. In the page, `None => Default::default()` creates a `(Score, Option<Round>)` pair of defaults in one word: the compiler infers the target type from the other match arm.

## 6. A closure that is `FnMut` and `Copy`

```rust
let mut play = move |player: Move| {
    if let Some(g) = game.write().as_mut() { g.play(player); }
};
...
for mv in Move::ALL {
    button { onclick: move |_| play(mv), "{mv}" }
}
```

1. `game.write()` needs `&mut` access to the signal handle, so `play` is `FnMut` and must be declared `let mut`. (Lesson 05's `new_game` never mutated its captured handles directly, so it did not need `mut`.)
2. `play` captures only a `Signal`, which is `Copy`, so `play` is `Copy`. Each button's closure copies `play`. We never write `.clone()`.
3. `mv` is also `Copy`. The `move` keyword gives each button its **own** `mv`. Without `move`, the closure would borrow the loop variable, which dies at the end of each iteration: compile error "may outlive borrowed value".

## 6b. Temporaries and guards, again

`if let Some(g) = game.write().as_mut() { ... }`: the temporary `game.write()` lives for the entire `if let` body. It is fine here because the body does one call and touches no other signal. If the body also wrote `message.set(...)` for a signal **that the same guard protects**, you would panic. Lesson 06's explicit `{ ... }` block is the safe style when the body is longer.

## 7. Arrays, indexes and `usize`

```rust
let index = self.rng.range(0, 2) as usize;
let computer = Move::ALL[index];
```

- Arrays are indexed with `usize` (the size of a pointer). Hence `as usize`.
- `Move::ALL[7]` would **panic** at run time ("index out of bounds"). Rust never reads memory past the end. Here the index is 0..=2 by construction. When an index comes from outside, use `.get(i)`, which returns `Option<&T>`.
- `Move::ALL.iter().position(|m| *m == x)` returns `Option<usize>`: the index of the first match.

## 8. Testing tables

```rust
let table = [(Rock, Rock, Draw), (Rock, Paper, Lose), ...];
for (player, computer, expected) in table {
    assert_eq!(judge(player, computer), expected, "{player} vs {computer}");
}
```

- `for (a, b, c) in table`: the loop pattern destructures each tuple.
- The third argument of `assert_eq!` is a message shown on failure. Always add one inside loops, or you will not know which row failed.
- `judging_is_symmetric` checks a *property* ("if I win, you lose") rather than examples. Properties catch bugs examples forget.

## 9. Pure functions

`judge` reads its inputs and returns an output: nothing else. No `&mut`, no signals, no randomness. Pure functions are the easiest code to test, reason about, and reuse. Wherever a rule can be a pure function, make it one, and keep the state-changing code (`RpsGame::play`) thin.

## 10. Experiments

1. `cargo run --example exp_copy_clone`, then un-comment each ERROR.
2. Remove `Copy` from `Move`'s derive. Count the compiler errors and read two. Where do you now need `.clone()` or `&`?
3. Add `Lizard`. Which `match`es fail? Is it better to extend `judge` or to rewrite it with a "beats" table? (See the challenge.)
4. Replace `Move::ALL[index]` by `Move::ALL[index + 1]` and play until it crashes. Read the panic message.
5. Make the computer cheat: `if player == Rock { Paper }`. Which test would you write to catch this?
6. Remove `mut` from `let mut play`. Read the message.

## 11. Key vocabulary

tuple, tuple pattern, or-pattern `|`, wildcard, associated constant, pure function, `Copy`, `Clone`, `Default`, `FnMut`, loop variable capture, `usize`, property test.

## 12. Self-check

- Why does `derive(Copy)` fail for a struct with a `String` field?
- Why do we avoid `_` in `judge`?
- Why must `play` be `let mut` while `new_game` did not need it?
- What does `Move::ALL[7]` do, and what is the safe alternative?
