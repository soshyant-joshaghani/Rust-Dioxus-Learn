# Lesson 02 - CHANGES

Previous: lesson 01, the untouched `dx new` Bare-Bones project.

## Commands

Start from a copy of lesson 01 (this repo already has the result in `rdx-02-components-and-rsx/`):

```bash
cp -r rdx-01-create-dioxus-project rdx-02-components-and-rsx     # or copy in Explorer
cd rdx-02-components-and-rsx
```

Then edit by hand as listed below, and run:

```bash
dx serve                 # see the page
cargo check              # fast compile check, no server
cargo clippy --all-targets   # linter advice, should print nothing
cargo run --example exp_ownership
```

## File changes

### `Cargo.toml`
- `name` -> `rdx-02-components-and-rsx`
- `edition` `"2021"` -> `"2024"` (same edition as lesson 00; both work with Dioxus)
- dependencies unchanged: **only `dioxus`**.

### `Dioxus.toml`
- Simplified to just `title = "FoxG Game Hub"`. The empty `[web.resource]` tables from the template were not needed.

### `assets/`
- **deleted** `header.svg` (the Dioxus logo from the template)
- **replaced** `main.css`: tokens in `:root` (`--bg`, `--panel`, `--accent`...), then `.page`, `.header`, `.hero`, `.grid`, `.card`, `.tag`
- kept `favicon.ico`

### `src/main.rs` (rewritten)

| Template | Now |
|----------|-----|
| `HEADER_SVG` asset | removed |
| `App` rendering `Hero {}` with links | `App` assembles `Header`, `Hero`, `GameGrid`, `Footer` |
| `Hero` with six links | `Hero` shows a title and a count computed from `GAMES.len()` |
| (none) | `struct GameInfo`, `const GAMES: [GameInfo; 4]` |
| (none) | `GameGrid` loops over `GAMES.iter()` |
| (none) | `GameCard(title, blurb, concept)`, the first component **with props** |

Key lines:

```rust
const GAMES: [GameInfo; 4] = [ ... ];

for game in GAMES.iter() {
    GameCard { title: game.title.to_string(), blurb: game.blurb.to_string(), concept: game.concept.to_string() }
}

#[component]
fn GameCard(title: String, blurb: String, concept: String) -> Element { ... }
```

### `examples/exp_ownership.rs` (new)
A runnable lab, not part of the app. Cargo finds files in `examples/` automatically.

## What did NOT change

- the `[features]` table (`web`/`desktop`/`mobile`)
- `dioxus::launch(App)` in `main`

## Verify

- `dx serve`: dark page, orange brand name, four cards.
- `cargo check` and `cargo clippy --all-targets`: no warnings.
- `cargo run --example exp_ownership`: seven numbered lines.
- Compile-error drills: in `GameGrid` replace `game.title.to_string()` with `game.title` and read the message about `&str` vs `String`.
