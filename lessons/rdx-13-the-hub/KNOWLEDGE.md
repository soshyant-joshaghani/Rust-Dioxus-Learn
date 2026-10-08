# Lesson 13 - KNOWLEDGE (the book part)

---

## 1. The whole architecture in one picture

```
 main.rs ──► app.rs  (providers: progress, theme  +  Router)
                │
                ▼
          routes/mod.rs  (enum Route)
                │  AppLayout: Header(nav + ThemeToggle) · Outlet · Footer
   ┌────────────┼──────────────────────────────┐
   ▼            ▼                              ▼
 home.rs   games/mod.rs ── guess.rs rps.rs tictactoe.rs snake.rs      about.rs
   │            │                 │
   │            │                 └─ uses ─►  modules/games/<game>/mod.rs  (RULES, plain Rust, tested)
   │            │                              impl Game for ...   (trait)
   │            └─ uses ─►  modules/games/{mod.rs, game_card.rs}
   │
   └─ uses ─► modules/games/progress.rs ─► modules/base/storage.rs ─► std::fs  |  localStorage
                                              (Persist trait)

 modules/base/  ui/card · header · footer · hooks (use_seeded) · random (Rng) · time (sleep_ms)
                storage · theme · theme_toggle
 config/        APP_NAME, VERSION, platform_name()
```

Dependency direction (never reversed): `routes -> modules -> config`. Inside `modules`: `games -> base`.

### Compare with the reference project

`rust-dioxus/frontend/src` has `config/`, `modules/{base,apps}`, `routes/` and a lot more (auth, HTTP, tailwind). Ours maps one-to-one:

| Reference | This project |
|-----------|--------------|
| `config/` | `config/` |
| `modules/base/` (shell, ui, stores, utils) | `modules/base/` (ui, header, hooks, storage, theme...) |
| `modules/apps/<name>/` (api, models) | `modules/games/<game>/` (rules) |
| `routes/` (one file per page, `Route` enum) | `routes/` (same) |
| `stores/theme.rs`, `storage.rs` | `modules/base/theme.rs`, `storage.rs` |

Same skeleton, no network and no login: that is what "simpler design, same architecture" means.

### One life of an action: "I win a guessing game"

1. Keyboard: Enter submits the `form` -> `on_submit` (a closure in `routes/games/guess.rs`).
2. It asks the **rules**: `current.guess_text("42")` -> `Ok(Hint::Correct)`.
3. The page shows the message and calls `progress.update(|p| p.record_guess(tries))`.
4. `ProgressStore::update` changes the signal (inside a short guard), then `save()`.
5. `save()` copies the value out and `spawn`s a task: `save_value("progress", &value).await`.
6. `save_value` -> `Persist::to_text` -> `storage::save` -> the file or `localStorage` (a compile-time `cfg` choice).
7. The signal change redraws every component that read it, e.g. the Home card.

Seven steps, five modules, no step needs to know how another one works. That is the architecture.

## 2. Lifetimes

A **lifetime** is the stretch of the program where a reference is valid. Every reference has one; usually the compiler works it out. You write one (`'a`) only when a signature returns or stores a reference and it is unclear *which input* it is tied to.

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str
```
"The result is valid for as long as **both** inputs are." The caller (the compiler on its behalf) then refuses to use the result after either input is gone.

### The three rules of elision (when you can omit them)

1. Each reference parameter gets its own lifetime.
2. If there is exactly **one** input lifetime, the output gets it. (`fn first_word(text: &str) -> &str`)
3. If one of the inputs is `&self` / `&mut self`, the output gets the lifetime of `self`.

When none applies (two inputs, ambiguous output), you must write it: that is the `longest` example.

### Structs that borrow

```rust
struct Line<'a> { key: &'a str, value: &'a str }
```
A `Line` cannot outlive the text it points into. In exchange it allocates nothing. Compare: a struct with `String` fields owns its data (no lifetime, but allocation).

**Which should you choose?** Own (`String`) by default. Borrow (`&'a str`) for short-lived views over data that clearly outlives them, such as a parser's temporary result. **Dioxus props are always owned**, for the reason in lesson 02: a component can be redrawn long after its creator returned.

### `'static`

- `&'static str`: the reference is valid for the whole program; string literals are like that.
- `T: 'static` (as a bound): `T` holds **no short-lived borrows**. An owned `String` is `'static`. It does *not* mean "lives forever".

### Dangling references are impossible

```rust
fn dangle() -> &String { let s = String::from("x"); &s }   // ERROR
```
`s` dies at the end of the function; the reference would point at nothing. The compiler rejects it. The fix is to **move ownership out**: return `String`.

Where you saw lifetimes before, silently: `Formatter<'_>` in every `Display` impl (lesson 06), `&'static str` in `GameInfo` (02), `G: 'static` in `use_seeded` (09), `split_once` returning slices into the line (12).

## 3. Smart pointers

A **smart pointer** owns data somewhere else and adds a rule.

| Type | Owners | Where | Rule it adds |
|------|--------|-------|--------------|
| `Box<T>` | 1 | heap | fixed-size handle to a possibly big/recursive value; `Box<dyn Trait>` |
| `Rc<T>` | many | heap | reference counting; dropped when the last owner goes; single thread |
| `Arc<T>` | many | heap | same, thread-safe (atomic counter) |
| `RefCell<T>` | 1 | inline | borrow rules checked at **run time** (`borrow`, `borrow_mut`) |
| `Mutex<T>` | 1 | inline | one thread at a time (lesson 11) |
| `Rc<RefCell<T>>` | many | heap | many owners **and** mutation (single thread) |

Run `cargo run --example exp_smart_pointers`:

- `Rc::clone(&x)` is **not** a deep copy: it adds an owner (count +1). `Rc::strong_count` shows it.
- `RefCell::try_borrow_mut()` returns `Err` while a borrow exists; `borrow_mut()` would panic. This is exactly what happens when two guards of one Dioxus signal clash: **a `Signal` behaves like an `Rc<RefCell<T>>` plus change notification**, wrapped in a `Copy` handle.
- Reference cycles with `Rc` leak memory (and Rust calls that *safe*). `Weak<T>` is the escape hatch for parent pointers. We never needed it.

### Choosing

```
need one owner, on the heap?        Box
need many owners, one thread?       Rc   (+ RefCell to mutate)
need many owners, many threads?     Arc  (+ Mutex to mutate)
inside Dioxus UI code?              Signal / use_context
```

## 4. The theme, as a final review of the toolbox

- `enum Theme { Dark, Light }` with `toggled()` and `css_class()`: lesson 04.
- `Settings` + `impl Persist`: lessons 09 + 12.
- `from_text` with a tuple `match (key, value)` and a `return Err(...)` arm: lessons 07 + 12.
- `ThemeStore` (`Copy`, holds a signal) and `use_context_provider`: lesson 12.
- `spawn(async move { let _ = save... })`: lesson 05; `let _ =` on purpose, because a failed settings save should not interrupt play.
- CSS variables: one class changes every colour. No Rust runs per element.

## 5. Concept map: where each Rust idea appears

| Concept | Lesson | Look at |
|---------|--------|---------|
| variables, `mut`, shadowing, types, functions, `match`, `loop` | 00 | `lessons/rdx-00-create-rust-project/src/main.rs` |
| crates, features, `cfg`, wasm, `Cargo.toml` | 01 | `Cargo.toml` |
| ownership, move, clone, borrow, `&'static str` vs `String` | 02 | `examples/exp_ownership.rs` |
| modules, `pub`, `use`, `crate::` | 03 | `src/main.rs`, `examples/exp_modules.rs` |
| enums, `match`, `Option`, `Result` | 04 | `routes/mod.rs`, `examples/exp_enums.rs` |
| closures, signals, `Fn`/`FnMut`/`FnOnce`, async basics | 05 | `routes/games/guess.rs`, `examples/exp_closures.rs` |
| structs, `impl`, custom errors, `Display`, `From`, `?`, tests | 06 | `modules/games/guess/mod.rs`, `examples/exp_errors.rs` |
| `Copy`/`Clone`, tuple patterns, `Default`, pure functions | 07 | `modules/games/rps/mod.rs`, `examples/exp_copy_clone.rs` |
| arrays, slices, `&mut`, borrowing rules, `RefCell` | 08 | `modules/games/tictactoe/mod.rs`, `examples/exp_borrowing.rs` |
| traits, generics, `dyn`, `impl Trait`, function pointers | 09 | `modules/games/mod.rs`, `modules/base/hooks.rs`, `examples/exp_traits_generics.rs` |
| `VecDeque`, `Vec`, `HashMap`, iterators, state machines | 10 | `modules/games/snake/mod.rs`, `examples/exp_collections.rs` |
| async timing, `use_future`, keyboard events, threads, channels | 11 | `routes/games/snake.rs`, `examples/exp_threads_channels.rs` |
| file I/O, `cfg`, parsing, context, custom trait + generic helper | 12 | `modules/base/storage.rs`, `modules/games/progress.rs`, `examples/exp_file_io.rs` |
| lifetimes, smart pointers, final architecture | 13 | `examples/exp_lifetimes.rs`, `examples/exp_smart_pointers.rs` |

## 6. What this project deliberately does NOT do

| Not done | Why | When you would add it |
|----------|-----|-----------------------|
| `rand` crate | `Rng` (xorshift) taught structs, `wrapping_*`, determinism in tests | for anything security-related, or statistical quality |
| `serde` | the text format showed what serialization *is* | once formats get nested or large |
| timer crates | `sleep_ms` shows what a timer is | in a bigger app with many timers |
| `dirs` (app folders) | std has no app-data folder; we used a documented fallback | before shipping on Android/iOS |
| a database | out of scope | when data has relations or grows |
| CSS framework | one file, one theme variable set | when the UI outgrows one stylesheet |

Each "not done" is a place where a dependency would be **justified**; you now know what it replaces.

## 7. Experiments

1. Add a fifth game (Hangman or Memory). List every file you touched; compare with the Snake list from lessons 10 and 11.
2. Run `cargo doc --open` and read your own module comments rendered as a website.
3. Run `cargo test` and count: how many of the 63 tests need no browser? (All of them.) How many lines of UI code have tests? (Almost none: that is the design.)
4. Break the theme file: write `theme=purple` into `settings.txt`; what does the app do? (It silently falls back to dark: `if let Ok(Some(saved))`.) Is that the right behaviour for settings? Compare with progress, where damage is *shown*.
5. Make `Theme` a three-way enum (`Dark`, `Light`, `Fox`). Follow the compiler: how many places must change?
6. In `exp_lifetimes.rs` un-comment each ERROR and say, in your own words, which value does not live long enough.

## 8. Key vocabulary

lifetime, elision, `'static`, borrowing struct, smart pointer, `Box`, `Rc`, `Arc`, `RefCell`, `Weak`, reference cycle, dependency direction, provider, context, architecture, concept map.

## 9. Self-check

- Draw the dependency arrows between `routes`, `modules/games`, `modules/base`, `config`.
- Why does `longest` need `'a` but `first_word` does not?
- When do you reach for `Rc<RefCell<T>>`, and what replaces it inside Dioxus?
- Which parts of the hub can you test without opening a browser, and why?
- Name one place where you would now add a dependency, and which std-only code it replaces.
