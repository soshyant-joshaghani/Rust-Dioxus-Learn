# Lesson 11 - KNOWLEDGE (the book part)

Read with `src/routes/games/snake.rs`, `src/modules/base/time.rs` and `examples/exp_threads_channels.rs`.

---

## 1. What a game loop is

Every real-time game repeats three steps forever:

```
   +--> 1. read input ----> 2. update the world (one tick) ----> 3. draw --+
   |                                                                       |
   +------------------------------ wait ----------------------------------+
```

In Dioxus you do not write the "draw" step: changing a signal makes it happen. So our loop is: **wait, then update**. Input arrives separately through event handlers that change state (`turn`).

Two clocks exist and must not be confused:

| Clock | Who runs it | Our use |
|-------|-------------|---------|
| game clock | our `use_future` loop | `step()` every 150 ms |
| screen clock | the browser (~60 times/s) | redraw when signals change |

Snake steps at 150 ms = about 7 times per second; that is a **game rule** ("one cell per tick"), not a drawing speed.

## 2. Why not `std::thread::sleep`?

`thread::sleep(150ms)` stops the current thread. On the web:

- the page runs on **one** thread that also draws and handles clicks, so sleeping freezes the tab;
- `std::thread::spawn` is not available in normal WebAssembly.

Even on desktop, sleeping inside a UI function would freeze the window. The cure is **async**: wait *without* holding the thread.

## 3. Async, one level deeper

```rust
pub async fn sleep_ms(ms: u32) { ... eval(...).join::<bool>().await ... }
```

- An `async fn` returns a **future**: a value describing work that can be paused and resumed. Nothing runs until something drives it.
- `.await` pauses *this future* and hands control back to the **executor** (here: Dioxus' runtime). When the awaited thing is ready, the executor resumes the future where it stopped.
- Rust compiles `async` code into a **state machine**: each `.await` is a state boundary. No thread per task, which is why you can have thousands.
- `async` is *cooperative*: a task that never awaits (an infinite plain `loop {}`) blocks everyone else. Our loop awaits every tick, so the UI stays responsive.

`document::eval(...)` runs JavaScript, and `.join::<bool>()` waits for its `return`. The browser's `setTimeout` is the actual timer.

## 4. `use_future`: a task owned by the component

```rust
use_future(move || async move { loop { ... } });
```

- starts when the component first renders;
- is **cancelled** (dropped at its next `.await`) when the component is removed, e.g. when you leave `/games/snake`;
- runs once per component instance, not per render.

Compare with `spawn` (lesson 05): `spawn` starts a one-off task inside an event handler; `use_future` ties a long-lived task to the component's lifetime so you cannot leak a loop. Without that, navigating away and back would leave two loops stepping the same game at double speed.

## 5. Golden rule: never hold a guard across an `.await`

```rust
// WRONG
let guard = game.read();
sleep_ms(150).await;        // while we sleep, `guard` still blocks every writer
let x = guard...;

// RIGHT: copy what you need out, drop the guard, THEN await
let delay = game.read().as_ref().map_or(150, |g| g.tick_ms());
sleep_ms(delay).await;
```

Why it matters: while the future is paused, other code (a key press!) runs. If it tries to `game.write()` while our read guard is alive, you get the "already borrowed" panic from lesson 05. And since the pause is 150 ms, it happens on the **first** key press. (Checked while writing this lesson: the page dies with `called Result::unwrap() on an Err value: AlreadyBorrowed(...)`.)

Look at how the loop body is shaped: every `.read()`/`.write()` is either a single-statement temporary or inside a `{ ... }` block that ends before the next `.await`. Dioxus even ships a clippy lint for it (`await-holding-invalid-types`; the file `clippy.toml` from `dx new`).

## 6. Keyboard input

```rust
div { tabindex: "0", onkeydown: on_key, onmounted: ... }
```

- A `<div>` receives key events only when it has **focus**; `tabindex="0"` makes it focusable and `set_focus(true)` focuses it immediately. A click on the board refocuses it.
- `event.key()` returns a `Key` **enum**:

```rust
match key {
    Key::ArrowUp => Some(Direction::Up),
    Key::Character(letter) if letter.eq_ignore_ascii_case("w") => Some(Direction::Up),
    _ => None,
}
```

Enums with data, guards and `Option`: the same toolbox as lessons 04-07, applied to a library type. `direction_for_key` is a **pure function**, so it has unit tests (`Key::ArrowLeft` is just a value you can construct).

- `event.prevent_default()` cancels the browser's default action (scrolling on arrows/space).
- Touch devices have no keyboard, so the same `turn()` is wired to buttons. **One action, several input devices**: the handlers differ, the function they call is the same.

## 7. The `running` flag: a second small state machine

```
 idle --first valid key--> running --Space--> paused --Space--> running
                              \--game over--> idle (stopped)
```

It is a plain `bool` signal. We could model it as an enum (`Idle | Running | Paused`), but the three states are never treated differently by code, so a `bool` is enough. **Pick the simplest type that represents all the states you actually distinguish.** (Compare `Status` in the rules, where each state has different data.)

`running.set(!running())`: `running()` reads (copies) the bool, `!` negates, `set` writes.

## 8. Speed as a rule

```rust
TICK_START_MS.saturating_sub(self.score * TICK_STEP_MS).max(TICK_MIN_MS)
```

- `saturating_sub` clamps at 0 instead of wrapping to `4294967295` (unsigned underflow). The test with `score = 1_000` proves it.
- `.max(TICK_MIN_MS)` sets the floor.
- This is in the **rules**, not the page: "how fast is the game" is game design, and a test can check it without a browser.

## 9. Threads and channels (the experiment)

`examples/exp_threads_channels.rs` builds the same "tick every 30 ms" idea with the **native** tools: `thread::spawn`, `mpsc::channel`, `thread::sleep`.

| Concept | One-line meaning |
|---------|------------------|
| `thread::spawn(move \|\| ...)` | run a closure on a new OS thread; `move` hands it its data |
| `handle.join()` | wait; returns `Result` (Err = the thread panicked) |
| `mpsc::channel()` | `(Sender, Receiver)`: send **moves** values from thread to thread |
| `Arc<T>` | shared ownership across threads (atomic reference counting) |
| `Mutex<T>` | one thread at a time may touch the data; `lock()` returns a guard |
| `Send` / `Sync` | marker traits the compiler checks: "may move to / be shared with another thread" |

Rust's promise: **a data race cannot compile**. Try the three commented errors at the bottom of the file. `Rc` (non-atomic counting) is rejected for threads: use `Arc`.

Notice the symmetry with this lesson:

| Thread world | Our async world |
|--------------|-----------------|
| `Mutex::lock()` guard | signal `write()` guard |
| channel `send` | `signal.set(...)` |
| `thread::sleep` | `sleep_ms(..).await` |
| runtime panics if misused? no: lock waits | runtime panic if two guards clash |

## 10. Experiments

1. Change `TICK_STEP_MS` to 20. At what score does the floor stop the speedup?
2. Remove `event.prevent_default()` in `on_key` and press the arrows with a short window. Why does the page scroll?
3. Hold the guard across the await on purpose: write `let guard = game.read();` before `sleep_ms(delay).await` and press a key while it waits. Read the panic text.
4. Remove `tabindex: "0"`. Do the keys still work? Click the board; now?
5. Replace `use_future` by `use_hook(|| spawn(async move { loop { ... } }))`. Navigate away and back: does the old loop stop? (Compare the speed of the snake.)
6. Run `cargo run --example exp_threads_channels` and un-comment each error.

## 11. Key vocabulary

game loop, tick, future, `async` / `.await`, executor, cooperative scheduling, `use_future`, cancellation, focus, `tabindex`, `prevent_default`, `Key`, `saturating_sub`, thread, channel, `Arc`, `Mutex`, `Send`, data race.

## 12. Self-check

- Why can't you use `std::thread::sleep` in a web game loop?
- What happens to the `use_future` task when you leave the page?
- Which line of the loop would panic if you held `game.read()` across `sleep_ms`?
- Why is the speed rule in the rules module and not in the page?
- What does `saturating_sub` protect against?
