# Lesson 10 - KNOWLEDGE (the book part)

Read with `src/modules/games/snake/mod.rs` and `examples/exp_collections.rs`.

---

## 1. Think in state and transitions

A game is a **state machine**: a value that is in exactly one state at a time, plus rules that move it to the next state.

```
            step()
  Running ----------> Running            (moved / ate)
  Running ----------> Dead(Wall)         (final)
  Running ----------> Dead(SelfBite)     (final)
  Running ----------> Won                (final)
```

In Rust the states are an **enum**, the transition is a **method**:

```rust
pub enum Status { Running, Dead(Cause), Won }
pub fn step(&mut self) -> Result<StepOutcome, SnakeError>
```

Because `Status` is an enum, "alive and dead at once" cannot exist. Because `step` is the only function that changes it, every change happens in one readable place.

### Separate the rules from time

`step()` knows nothing about a timer. Tests call it in a loop:

```rust
g.step()?; g.step()?;     // two ticks, instantly
```

A timer-driven game is hard to test; a step-driven one is trivial. The **timer is just one more caller of `step()`** (lesson 11).

## 2. Outcome vs error (again, because it is important)

| | Type | Example |
|---|------|---------|
| Normal result of a valid action | enum variant | `StepOutcome::Died(Cause::Wall)` (dying is part of the game) |
| Caller used the API wrongly | `Err(...)` | `Err(SnakeError::GameOver)` (stepping a finished game) |
| Impossible by construction | `assert!` / `expect` | `with_size(1, 0, ..)`: the programmer passed nonsense |

The page `let _ = g.step();`: `let _ =` explicitly throws a `Result` away. Rust warns about *unused* Results (`#[must_use]`); writing `let _` is how you say "yes, I looked at it and ignoring it is correct here". A stray timer tick after death is exactly that case.

## 3. `i32` or `usize` for coordinates?

`usize` looks natural for grid positions (it is what arrays are indexed with), but:

```rust
let x: usize = 0;
let left = x - 1;      // debug build: PANIC "attempt to subtract with overflow"
                       // release build: wraps to 18446744073709551615
```

With `i32` the "cell left of the wall" is `-1`, and `inside()` can answer honestly:

```rust
(0..self.width).contains(&pos.x) && (0..self.height).contains(&pos.y)
```

**Choose the number type that can represent every value your rules talk about**, including the invalid ones you need to reject. Convert to `usize` only at the edge where you index an array.

(`free.len() as u32 - 1`: `len()` is `usize`; `Rng::range` takes `u32`. We use `as` here because a 16x12 board is nowhere near `u32::MAX`; a library would use `u32::try_from(len)`.)

## 4. `VecDeque`: the right collection for a snake

| Operation | `Vec<T>` | `VecDeque<T>` |
|-----------|----------|---------------|
| push at back | fast | fast |
| pop at back | fast | fast |
| push at front | **slow** (shifts everything) | fast |
| pop at front | **slow** | fast |
| index `v[i]` | fast | fast |
| contiguous slice | yes | no (it is a ring buffer) |

A step does `push_front(new_head)` and `pop_back()`: exactly the two things `VecDeque` is good at. Look at `body[0]` for the head, `body.iter()` from head to tail, `body.contains(&pos)`.

### Why `Pos: PartialEq`?

`contains`, `==`, `position` and `assert_eq!` all need `PartialEq`. A `derive` costs nothing and removes a whole class of hand-written comparisons.

## 5. Iterators in the collision check

```rust
let blocking = if eating { self.body.len() } else { self.body.len() - 1 };
if self.body.iter().take(blocking).any(|cell| *cell == new_head) { ... }
```

- `iter()` walks head -> tail.
- `take(blocking)` stops after that many cells. When we only move, the **last** cell (the tail) will be vacated this very step, so it must not count as an obstacle. That is the "following your own tail is allowed" rule, expressed as "skip the last cell".
- `any(|cell| ...)` returns `true` at the first match (and stops).

When growing (`eating`), the tail stays, so *all* cells block.

This is the kind of rule people get wrong in snake implementations; it has its own test (`following_your_own_tail_is_allowed`), written by drawing the situation as comments.

## 6. Choosing a random free cell safely

```rust
let mut free: Vec<Pos> = Vec::new();
for y in 0..self.height { for x in 0..self.width { if !self.body.contains(&Pos{x, y}) { free.push(...) } } }
```

Two approaches:

| Approach | Problem |
|----------|---------|
| draw random cells until one is free | on an almost full board it may loop a very long time (in theory, forever) |
| list free cells, pick one by index | always finishes; tells you when the board is **full** (`free.is_empty()` -> `Won`) |

We take the second. It costs O(width x height x body) per meal: a few thousand operations, irrelevant at this size. **Prefer the obviously-correct version until measurement says otherwise.**

`Option<Pos>` is the return type because "no free cell" is a legitimate answer, not an error.

## 7. Direction handling: two fields, one rule

```rust
direction:      Direction,   // the direction of the last step actually taken
next_direction: Direction,   // what the player asked for since then
```

`turn` compares the request with `direction` (not with `next_direction`). Why? Imagine moving Right and pressing Up then Left very quickly between two ticks:

- compare with `next_direction` (Up): Left is not the opposite of Up, accepted; the next step goes Left, straight into the neck. **Bug.**
- compare with `direction` (Right): Left is the opposite, refused. Correct.

The length-1 exception (`self.body.len() > 1`) lets a one-cell snake turn around freely; there is no neck to bite.

## 8. Testing technique: build the situation by hand

Tests live in the same module, so they can write private fields:

```rust
g.body = VecDeque::from(vec![Pos { x: 4, y: 4 }, Pos { x: 4, y: 5 }, ...]);
g.food = Pos { x: 0, y: 0 };
```

Instead of playing 40 moves to curl the snake, we **set up the state directly**, then perform the one step under test. The `curled_snake` helper has a picture in its doc comment: a test whose situation you cannot see is a test you cannot trust.

Other techniques used:

- a tiny board (`2x1`) so that "win" needs one step
- loop over many seeds for a property (`food_never_spawns_on_the_snake`)
- deterministic randomness: same seed, same food (`same_seed_same_first_food`)

## 9. `Tile`: a boundary type

The page needs to know "what do I draw here?", not "how is the snake stored?". `tile_at(Pos) -> Tile` answers with a four-variant enum. If we later change `body` to a `Vec` or a grid, the page does not change. **Return answers, not internals.**

## 10. Ownership in the page

| Code | What happens |
|------|--------------|
| `game.read().as_ref()` | a read guard; `as_ref()` makes `Option<&SnakeGame>` |
| inside the `match`: `g.tile_at(...)` | shared borrow of the game |
| `rows: Vec<Vec<&'static str>>` | OWNED vectors of `'static` strs: no borrow of the game survives |
| `describe(g)` | returns a fresh `String` |
| leaving the `match` | the guard is dropped; drawing proceeds with owned data only |

## 11. Experiments

1. Un-comment nothing; run `cargo run --example exp_collections` and predict each line first.
2. Change `DEFAULT_WIDTH` to 30. Does the page still look right? (It reads `width()` and sets `--cols`.)
3. Implement the **wrap-around** challenge. Which tests break, and which keep passing?
4. In `random_free_cell`, replace `Vec` by building a `HashSet<Pos>` of the body for `contains`. Needs `#[derive(Hash)]` on `Pos`. Is it worth it for a 192-cell board?
5. Make the snake start with length 3. Which tests need an update? What does that tell you about test setup helpers?
6. Add `StepOutcome::Won` handling to the page's message (it is already in `summary()`): play a 2x1 board by editing `with_size` temporarily.

## 12. Key vocabulary

state machine, transition, `VecDeque`, ring buffer, push/pop front/back, iterator adaptor (`take`, `any`), `collect`, `HashMap`, `HashSet`, `must_use`, `let _ =`, boundary type, deterministic test, property, invariant.

## 13. Self-check

- Why is `i32` a better coordinate type than `usize` here?
- Why does the collision check skip the last cell when the snake is only moving?
- Which operations make `VecDeque` the right choice?
- Why does `turn` compare with `direction` and not `next_direction`?
- When is dying an `Ok(...)` and when is stepping an `Err(...)`?
