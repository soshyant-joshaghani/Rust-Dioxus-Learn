# Lesson 11 - Snake, part 2: the game loop

> Goal: make Snake **real time**. A timer calls `step()`, the keyboard calls `turn()`, Space pauses, and the game speeds up as you score. Concepts: async tasks and timing, `use_future`, keyboard events (an enum again), and why you never hold a signal guard across an `.await`.

The rules module from lesson 10 is **almost untouched**: one new function, `tick_ms()`. That is the payoff of keeping rules free of the screen.

## What you build

`/games/snake`:

- the board is focused automatically; press an **arrow key or WASD** to start,
- the snake moves every 150 ms (faster as the score grows, minimum 60 ms),
- **Space** (or the Start/Pause button) pauses,
- on-screen Up/Left/Right/Down buttons for phones and tablets,
- game over stops the clock.

## What you learn

| Rust / Dioxus | Project |
|---------------|---------|
| `async fn`, `.await`, why `thread::sleep` is wrong here | `sleep_ms` |
| `use_future`: a task tied to the component's life | the game loop |
| **never hold a guard across `.await`** | `let delay = game.read()...;` then `sleep_ms(delay).await` |
| `match` on a library enum with data (`Key`) | `direction_for_key` |
| `saturating_sub`, `max` | `tick_ms` |
| `let accepted = match ... ; if accepted { ... }` | keeping borrows short |
| threads, channels, `Arc<Mutex<T>>` (native only) | `examples/exp_threads_channels.rs` |

## Run

```bash
cd lessons/rdx-11-snake-loop
cargo test                          # 46 tests
dx serve                            # /games/snake  (click nothing: just press an arrow key)
dx serve --platform desktop         # same game in a native window
cargo run --example exp_threads_channels
```

## Files

```
src/modules/base/time.rs         sleep_ms(ms) via JavaScript setTimeout
src/modules/games/snake/mod.rs   + tick_ms(), + test
src/routes/games/snake.rs        loop, keyboard, pause, d-pad (+2 tests)
assets/main.css                  .dpad, board focus outline
examples/exp_threads_channels.rs threads / mpsc / Arc<Mutex>
```

## Challenge

Add a **best score** line (kept only while the page is open): a `best: Signal<u32>` updated when a game ends. In lesson 12 you will make it survive a restart of the app.

## Read next

`CHANGES.md` -> `KNOWLEDGE.md` -> [lesson 12](../rdx-12-persistence/README.md): saving progress with `std::fs`.
