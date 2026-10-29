# Lesson 03 - KNOWLEDGE (the book part)

---

## 1. Why architecture, and why now?

Code that "works" is not enough. Code must be **findable**. A good folder layout answers questions without a search:

| Question | Answer in this project |
|----------|------------------------|
| Where is the page at `/about`? | `src/routes/about.rs` |
| Where is a reusable button? | `src/modules/base/ui/` |
| Where do the rules of Snake live? | `src/modules/games/snake/` |
| Where do I rename the app? | `src/config/mod.rs` |

The three top-level groups have **one job each**:

- `routes/`: *what the user sees at a URL*. Thin. Composes pieces.
- `modules/`: *the pieces and the logic*. Reusable, testable.
- `config/`: *constants*. No logic.

And one **direction rule**: `routes -> modules -> config`. Never the reverse. If `modules/games` imported something from `routes`, two files could not be understood alone. Dependencies pointing one way is what lets you read a module without reading the whole project.

## 2. What is a module?

A **module** is a named namespace that groups items (functions, structs, constants, other modules). It controls **two** things:

1. **Names**: `games::GAMES` vs `other::GAMES` do not clash.
2. **Privacy**: everything inside is private until marked `pub`.

A **crate** is a tree of modules. The root of the tree is `main.rs` (for a program) or `lib.rs` (for a library).

## 3. Declaring modules: `mod`

```rust
// main.rs
mod app;          // look for src/app.rs  OR  src/app/mod.rs
mod routes;       // src/routes.rs OR src/routes/mod.rs
```

`mod x;` is not an import. It *creates* the module `x` by loading a file. A `.rs` file that is never named in a `mod` line is **not compiled at all**. (That is the most common "why doesn't my change do anything?" for beginners.)

Two file styles exist, both valid:

```
routes.rs  + routes/home.rs       (2018 style)
routes/mod.rs + routes/home.rs    (what we use: every folder has a mod.rs, easy to spot)
```

Inline modules (`mod name { ... }`) are the same thing without a file. `examples/exp_modules.rs` uses them to show everything in one screen.

## 4. Paths and `use`

```rust
use crate::modules::base::ui::card::Card;
```

| Path start | Meaning |
|------------|---------|
| `crate::` | the crate root (`main.rs`) |
| `self::` | the current module |
| `super::` | the parent module |
| a name | something already in scope, or an external crate (`dioxus::...`) |

`use` only creates a shortcut. After `use ...::Card;` you may write `Card` instead of the long path. It is purely about convenience, not about loading code. Group imports:

```rust
use crate::modules::base::{footer::Footer, header::Header};
```

**Why `crate::` and not relative paths?** With absolute paths, moving a file does not silently change what an import means.

### The prelude

`use dioxus::prelude::*;` imports a curated bundle of names (`rsx!`, `Element`, `component`, `use_signal`, ...). The Rust standard library has one too (`Option`, `Vec`, `String`, `Result`...) which is why you never import those.

## 5. Privacy: private by default

```rust
pub struct GameInfo {
    pub title: &'static str,    // each field needs its own pub
    secret: u32,                // private: only code inside the `games` module sees it
}
```

Rules:

- An item is visible **inside the module where it is defined and in that module's children**.
- `pub` makes it visible wherever its parent is visible. (A `pub fn` inside a private `mod` is still unreachable from outside.)
- `pub(crate)` = visible in the whole crate but not outside it. Good default for an application: you do not publish an API to anyone.
- Struct fields are private unless marked. A struct with private fields can only be built **inside its module**, which is how a module guarantees "a `GameInfo` is always valid" (`examples/exp_modules.rs`).

Private by default makes the public surface of a module a **deliberate decision**, which keeps modules small to understand: to learn how to use a module, read its `pub` items only.

## 6. `children` and the first abstraction

```rust
#[component]
pub fn Card(children: Element) -> Element {
    rsx! { div { class: "card", {children} } }
}

Card { h3 { "{title}" }  p { "{blurb}" } }
```

`children` is a reserved prop name: whatever you put between the braces is delivered as an `Element`. `{children}` in `rsx!` renders an already-built Element (curly braces = "an expression goes here").

### Abstraction check (how we decide)

This course follows a rule: **introduce an abstraction only when you can write down three things.**

| Question | `Card` |
|----------|--------|
| Why does it exist? | many pages want the same boxed look |
| What would code look like without it? | each page repeats `div { class: "card" ... }` |
| Why is it useful? | the class name lives in one file, one edit renames it everywhere |

If the third answer is weak, delete the abstraction. A wrapper that merely forwards to another thing without adding meaning only hides code.

## 7. Doc comments

```rust
//! Module-level docs. Describes the file they are in.   (inner doc comment)
/// Item-level docs. Describes the item below.           (outer doc comment)
```

`cargo doc --open` turns them into a browsable website. Try it: your own module comments appear there. That is why every file in this project begins with `//!`: *select any file and understand it without reading the others.*

## 8. Binary crate vs library crate (why we have no `lib.rs`)

The reference project has both `lib.rs` and `main.rs` so that tests in a separate folder can import the code. We stay simpler: one binary crate, tests inside the files (`#[cfg(test)] mod tests`, lesson 06). If you later want to reuse the game rules in a second program, you move `modules/` into a `lib.rs`; the module tree stays the same.

## 9. What to put where (decision table)

| You want to add... | Put it in |
|--------------------|-----------|
| a new page (`/about`) | `routes/about.rs` + a `Route` variant (lesson 04) |
| a generic ui part (button, badge) | `modules/base/ui/` |
| layout around pages (nav, footer) | `modules/base/` |
| rules/state of a game | `modules/games/<game>/` |
| a constant used in many places | `config/mod.rs` |

## 10. Experiments

1. `cargo run --example exp_modules` and un-comment each ERROR line.
2. Make `LESSON_TAG` private (remove `pub`). Read the error that names the module boundary.
3. Run `cargo doc --open` and find `modules::base::ui::card::Card`.
4. Move `Footer` into `ui/` and fix every error. Notice how the compiler walks you through each broken path. (Renaming/moving with compiler help is a core Rust skill.)
5. Add `pub(crate)` instead of `pub` on `Card`. Does anything break? Why not?

## 11. Key vocabulary

module, crate root, `mod`, `use`, `pub`, `pub(crate)`, `crate::`, `super::`, prelude, `mod.rs`, doc comment, `children`, abstraction.

## 12. Self-check

- What happens to a `.rs` file that no `mod` line mentions?
- What is the difference between `mod` and `use`?
- Why is `pub struct S { x: i32 }` not enough for outside code to read `x`?
- Which direction may imports flow between `routes`, `modules`, `config`, and why?
