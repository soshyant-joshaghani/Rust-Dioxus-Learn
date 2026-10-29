# Lesson 05 - KNOWLEDGE (the book part)

Open `src/routes/games/guess.rs` and `examples/exp_closures.rs` next to this text.

---

## 1. State and why it needs a special tool

A normal Rust variable changes silently: the screen would not know. UI needs a way to say "this value matters to what I draw; tell me when it changes".

```rust
let mut tries = use_signal(|| 0u32);   // create state
rsx! { p { "Tries: {tries}" } }        // READ it in markup: this component subscribes
tries += 1;                             // WRITE it: every subscriber is drawn again
```

A **signal** is exactly that: a value + a subscriber list. The component function is called again (a "re-render"), reads the new value, produces a new `Element`, and Dioxus patches the screen with the difference.

Important consequences:

1. **A component function runs many times.** Local variables are recreated every time. Anything that must survive lives in a signal (or another hook).
2. `use_signal(|| initial)` takes a **closure** that runs only the first time; later renders return the existing signal.
3. **Hooks** (`use_signal`, `use_hook`, `use_future`...) must be called in the **same order, every render**: not inside `if`, loops or after an early `return`. Dioxus identifies them by call order.

### Reading and writing

| Code | Meaning |
|------|---------|
| `signal()` | copy the value out (needs `T: Copy`/Clone). Same as `*signal.read()`. |
| `signal.read()` | a read **guard** (a borrow); deref to use: `signal.read().len()` |
| `signal.set(v)` | replace the value |
| `signal += 1` | arithmetic assign for number signals |
| `signal.write()` | a write **guard** to change in place: `signal.write().push(x)` |
| `"{signal}"` in rsx | read + display |

`let mut x = use_signal(...)`: the `mut` is needed for methods that take `&mut self` (`set`, `write`, `+=`).

### Why signals are `Copy`

`Signal<T>` is a small handle (basically an id). The real `T` lives in Dioxus' storage. Copying the handle is cheap and **both copies point at the same data**. That's what lets us write `move |_| input.set(...)` in five different closures without `.clone()`.

## 2. Closures

A closure is an unnamed function that can use variables from where it is written.

```rust
let add_one = |x| x + 1;
let name = String::from("Fox");
let greet = || println!("hello {name}");     // captures `name` by borrow
```

### How a closure captures

| Capture | When the compiler picks it | Closure trait |
|---------|----------------------------|---------------|
| shared borrow `&T` | closure only reads | `Fn` |
| mutable borrow `&mut T` | closure changes a captured value | `FnMut` (closure variable must be `mut`) |
| move (ownership) | `move` keyword, or value is consumed | `FnOnce` if it moves a value out |

- `Fn` can be called many times and many at once. `FnMut` many times, one at a time. `FnOnce` once.
- **`move`** transfers ownership **into** the closure. For `Copy` values (numbers, signals) that means the closure gets its own copy.
- **A closure is `Copy` if everything it captures is `Copy`.** Our `new_game` captures only signals, so it is `Copy`, and we can pass it to `use_hook` *and* call it from the button's handler.

Why must event handlers be `move`? A handler is stored and called **later**, after the function that created it has returned. A borrow of a local variable would be dangling by then. `move` makes the closure independent of the function's stack frame. Try removing `move` from `oninput` and read the error ("closure may outlive the current function").

Run `cargo run --example exp_closures` and un-comment each ERROR.

## 3. `Result` and `match`: the parse

```rust
let parsed = input.read().trim().parse::<u32>();

match parsed {
    Err(_) => message.set(...),
    Ok(guess) if !(MIN..=MAX).contains(&guess) => message.set(...),   // guard
    Ok(guess) => { ... }
}
```

- `parse::<u32>()` returns `Result<u32, ParseIntError>`. Both outcomes are real; `match` forces us to write both.
- `u32` is **unsigned**: `"-5".parse::<u32>()` is an `Err`, so negative input is already rejected for free by the *type*.
- A **guard** (`if ...` after the pattern) refines an arm. Arms are tried in order, so the guarded `Ok(guess)` comes before the general `Ok(guess)`.
- `(MIN..=MAX)` is a **range** (inclusive of the end). `1..5` excludes 5. `.contains(&x)` borrows `x`.

### Why `parse::<u32>` and the turbofish?

`::<u32>` is called the *turbofish*. `parse` can produce many types, so the compiler needs to know which. Alternatively annotate the variable: `let n: Result<u32, _> = text.parse();`.

## 4. `Option` and `let ... else`

```rust
let Some(secret_number) = *secret.read() else {
    return;
};
```

"If `secret` holds a value, bind it to `secret_number`; otherwise run the `else` block, which must leave (return/break/continue/panic)." It keeps the happy path unindented, unlike nested `if let`.

We use `Option<u32>` for the secret because it **really is** unknown at first (the random seed arrives asynchronously). We could have used `0` as "not set", but then a bug could treat `0` as a real secret. `None` cannot be confused with a number.

`*secret.read()`: `.read()` returns a guard that derefs to `Option<u32>`; `*` copies it (Option<u32> is Copy). The guard is dropped at the end of the statement.

## 5. A borrow rule that is checked at RUN time

Rust's `&`/`&mut` rules are normally enforced at compile time. A signal's data lives in shared storage that many closures can reach, so the compiler cannot prove who is reading or writing. The rule moves to **runtime**:

```rust
let parsed = input.read().trim().parse::<u32>();   // guard created AND dropped in this statement
input.set(String::new());                           // fine: no guard alive
```

versus the trap:

```rust
match input.read().trim().parse::<u32>() {   // the guard (a temporary) lives for the WHOLE match
    Ok(_) => input.set(String::new()),       // PANIC: already borrowed
    ...
}
```

Temporaries in the scrutinee of a `match` live until the end of the match. So: **finish reading, keep only owned values, then write.** This is the same "many readers OR one writer" rule from lesson 02, enforced by a counter instead of the compiler. The std type behind this idea is `RefCell` (see `exp_borrowing`, lesson 08).

## 6. Async in one page

```rust
let new_game = move || {
    spawn(async move {
        let seed = random_seed().await;
        secret.set(Some(...));
    });
};
```

- `async fn` / `async { }` do not run immediately. They produce a **future**: a description of work that can be paused.
- `.await` means "pause here until the answer is ready; let others run meanwhile". It is only allowed inside async code.
- A component must return quickly, so we `spawn` a task: Dioxus runs it alongside the UI and the task writes the result into signals, which redraws the page.
- Why async at all? The JavaScript behind `random_seed` answers later. On the web there are no blocking waits: you must not freeze the single thread.
- Never hold a `.read()`/`.write()` guard across an `.await`: other code might need the signal while you wait. (The Dioxus `clippy.toml` from `dx new` enforces exactly that; we deleted the file, but remember the rule.)

`document::eval("...")` runs JavaScript. We use it for a tiny set of jobs (randomness here, timers later). It works on web, desktop and Android because all of them render with a browser engine.

## 7. Forms and events

```rust
form { onsubmit: on_submit,
    input { value: "{input}", oninput: move |event| input.set(event.value()) }
    button { r#type: "submit", "Guess" }
}
```

- `r#type`: `type` is a Rust keyword, so attributes with that name are written with the raw-identifier prefix `r#`.
- `event.value()` is the current text of the input; we store it in a signal and show it again with `value: "{input}"`. The signal is the **single source of truth**; the input is just a window onto it.
- `event.prevent_default()` stops the browser's own form submission (which would reload the page).
- Using a `<form>` gives us "press Enter to guess" for free.

## 8. `Ordering`, an enum from the standard library

`a.cmp(&b)` returns `Ordering::Less`, `Equal` or `Greater`. Three variants, exhaustive `match`: no way to forget the "equal" case. In many languages this would be an integer (-1/0/1) and you'd have to remember what it means.

## 9. Option-returning lookup: `route_for`

```rust
fn route_for(slug: &str) -> Option<Route> {
    match slug { "guess" => Some(Route::Guess {}), _ => None }
}
```

The catalogue lists games that do not exist yet. Returning `Option` makes "no page yet" a visible case in the code that draws the card. When you add the next game, you add one `match` arm and the "coming soon" disappears.

## 10. Ownership recap for this lesson

| Value | Who owns it |
|-------|-------------|
| signals' data | Dioxus' storage; we hold Copy handles |
| `parsed` | the handler (an owned `Result`), created from a short borrow of `input` |
| message `String` | moved into the signal by `.set()` |
| `format!(...)` result | a fresh `String`, moved into `set` |
| closures | own their captured (copied) signal handles because of `move` |

## 11. Experiments

1. In `on_submit`, use `match input.read().trim().parse::<u32>() { ... }` with `input.set(...)` in an arm, then play. Read the panic message. Restore the version with `let parsed`.
2. Remove `move` from `oninput`. Read the error.
3. Change `use_hook(new_game)` to run `new_game()` directly in the body of the component. What happens on every keystroke? Why?
4. Make `MAX` 1000. Does the "7 tries" claim still hold? (log2(1000) is about 10.)
5. Replace `u32` with `i32` in `parse` and `MIN` = -50. What changes for negative input?
6. Run `cargo run --example exp_closures`; for each example predict the output before running.

## 12. Key vocabulary

signal, re-render, hook, closure, capture, `move`, `Fn` / `FnMut` / `FnOnce`, `Result`, match guard, range, `Option`, `let ... else`, turbofish, `async`, `await`, future, `spawn`, read/write guard, `Ordering`.

## 13. Self-check

- Why do hooks have to run in the same order on every render?
- What is the difference between `Fn`, `FnMut` and `FnOnce`?
- Why is a signal `Copy` even though it "holds" a `String`?
- Why can't you call `input.set(...)` while a `input.read()` guard is alive?
- Why is the secret an `Option<u32>` and not a `u32`?
