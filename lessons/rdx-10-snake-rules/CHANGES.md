# Lesson 10 - CHANGES

Previous: lesson 09 (three games, `Game` trait, `use_seeded`).

## Commands

```bash
cp -r rdx-09-traits-and-generics rdx-10-snake-rules
cd rdx-10-snake-rules
mkdir src/modules/games/snake
# add/edit files below, then
cargo clippy --all-targets
cargo test
dx serve
cargo run --example exp_collections
```

## Files

| File | Change |
|------|--------|
| `Cargo.toml` | name only |
| `src/config/mod.rs` | `LESSON_TAG = "lesson 10"` |
| `src/modules/games/snake/mod.rs` | **new**: the whole rules module + 13 tests |
| `src/modules/games/mod.rs` | `pub mod snake;` |
| `src/routes/games/snake.rs` | **new**: grid and buttons |
| `src/routes/games/mod.rs` | `pub mod snake;`, `"snake" => Some(Route::Snake {})` |
| `src/routes/mod.rs` | `Route::Snake` at `/games/snake`; `is_in_games` lists it |
| `assets/main.css` | snake board styles |
| `examples/exp_collections.rs` | **new** |

Dependencies: none added. All four catalogue cards now have a **Play** button.

## The shape of `SnakeGame`

```rust
pub struct SnakeGame {
    width: i32, height: i32,
    body: VecDeque<Pos>,          // front = head, back = tail
    direction: Direction,         // last step taken
    next_direction: Direction,    // what the player asked for
    food: Pos, score: u32, status: Status, rng: Rng,
}
```

Public surface:

```rust
SnakeGame::from_seed(seed)  /  SnakeGame::with_size(w, h, seed)
game.turn(Direction)  -> Result<(), SnakeError>
game.step()           -> Result<StepOutcome, SnakeError>
game.tile_at(Pos)     -> Tile        // what to draw
game.score() / status() / width() / height()
impl Game for SnakeGame
```

## `step()` in order

1. refuse if the game is over (`Err(GameOver)`)
2. `direction = next_direction`, compute `new_head`
3. outside the board? -> `Died(Wall)`
4. `eating = new_head == food`
5. collision with own body (ignoring the tail cell when we are only moving) -> `Died(SelfBite)`
6. `push_front(new_head)`; if not eating `pop_back()`
7. if eating: score+1, choose a new free food cell, or `Won` when none is left

## The page, in short

```rust
let (mut game, restart) = use_seeded(SnakeGame::from_seed);
let mut step = move || { if let Some(g) = game.write().as_mut() { let _ = g.step(); } };
// build rows of CSS classes from `tile_at`, then:
div { class: "snake-board", style: "--cols: {width}", for row in rows { for class in row { div { class } } } }
```

## Verify

1. `cargo test`: `43 passed`.
2. Browser: head starts mid-board; Step moves it right; Down + Step moves it down; repeated Step eventually shows "Finished. You hit the wall. Score: 0".
3. Break a rule: remove the `if eating { ... } else { pop_back }` distinction (always `pop_back`). Which tests fail?
4. Break another: use `self.body.len()` instead of `self.body.len() - 1` in the collision check. Which test fails? (It is the one named after the tail.)

## Compile notes

- The grid is built as `Vec<Vec<&'static str>>` and rendered with two nested `for` loops. `div { class }` is the field-init shorthand: the variable name equals the attribute name.
- clippy was silent on the first compile of this lesson.
