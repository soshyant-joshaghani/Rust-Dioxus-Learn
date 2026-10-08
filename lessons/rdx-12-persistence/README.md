# Lesson 12 - Persistence: saving progress

> Goal: the hub **remembers**. Best scores and counters survive closing the tab or the app. We write the file format by hand and read/write it with `std::fs` (native) or `localStorage` (web). Concepts: file I/O, `Result` everywhere, `?` with `From`, a custom trait (`Persist`), conditional compilation (`cfg`), and Dioxus **context**.

## What you build

- the Home page gets a **Your progress** card:
  Number Guessing best (fewest tries), Rock Paper Scissors rounds, Tic-Tac-Toe finished games, Snake best score,
- every game records into it, and every change is saved,
- a visible line tells you **where** the data lives (a folder on desktop, "browser storage" on the web),
- damaged data shows a red message instead of silently disappearing,
- **Reset progress** clears it.

## What you learn

| Rust | Project |
|------|---------|
| `std::fs`: `read_to_string`, `write`, `create_dir_all`, `rename` | `native::load_from / save_to` |
| `Path` / `PathBuf`, environment variables (`var_os`) | `data_dir()` |
| `io::Error`, `ErrorKind::NotFound` as "not an error" | first run |
| `?` + `From<io::Error>` | `StorageError` |
| your own **trait** with a generic user | `Persist`, `save_value<T: Persist>` |
| `#[cfg(target_arch = "wasm32")]` | two implementations, one API |
| text parsing: `lines`, `split_once`, `ok_or_else`, `map_err` | `Progress::from_text` |
| lenient vs strict parsing | old/new save files still load; damaged ones are reported |
| Dioxus **context** (`use_context_provider`, `use_context`) | the shared `ProgressStore` |
| `Copy` struct holding signals | `ProgressStore` |

## Run

```bash
cd lessons/rdx-12-persistence
cargo test                        # 58 tests (12 new, including real file tests in a temp folder)
cargo check --target wasm32-unknown-unknown   # the web branch compiles too
dx serve                          # web: saved in localStorage
dx serve --platform desktop       # desktop: saved in %APPDATA%\foxg-hub\progress.txt
cargo run --example exp_file_io
```

Play a game, close the app, start it again: the Home card remembers. Open the saved file and read it.

## Files

```
src/modules/base/storage.rs        Persist trait, StorageError, ParseError, load/save (native + web)
src/modules/games/progress.rs      Progress (data + text format) and ProgressStore (+ context hook)
src/app.rs                         calls use_progress_provider()
src/routes/home.rs                 the progress card
src/routes/games/*.rs              each game calls progress.update(...)
examples/exp_file_io.rs            std::fs lab
```

## Challenge

Add a `theme=dark|light` **setting**: a second type `Settings` that implements `Persist`, saved under the name `"settings"`. How many lines of storage code did you have to write? (Hopefully none: `save_value` and `load_value` are generic.) Lesson 13 does this for real.

## Read next

`CHANGES.md` -> `KNOWLEDGE.md` -> [lesson 13](../rdx-13-the-hub/README.md): the finished hub.
