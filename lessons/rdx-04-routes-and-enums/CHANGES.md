# Lesson 04 - CHANGES

Previous: lesson 03 (modules; one page, no router).

## Commands

```bash
cp -r rdx-03-modules-and-architecture rdx-04-routes-and-enums
cd rdx-04-routes-and-enums
# edit as below, then:
cargo clippy --all-targets
dx serve
cargo run --example exp_enums
```

## `Cargo.toml`

```diff
-dioxus = { version = "0.7.1", features = [] }
+dioxus = { version = "0.7.1", features = ["router"] }
```
Still only the `dioxus` crate; the router is an optional *feature* of it. Without this line `Routable` and `use_route` do not exist (that was our first compile error).

## `src/config/mod.rs`
`LESSON_TAG` -> `"lesson 04"`.

## `src/app.rs`
`Home {}` replaced by `Router::<Route> {}`.

## `src/routes/mod.rs` (the heart of the lesson)
```rust
#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/")]      Home {},
        #[route("/games")] Games {},
        #[route("/about")] About {},
    #[end_layout]
    #[route("/:..segments")] NotFound { segments: Vec<String> },
}
```
Plus `pub mod` lines for the new files and `use` lines for each page.

## `src/routes/layout.rs` (new)
- `enum NavItem { Home, Games, About }` with `label()` and `route()` methods (both are `match`es).
- `const NAV_ITEMS: [NavItem; 3]`.
- `AppLayout`: `Header { for item in NAV_ITEMS { NavLink { item } } }`, `Outlet::<Route>`, `Footer`.
- `NavLink`: uses `use_route()` and `==` to add the `active` CSS class.

## Pages
| File | Content |
|------|---------|
| `home.rs` | hero (title, count, button linking to `Route::Games {}`). The catalogue moved out. |
| `games.rs` | `h2` + the grid (the loop that used to be on Home) |
| `about.rs` | a `Card` with two paragraphs about foxg-kit |
| `not_found.rs` | `segments.join("/")` and a link back |

## `src/modules/base/header.rs`
`Header()` -> `Header(children: Element)`; renders `nav { {children} }`. Reason: the nav needs `Route`, and `modules` may not import `routes`.

## `assets/main.css`
Appended: `.nav`, `.nav-link`, `.nav-link.active`, `.title`, `.btn`.

## `examples/`
- new `exp_enums.rs`
- `exp_ownership.rs`, `exp_modules.rs` carried over (experiments accumulate)

## Compile errors you will probably hit (and why)

1. `cannot find ... Routable / use_route` -> forgot `features = ["router"]`.
2. `binary operation == cannot be applied to NavItem` -> a prop type needs `PartialEq`; add it to the `derive`.
3. `non-exhaustive patterns` after adding a `NavItem` variant -> exactly what the challenge asks for.

## Verify

- `/`, `/games`, `/about` open from the nav; the current link is highlighted.
- A wrong URL shows "Not found" with the path.
- Reloading the browser on `/games` works (dx serves `index.html` for unknown paths so the router can take over).
- `cargo clippy --all-targets` is silent.
