# Lesson 05 - CHANGES

Previous: lesson 04 (router, three pages, no state).

## Commands

```bash
cp -r rdx-04-routes-and-enums rdx-05-signals-and-closures
cd rdx-05-signals-and-closures
mkdir src/routes/games
mv src/routes/games.rs src/routes/games/mod.rs     # a page that gets sub-pages becomes a folder
# edit/add the files below, then
cargo clippy --all-targets
dx serve
cargo run --example exp_closures
```

(PowerShell: `Move-Item src/routes/games.rs src/routes/games/mod.rs`.) Nothing in `routes/mod.rs` has to change for the move: `pub mod games;` finds `games/mod.rs` as well.

## `Cargo.toml`
Name only. Still just `dioxus` with the `router` feature.

## `src/modules/base/random.rs` (new) + `base/mod.rs`
```rust
pub async fn random_seed() -> u64 {
    let answer = document::eval("return Math.random();").join::<f64>().await;
    match answer { Ok(f) => (f * 4_294_967_296.0) as u64, Err(_) => 12_345 }
}
```
and `pub mod random;` in `modules/base/mod.rs`.

## `src/modules/games/mod.rs`
`GameInfo` got `slug: &'static str` (`"guess"`, `"rps"`, `"tictactoe"`, `"snake"`).

## `src/modules/games/game_card.rs`
New prop `children: Element`, rendered in `div.card-action`.

## `src/routes/mod.rs`
```diff
-use games::Games;
+use games::{guess::Guess, Games};
 ...
         #[route("/games")]
         Games {},
+        #[route("/games/guess")]
+        Guess {},
```

## `src/routes/layout.rs`
`NavItem::is_active(self, current: &Route) -> bool` (uses `matches!`) replaces `current == target`, so the **Games** tab stays highlighted on `/games/guess`.

## `src/routes/games/mod.rs`
- `fn route_for(slug: &str) -> Option<Route>`
- each card gets either `Link { ... "Play" }` or `span { "coming soon" }` chosen by a `match` expression inside `rsx!`.

## `src/routes/games/guess.rs` (new, the lesson)
Signals: `secret: Option<u32>`, `input: String`, `message: String`, `tries: u32`, `won: bool`.
Closures: `new_game` (spawns the async seed request), `on_submit` (parse, compare, update).
View: message, form with input + Guess button, tries counter, New game button.

## `assets/main.css`
Appended: `.muted`, `.card-action`, `.message(.win)`, `.row`, `input[type=text]`, `.btn.secondary`.

## `examples/exp_closures.rs` (new)
Seven numbered closure experiments.

## Errors you may meet (and what they mean)

| Error | Cause |
|-------|-------|
| `unused_mut` warning on `new_game` | the closure does not mutate its own captures, so `let new_game = move || ...` is enough |
| "already borrowed" panic at run time | writing a signal while a `.read()` guard of the same signal is alive (see KNOWLEDGE section 5) |
| `use of moved value` in handlers | capturing a non-Copy value (e.g. a `String`) with `move` in two closures: clone it first |

## Verify

1. `/games`: Play on the first card opens the game. The other cards say "coming soon".
2. Type `abc` -> "not a whole number". `500` -> "Pick a number from 1 to 100".
3. Binary-search to a win: the message shows the number of tries; the New game button starts over with a new secret.
4. `cargo clippy --all-targets`: silent.
