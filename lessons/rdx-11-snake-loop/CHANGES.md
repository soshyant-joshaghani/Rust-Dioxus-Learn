# Lesson 11 - CHANGES

Previous: lesson 10 (Snake rules, stepped by a button).

## Commands

```bash
cp -r rdx-10-snake-rules rdx-11-snake-loop
cd rdx-11-snake-loop
# add/edit files below, then
cargo clippy --all-targets
cargo test
dx serve
cargo run --example exp_threads_channels
```

## Files

| File | Change |
|------|--------|
| `Cargo.toml` | name only |
| `src/config/mod.rs` | `LESSON_TAG = "lesson 11"` |
| `src/modules/base/time.rs` | **new**: `pub async fn sleep_ms(ms: u32)` |
| `src/modules/base/mod.rs` | `pub mod time;` |
| `src/modules/games/snake/mod.rs` | `tick_ms()`, 3 constants (`TICK_START_MS`, `TICK_STEP_MS`, `TICK_MIN_MS`), 1 test |
| `src/routes/games/snake.rs` | **rewritten**: `use_future` loop, `onkeydown`, `running` signal, d-pad; 2 tests |
| `assets/main.css` | `.snake-board:focus`, `.dpad` grid |
| `examples/exp_threads_channels.rs` | **new** |

Still **no** dependencies besides `dioxus`.

## `sleep_ms`

```rust
pub async fn sleep_ms(ms: u32) {
    let script = format!("await new Promise(resolve => setTimeout(resolve, {ms})); return true;");
    let _ = document::eval(&script).join::<bool>().await;
}
```

## `tick_ms` (rules)

```rust
pub fn tick_ms(&self) -> u32 {
    TICK_START_MS.saturating_sub(self.score * TICK_STEP_MS).max(TICK_MIN_MS)
}
```
150 ms at score 0, 5 ms faster per point, clamped to 60 ms.

## The loop (page)

```rust
use_future(move || async move {
    loop {
        let delay = game.read().as_ref().map_or(150, |g| g.tick_ms()); // guard dropped here
        sleep_ms(delay).await;                                        // nothing borrowed while waiting
        if !running() { continue; }
        let finished = {
            let mut slot = game.write();
            match slot.as_mut() { Some(g) => { let _ = g.step(); g.is_over() } None => false }
        };
        if finished { running.set(false); }
    }
});
```

## The keyboard (page)

```rust
div { class: "snake-board", tabindex: "0", onkeydown: on_key,
      onmounted: move |event| async move { let _ = event.set_focus(true).await; }, ... }
```
`on_key` calls `prevent_default()` (so arrows/space do not scroll the page), maps the key with `direction_for_key`, or toggles `running` for Space.

## Differences from lesson 10 in the UI

| Lesson 10 | Lesson 11 |
|-----------|-----------|
| `Step` button | timer loop |
| Left/Up/Down/Right in a row | keyboard + a cross-shaped d-pad |
| no pause | Space / Start / Pause button |
| fixed speed | speeds up with score |

## Build problems met

- `clippy::collapsible_if` on `if let Some(g) = game.write().as_mut() { if g.turn(direction).is_ok() { running.set(true) } }`. Rewritten as `let accepted = match ... ; if accepted { ... }`, which also drops the write guard **before** `running.set`.
- `clippy::let_and_return` in the thread experiment (`let answer = 6*7; answer`).

## Verify (all six items were run while writing the lesson: 1-4 and 6 in a real browser, 5 in the terminal)

1. The board has focus on load; the snake does **not** move until a key is pressed.
2. ArrowDown starts it; it advances about one cell per 150 ms (three cells in about 450 ms).
3. Space freezes it (position unchanged after 500 ms); Space again resumes.
4. It runs into the wall and shows `Finished. You hit the wall. Score: 0`; the clock stops.
5. `cargo test`: 46 passed. `cargo clippy --all-targets`: silent.
6. Navigate to `/about` and back with the nav links: the snake still moves about one cell per 150 ms (about 3 cells in 460 ms), so the old loop was cancelled and did not stack up.
