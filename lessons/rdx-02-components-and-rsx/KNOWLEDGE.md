# Lesson 02 - KNOWLEDGE (the book part)

Keep `src/main.rs` open. Every example here exists in that file or in `examples/exp_ownership.rs`.

---

## 1. A component is a function

```rust
#[component]
fn Footer() -> Element {
    rsx! { div { class: "footer", "Built with Rust and Dioxus." } }
}
```

- Plain Rust function. No class, no inheritance, no hidden magic.
- It returns an `Element`: a description of some UI (it is actually `Result<VNode, RenderError>` underneath, which is why you sometimes see `?` in components).
- `#[component]` is an **attribute macro**. It rewrites the function so you can write `Footer {}` in markup and so props can be generated. Component names are `PascalCase`.
- A component must be **pure-ish**: given the same inputs it should describe the same UI. Side effects belong in event handlers and hooks (lesson 05).

Why many small components instead of one big one? Each one has a name, a single job, and can be moved to its own file (lesson 03). This is the same reason we write small functions in any language.

## 2. `rsx!` syntax cheat sheet

```rust
rsx! {
    div { class: "card", id: "x",         // element, attributes are `name: value,`
        h3 { "{title}" }                  // text; {title} interpolates a variable
        p { "a plain string" }
        MyComponent { prop: 1 }           // a component: capitalised
        if show { span { "yes" } }        // `if` works inside markup
        for item in list.iter() { li { "{item}" } }   // so does `for`
    }
}
```

Rules to remember:

1. Attributes first, then children.
2. Attributes are separated by commas; children are not.
3. Text must be a string literal. It may contain `{name}` and `{expr:?}` like `println!`.
4. Class names are written as strings: `class: "card tag"`.
5. `rsx!` is a macro: it is expanded **at compile time**. Wrong attribute names or missing commas are compile errors, with the position pointing into your markup.

Compare with HTML: there are no closing tags (the braces close elements) and no angle brackets.

## 3. `struct` and `const`

```rust
struct GameInfo {
    title: &'static str,
    blurb: &'static str,
    concept: &'static str,
}
```

A **struct** groups named fields into one type. It is the Rust way to say "a game has a title and a blurb and a concept". We create one with `GameInfo { title: "Snake", ... }`. Methods come in lesson 06.

```rust
const GAMES: [GameInfo; 4] = [ ... ];
```

- `[T; N]` is an **array**: exactly `N` values of the same type, stored inline, size known at compile time.
- A `const` is inlined wherever it is used. Good for fixed tables.
- `GAMES.len()` is `4`. Because the length is part of the type, adding a fifth element without changing `4` is a compile error. A free consistency check.

## 4. `&'static str` versus `String` (the real story)

| | `&'static str` | `&str` | `String` |
|---|---|---|---|
| Lives where | inside the program binary | somewhere else | heap |
| Who owns the text | nobody (it is always there) | someone else | the variable itself |
| Can grow | no | no | yes |
| Cost to pass around | tiny (pointer + length) | tiny | moving = tiny, cloning = a copy |

A string literal such as `"Snake"` is a `&'static str`. The `'static` is a **lifetime**: "this reference is valid for the entire run of the program". Lifetimes are Rust's way of saying how long a borrow lasts. For now, read `'static` as "can never dangle". Lesson 13 comes back to lifetimes properly.

`game.title.to_string()` creates a **new owned `String`** by copying the characters. (`String::from(..)`, `.to_owned()` and `.into()` do the same job.)

## 5. Ownership, the big idea

Rust has no garbage collector and no manual `free`. Instead there are three rules:

1. Every value has exactly **one owner** (a variable, a struct field, ...).
2. When the owner goes out of scope, the value is **dropped** (memory freed).
3. Ownership can be **moved** to someone else, or **borrowed** with a reference.

The compiler checks the rules statically. If your program compiles, a whole family of bugs (use-after-free, double free, dangling pointers, data races) cannot happen.

### Move

```rust
let a = String::from("fox");
let b = a;              // a is MOVED into b
println!("{a}");        // ERROR: borrow of moved value: `a`
```

Only one owner at a time, so after the move `a` is unusable. (Nothing was copied: the pointer to the heap text just changed hands.)

### Clone

```rust
let c = b.clone();      // a real deep copy: now there are two Strings
```

Explicit, visible, and costs memory/time. Rule of thumb: **if you type `.clone()` to silence the compiler, stop and ask whether a borrow would do.**

### Copy

Plain, small types (`i32`, `f64`, `bool`, `char`, tuples/arrays of them) are `Copy`: using them copies bits, the original stays valid. `String` is not `Copy` because copying would hide an allocation.

### Borrow

```rust
fn length_of(s: &str) -> usize { s.len() }
length_of(&e);          // lend `e` immutably; we keep ownership
```

- `&T` shared (read-only) reference. Any number may coexist.
- `&mut T` exclusive (read-write) reference. While it exists, **no other reference** to that value may exist.
- References must never outlive their owner. The compiler tracks that ("borrow checker").

This "many readers **or** one writer" rule is the heart of Rust's safety. Run `cargo run --example exp_ownership` and un-comment the `// ERROR` lines to see every message.

### Who owns what in this lesson?

| Value | Owner | Notes |
|-------|-------|-------|
| `GAMES` | the program (a constant) | `.iter()` only borrows its elements |
| `game` in the `for` loop | `&GameInfo`, a shared borrow | the array is not changed or moved |
| `game.title` | `&'static str` | copy of the pointer, not of the text |
| `game.title.to_string()` | a new `String`, created by the parent | then **moved into** the `GameCard` props |
| `title` inside `GameCard` | the component | dropped when the component is removed from the screen |

## 6. Props and ownership

Why does `GameCard` take `String` and not `&str`?

Props must be `'static`-ish: a component can be drawn again later, long after the parent function finished executing. A borrowed `&str` could point at something that no longer exists, and the compiler cannot prove it won't. So component props **own** their data (or hold `&'static` data).

Options when passing text to a component:

```rust
GameCard { title: game.title.to_string(), .. }   // explicit: you see the allocation
GameCard { title: game.title, .. }                // ERROR when the prop is `String`
#[props(into)] title: String                      // lets callers pass &str; conversion happens inside
```

We use the explicit form in these lessons because explicit code teaches. The cost is a handful of tiny allocations per render; for a game hub that is irrelevant.

Props also need `Clone + PartialEq` because Dioxus compares the old props with the new ones. If nothing changed, it skips re-rendering this component. `String` and `i32` already provide both.

## 7. `for` and iterators (first look)

```rust
for game in GAMES.iter() { ... }
```

`.iter()` produces an **iterator** that yields `&GameInfo`, one borrow at a time. Iterators are lazy, composable (`.map`, `.filter`, `.enumerate`) and as fast as hand-written loops. We use them heavily from lesson 08.

Three ways to loop over a collection:

| Code | `item` is | Collection afterwards |
|------|-----------|-----------------------|
| `for item in &list` / `list.iter()` | `&T` | untouched |
| `for item in &mut list` / `list.iter_mut()` | `&mut T` | changed in place |
| `for item in list` / `list.into_iter()` | `T` | **moved**, list unusable |

## 8. Assets

```rust
const MAIN_CSS: Asset = asset!("/assets/main.css");
...
document::Link { rel: "stylesheet", href: MAIN_CSS }
```

`asset!` checks at compile time that the file exists (a wrong path will not compile). `dx` bundles it and rewrites the URL. `document::Link` / `document::Title` / `document::Stylesheet` place elements into the page `<head>`, from anywhere in your tree.

CSS variables (`--bg`, `--accent`) are what make theming possible later (lesson 13): one wrapper class changes the variables, everything else follows.

## 9. Experiments

1. `cargo run --example exp_ownership`, then un-comment each `// ERROR` line, one at a time. Read the full message. Notice that it names the variable, the line where the move happened, and often suggests a fix.
2. In `GameGrid`, change `.iter()` to `.into_iter()` and try it. Then change `const GAMES` to `static GAMES` and try again. Which versions compile, and what does `game` become in each (`&GameInfo` or `GameInfo`)? Use the table in section 7 to explain the result. (A `const` is copied into every place it is used; a `static` is one single value that cannot be moved out of.)
3. Make `GameCard` take `title: &'static str`. It compiles. Why is that fine, while `&str` is not?
4. Add `#[derive(Clone, PartialEq)]` to `GameInfo`. Nothing changes in the page. Why would you ever want it? (Preview of lesson 04+.)
5. Remove the `{games_count}` interpolation and use `{GAMES.len()}` directly.

## 10. Key vocabulary

component, `Element`, `rsx!`, attribute macro, props, struct, field, array `[T; N]`, `const`, `'static`, ownership, move, clone, `Copy`, borrow, shared / mutable reference, drop, iterator.

## 11. Self-check

- After `let b = a;` (Strings), why can't you use `a`? What would change if they were `i32`?
- Why do component props own their data?
- What is the difference between `&str` and `String`, and when does each one appear in this lesson?
- Name the two kinds of reference and the rule that connects them.
