# rust-dioxus-learn

A small **game hub** written in Rust with [Dioxus](https://dioxuslabs.com) (web, desktop, Android), built as a step-by-step course. This folder is the **finished app**; the lessons that lead to it are in [`lessons/`](lessons/README.md).

Pages: **Home** (`/`, with your saved progress), **Games** (`/games`: Number Guessing, Rock Paper Scissors, Tic-Tac-Toe, Snake), **About** (`/about`: foxg-kit).

## Run

```bash
dx serve                          # web, http://localhost:8080
dx serve --platform desktop       # native window
dx serve --platform android       # needs the Android setup from lessons/rdx-01
cargo test                        # 63 unit tests, no browser needed
cargo run --example exp_ownership # experiments live in examples/
```

## Architecture

Same skeleton as `../rust-dioxus/frontend/src`, simpler:

```
src/
  main.rs          launch
  app.rs           providers (progress, theme) + router
  config/          APP_NAME, VERSION, platform_name()
  modules/
    base/          ui/card, header, footer, hooks, random, time, storage, theme
    games/         Game trait, catalogue, progress, and one folder of RULES per game
  routes/          Route enum, layout, home, about, games/<game>.rs (pages)
```

Rules (`modules/games/*`) are plain Rust with tests; pages (`routes/games/*`) only draw and forward input. Dependencies point one way: `routes -> modules -> config`.

## Rules of the project

- One dependency: `dioxus`. Everything else is `std`.
- Code is written to be read: every file starts with a `//!` comment explaining what it is for, and ownership/borrowing is explained where it matters.
- Errors are values (`Result`, `Option`) and are shown, not hidden.

## Learn from it

Start with [`lessons/README.md`](lessons/README.md). Each lesson has `README.md` (short), `CHANGES.md` (exact commands and diffs) and `KNOWLEDGE.md` (the explanations).
