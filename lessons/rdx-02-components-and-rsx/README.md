# Lesson 02 - Components and `rsx!`

> Goal: take the template apart and build the first real page of the **FoxG Game Hub**: a header, a hero, and a grid of game cards. Meet functions-as-components, `rsx!`, structs, and **ownership** for the first time.

## What you build

A static home page. No clicks yet, no router yet.

```
App
|-- Header      "FoxG Game Hub"
|-- Hero        "Learn Rust by playing"  +  "4 small games..."
|-- GameGrid
|   `-- GameCard x4   (title, blurb, the Rust concept the game teaches)
`-- Footer
```

## What you learn

| Rust | Dioxus |
|------|--------|
| functions that return a value | a component is a function returning `Element` |
| `struct` and `const` arrays | `rsx!` syntax: elements, attributes, text, `{expr}` |
| `&'static str` vs `String` | `#[component]`, props |
| **ownership: move vs borrow** | `for` loops inside markup |
| `.iter()` borrows | CSS file with `asset!` |

## Run

```bash
cd lessons/rdx-02-components-and-rsx
dx serve                       # web
dx serve --platform desktop    # native window
cargo run --example exp_ownership   # the ownership lab (no UI)
```

## Files

```
rdx-02-components-and-rsx/
  Cargo.toml
  Dioxus.toml            title: "FoxG Game Hub"
  assets/main.css        all styling (plain CSS, theme colours as variables)
  assets/favicon.ico
  src/main.rs            App, Header, Hero, GameGrid, GameCard, Footer
  examples/exp_ownership.rs   experiment: move / clone / copy / borrow
```

## Challenge

Add a fifth entry to `GAMES`. Change `[GameInfo; 4]` to `[GameInfo; 5]` and watch the "N small games" text update by itself. Then try to forget to change the `4`: read the compiler error.

## Read next

`CHANGES.md` -> `KNOWLEDGE.md` -> [lesson 03](../rdx-03-modules-and-architecture/README.md) where this one file becomes the real project layout.
