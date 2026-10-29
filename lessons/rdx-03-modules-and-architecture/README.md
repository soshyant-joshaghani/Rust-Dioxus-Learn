# Lesson 03 - Modules and architecture

> Goal: split the single `main.rs` of lesson 02 into the real folder layout of the project. The page looks **identical**; the code is now organised the way it will stay until the end of the course.

This lesson is about *where code lives*. Architecture matters most, so we do it early, while the project is tiny and mistakes are cheap.

## The layout

```
src/
  main.rs              crate root: `mod` declarations + launch
  app.rs               App component: stylesheet + page
  config/mod.rs        constants (APP_NAME, LESSON_TAG)
  routes/              PAGES (one file per page)
    mod.rs
    home.rs
  modules/             BUILDING BLOCKS
    mod.rs
    base/              shared: layout + generic ui
      header.rs  footer.rs
      ui/card.rs
    games/             the game domain
      mod.rs           GameInfo, GAMES
      game_card.rs
```

This is the same idea as the reference project `rust-dioxus/frontend/src` (`config`, `modules/base`, `modules/apps`, `routes`), only simpler.

## What you learn

| Rust | Project skill |
|------|---------------|
| `mod`, `pub`, `use`, `crate::`, `super::` | deciding what goes in `routes` vs `modules` vs `config` |
| privacy: private by default | one-direction dependencies: routes -> modules -> config |
| `mod.rs` files and crate roots | the `children` prop and a first honest abstraction (`Card`) |

## Run

```bash
cd lessons/rdx-03-modules-and-architecture
dx serve
cargo run --example exp_modules     # privacy lab
```

## Checklist

- [ ] The page looks the same as lesson 02 (only the tag says "lesson 03").
- [ ] You can answer: "where do I put a new page? a new shared button? a new game?"
- [ ] You un-commented the `// ERROR` lines in `examples/exp_modules.rs` and read each message.

## Challenge

Add `src/modules/base/ui/badge.rs` with a `Badge { text: String }` component (the little orange pill). Use it in `GameCard` and `Header`. What do you have to add besides the file?  *(Answer: `pub mod badge;` in `ui/mod.rs`. A file nobody declares does not exist.)*

## Read next

`CHANGES.md` -> `KNOWLEDGE.md` -> [lesson 04](../rdx-04-routes-and-enums/README.md): real pages with a router.
