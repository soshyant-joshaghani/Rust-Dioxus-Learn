# Lesson 13 - The finished hub

> Goal: finish the **FoxG Game Hub** and look back at it as a whole. We add a light/dark **theme** (a second saved setting that costs almost no new code), a real **About** page, and close the remaining Rust topics: **lifetimes** and **smart pointers**.

This lesson's code is also copied to the project root (`rust-dioxus-learn/src`), so the root is always "the finished app".

## The three pages

| URL | Page |
|-----|------|
| `/` | **Home**: hero + your saved progress |
| `/games` | **Games**: the catalogue; `/games/guess`, `/games/rps`, `/games/tictactoe`, `/games/snake` open each game |
| `/about` | **About foxg-kit**: version, platform, data location, what each game teaches |

Header on every page: brand, version, navigation, theme toggle.

## What you learn

| Rust | Project |
|------|---------|
| reusing a trait: `impl Persist for Settings` | theme saved with zero new storage code |
| `env!("CARGO_PKG_VERSION")`, `cfg!`, `std::env::consts::OS` | `config::VERSION`, `platform_name()` |
| `Copy` store + context, again | `ThemeStore` |
| **lifetimes**: `'a`, elision, `'static`, structs that borrow | `examples/exp_lifetimes.rs` |
| **smart pointers**: `Box`, `Rc`, `RefCell`, `Rc<RefCell<T>>` | `examples/exp_smart_pointers.rs` |
| a CSS variable theme switched by one class | `.app.light` |
| reading the whole architecture | KNOWLEDGE section 1 |

## Run

```bash
cd lessons/rdx-13-the-hub          # or the project root
cargo test                          # 63 tests
dx serve                            # web
dx serve --platform desktop         # desktop
cargo run --example exp_lifetimes
cargo run --example exp_smart_pointers
```

## Files (new or changed)

```
src/config/mod.rs                   VERSION, platform_name()
src/modules/base/theme.rs           Theme, Settings (Persist), ThemeStore, hooks (+5 tests)
src/modules/base/theme_toggle.rs    the header button
src/modules/base/header.rs          shows version, hosts the toggle
src/modules/base/storage.rs         ParseError: MissingEquals / NotANumber / InvalidValue
src/routes/about.rs                 the finished About page
src/app.rs                          theme provider + wrapper class
assets/main.css                     .app, .app.light
examples/exp_lifetimes.rs  exp_smart_pointers.rs
```

## What now?

You have a complete, small, documented Rust + Dioxus app. Ideas, from easy to ambitious:

1. Add a fifth game (Memory match? Hangman?). Count the files you touch: that is your architecture's score.
2. Add a computer opponent to Tic-Tac-Toe.
3. Add a `Settings` option for Snake's board size.
4. Package the app: `dx bundle --platform desktop --release` (not run while writing these lessons).
5. Replace the `data_dir()` fallback with the platform's real app folder, as a **conscious** decision to take on one dependency.

## Read next

`CHANGES.md`, then `KNOWLEDGE.md` (it ends with a map of every concept and where it appears).
