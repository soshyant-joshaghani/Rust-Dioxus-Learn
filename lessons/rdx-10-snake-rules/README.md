# Lesson 10 - Snake, part 1: the rules

> Goal: build the **brain** of Snake as a plain Rust state machine and prove it with tests, *before* any timer or keyboard exists. Collections (`VecDeque`, `Vec`) and iterators are the star.

The page for this lesson is deliberately primitive: arrow **buttons** and a **Step** button. You advance the game by hand, one tick at a time. In lesson 11 a timer calls `step()` for you.

## What you build

`/games/snake`: a 16x12 grid. The green head is where the snake is, orange is food. Turn with Left/Up/Down/Right, press Step to move one cell. Eating food grows the snake and increases the score; hitting a wall or your own body ends the game.

## What you learn

| Rust | Project |
|------|---------|
| `struct Pos { x, y }` and why `i32` | coordinates, "off the board" is just `x = -1` |
| `VecDeque<T>`: push/pop at both ends | the snake's body |
| `Vec<T>`, `collect`, `contains`, `iter().take(n).any(...)` | free cells, collision check |
| a **state machine** as enums (`Status`, `StepOutcome`) | `Running -> Dead(cause) / Won` |
| `Result` for caller mistakes vs enum for normal outcomes | `step() -> Result<StepOutcome, SnakeError>` |
| testing time-dependent logic **without time** | 13 tests drive `step()` in a loop |
| `let _ = result;` (consciously ignoring a Result) | the page |
| generic reuse | `use_seeded(SnakeGame::from_seed)`, `impl Game for SnakeGame` |

## Run

```bash
cd lessons/rdx-10-snake-rules
cargo test                          # 43 tests (13 new)
dx serve                            # /games/snake
cargo run --example exp_collections
```

## Files

```
src/modules/games/snake/mod.rs   Pos, Direction, Status, StepOutcome, SnakeError, Tile, SnakeGame (+13 tests)
src/routes/games/snake.rs        grid + buttons
src/routes/mod.rs, games/mod.rs  Route::Snake, route_for
assets/main.css                  .snake-board, .tile(.head/.body/.food)
examples/exp_collections.rs      Vec / VecDeque / HashMap / HashSet / iterator lab
```

## Challenge

Make walls **wrap around** (leaving on the right re-enters on the left). Change only `step()` and one helper, update the wall test, and notice the page needs **no** change at all.

## Read next

`CHANGES.md` -> `KNOWLEDGE.md` -> [lesson 11](../rdx-11-snake-loop/README.md): timing, keyboard input and the game loop.
