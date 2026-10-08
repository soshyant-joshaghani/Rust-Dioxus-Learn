# Lesson 04 - Routes and enums

> Goal: turn the one-page app into three real pages (`/`, `/games`, `/about`) plus a "not found" page, using Dioxus' router. The central Rust concept: **enums and `match`**.

## What you build

```
/        Home    hero + "See the games"
/games   Games   the catalogue (moved from Home)
/about   About   what foxg-kit is
/xyz     NotFound  carries the unknown path
```

All pages live inside one `AppLayout` (header with navigation, page, footer).

## What you learn

| Rust | Dioxus |
|------|--------|
| `enum`: variants with and without data | `#[derive(Routable)]` on an enum |
| `match` and exhaustiveness | `Router`, `Link`, `Outlet`, `use_route` |
| `Option<T>` and `Result<T, E>` are enums | layouts wrap pages |
| derive: `Debug`, `Clone`, `Copy`, `PartialEq` | feature flags on a dependency (`features = ["router"]`) |
| `Vec<String>` | `modules` must not import `routes` (so `Header` takes `children`) |

## Run

```bash
cd lessons/rdx-04-routes-and-enums
dx serve
cargo run --example exp_enums
```

Open `/games`, click around, then type a wrong URL such as `/nothing/here`.

## Files that are new or changed

```
Cargo.toml                     features = ["router"]
src/app.rs                     now just: stylesheet + Router::<Route>
src/routes/mod.rs              the Route enum
src/routes/layout.rs           AppLayout, NavItem enum, NavLink
src/routes/{home,games,about,not_found}.rs
src/modules/base/header.rs     takes `children` (the nav links)
examples/exp_enums.rs          enums / Option / Result lab
```

## Challenge

Add a fifth `NavItem` called `Settings` **without** adding a page. Read the compiler error: it lists every `match` you must update. Then add the page `/settings` for real.

## Read next

`CHANGES.md` -> `KNOWLEDGE.md` -> [lesson 05](../rdx-05-signals-and-closures/README.md): our first game, and state that changes.
