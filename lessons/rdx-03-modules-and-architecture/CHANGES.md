# Lesson 03 - CHANGES

Previous: lesson 02, everything in one `src/main.rs`.

## Commands

```bash
cp -r rdx-02-components-and-rsx rdx-03-modules-and-architecture
cd rdx-03-modules-and-architecture
# create the folders
mkdir -p src/config src/routes src/modules/base/ui src/modules/games
```

(PowerShell: `New-Item -ItemType Directory -Force src/config, src/routes, src/modules/base/ui, src/modules/games`)

Then move code as in the table, and check often:

```bash
cargo check
cargo clippy --all-targets
dx serve
cargo run --example exp_modules
```

## Where every piece of lesson 02 went

| Lesson 02 (`main.rs`) | Lesson 03 |
|-----------------------|-----------|
| `fn main` | `src/main.rs` (with the `mod` lines) |
| `App` + the `asset!` constants | `src/app.rs` |
| string `"FoxG Game Hub"` typed in `Header` | `src/config/mod.rs` -> `APP_NAME` |
| `Header` | `src/modules/base/header.rs` |
| `Footer` | `src/modules/base/footer.rs` |
| `GameInfo`, `GAMES` | `src/modules/games/mod.rs` (now `pub`) |
| `GameCard` | `src/modules/games/game_card.rs` |
| `Hero` + `GameGrid` | merged into `src/routes/home.rs` as the `Home` page |
| (new) | `src/modules/base/ui/card.rs`: `Card` |
| (new) | `mod.rs` file in every folder |
| (new) | `examples/exp_modules.rs` |

## What changed in each file

### `src/main.rs`
```rust
mod app;
mod config;
mod modules;
mod routes;

fn main() { dioxus::launch(app::App); }
```
Four `mod` lines register the four top-level modules. The file also holds the architecture map as a doc comment.

### Every `mod.rs`
Lists its children as `pub mod name;`. Example `modules/base/mod.rs`:
```rust
pub mod footer;
pub mod header;
pub mod ui;
```

### Visibility changes
Everything that crosses a file boundary got `pub`: `App`, `Header`, `Footer`, `Home`, `GameCard`, `Card`, `GameInfo` **and each of its fields**, `GAMES`, `APP_NAME`, `LESSON_TAG`.

### `Card` (new component)
```rust
#[component]
pub fn Card(children: Element) -> Element {
    rsx! { div { class: "card", {children} } }
}
```
`GameCard` now wraps its content in `Card { ... }`, so the CSS class `card` appears in exactly one Rust file.

### `assets/`, `Dioxus.toml`, `Cargo.toml`
Only the package name changed in `Cargo.toml`. Nothing else: still **one** dependency.

## Verify

1. `dx serve`: same page as lesson 02.
2. `cargo clippy --all-targets`: silent.
3. Break it on purpose: delete the `pub` in front of `fn Header` and read the error: "function `Header` is private". Restore it.
4. Break it again: delete `pub mod card;` in `ui/mod.rs`. The error is at the *use* site, which tells you the module does not exist, even though the file does.
