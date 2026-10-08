# Lesson 12 - CHANGES

Previous: lesson 11 (four games, no memory between runs).

## Commands

```bash
cp -r rdx-11-snake-loop rdx-12-persistence
cd rdx-12-persistence
# add/edit files below, then
cargo clippy --all-targets
cargo test
cargo check --target wasm32-unknown-unknown     # needs: rustup target add wasm32-unknown-unknown (done in lesson 01)
dx serve
cargo run --example exp_file_io
```

## Files

| File | Change |
|------|--------|
| `Cargo.toml` | name only. Still only `dioxus`. |
| `src/config/mod.rs` | `LESSON_TAG = "lesson 12"` |
| `src/modules/base/storage.rs` | **new** (~280 lines): `StorageError`, `ParseError`, `trait Persist`, `save_value`, `load_value`, `load`, `save`, `location_hint`, private `native` and `web` modules, 4 file tests |
| `src/modules/base/mod.rs` | `pub mod storage;` |
| `src/modules/games/progress.rs` | **new**: `Progress`, `impl Persist for Progress`, `ProgressStore`, `use_progress_provider`, `use_progress`, 8 tests |
| `src/modules/games/mod.rs` | `pub mod progress;` |
| `src/app.rs` | `use_progress_provider();` before the router |
| `src/routes/home.rs` | **rewritten**: hero + progress card |
| `src/routes/games/guess.rs` | `progress.update(\|p\| p.record_guess(tries))` on `Hint::Correct` |
| `src/routes/games/rps.rs` | `record_rps_round()` after each round |
| `src/routes/games/tictactoe.rs` | `record_tictactoe_game()` when status leaves `InProgress` |
| `src/routes/games/snake.rs` | loop returns `(finished, score)`; `record_snake(score)` on game over; shows "Best score" |
| `assets/main.css` | `.stats`, `.message.error` |
| `examples/exp_file_io.rs` | **new** |

## The save file

`progress.txt` (desktop) / `localStorage["foxg:progress"]` (web):

```
guess_best_tries=6
rps_rounds=12
tictactoe_games=3
snake_best=7
```

Desktop location, first rule that applies: `FOXG_DATA_DIR`, then `%APPDATA%\foxg-hub` (Windows), then `$HOME/.local/share/foxg-hub` (macOS/Linux), then the temp folder.

## The key APIs

```rust
pub trait Persist: Sized {
    fn to_text(&self) -> String;
    fn from_text(text: &str) -> Result<Self, ParseError>;
}
pub async fn save_value<T: Persist>(name: &str, value: &T) -> Result<(), StorageError>
pub async fn load_value<T: Persist>(name: &str) -> Result<Option<T>, StorageError>
```
`Ok(None)` means "nothing saved yet" (first run), which is not an error.

```rust
store.update(|p| p.record_snake(score));      // change + save
let progress = store.get();                   // a copy
if let Some(problem) = store.notice() { ... } // storage problems, shown on Home
```

## Verified in a real browser while writing this lesson

1. Win the guessing game -> `localStorage["foxg:progress"]` contains `guess_best_tries=6` and the other three keys.
2. Open `/`: the card shows the saved numbers and `Saved in: browser storage (localStorage)`.
3. Put `snake_best=lots` into storage and reload: a red line says `Could not load progress: saved data is damaged: 'lots' is not a number (key 'snake_best')`.
4. Put `guess_best_tries=4`, `snake_best=9`, `future=1` (an unknown key): the card shows 4 tries and 9, no error.
5. **Reset progress** zeroes the card and rewrites the stored text.

## Verified by tests (native)

The file functions are tested against a real temporary folder: missing file -> `Ok(None)`, save/load round trip, overwrite, no `.tmp` left behind. The desktop and Android runs were **not** driven by hand here; Android in particular has no `HOME`/`APPDATA` for us, so it falls back to the temp folder (see KNOWLEDGE section 8).

## Build notes

- `use dioxus::prelude::*;` in `storage.rs` is compiled only for wasm (`#[cfg(target_arch = "wasm32")]`), otherwise the native build warns "unused import".
- clippy complained about doc-comment list formatting in `progress.rs`; the comments were rewritten as proper Markdown lists with blank lines.
- `cargo check --target wasm32-unknown-unknown` is the quick way to prove the web-only code compiles without running a server.
