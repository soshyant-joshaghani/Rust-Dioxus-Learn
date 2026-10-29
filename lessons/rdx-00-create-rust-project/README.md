# Lesson 00 - Create a Rust project

> Goal: meet Rust and Cargo, create a project, and bend `Hello, world!` into a tiny command line program.

## What you will do

1. Install Rust (`rustup`) and check that `cargo` works.
2. Create a project with `cargo new`.
3. Run it, build it, check it.
4. Grow `Hello, world!` into **Fox Greeter**: it asks your name and birth year, then answers simple commands.

## What you will learn

| Rust idea | Where it shows up in `src/main.rs` |
|-----------|------------------------------------|
| `cargo new / run / build / check` | the workflow |
| variables, `mut`, shadowing | `let mut visits`, `let year = ...` twice |
| primitive types (`i32`, `u32`, `&str`, `String`) | everywhere |
| functions and return values | `ask`, `age_in`, `ask_year`, `greet` |
| expressions vs statements | `age_in` returns without `return` |
| `if` / `loop` / `match` | `ask_year`, the command loop |
| `Result` (first glimpse) | `text.parse::<i32>()` |

Ownership and borrowing appear only as small footnotes here (`String` vs `&str`). They get their own spotlight from lesson 02 on.

## Run it

```bash
cd lessons/rdx-00-create-rust-project
cargo run
```

Try to break it: type `abc` as the year, type an unknown command, type `quit`.

## Files

```
rdx-00-create-rust-project/
  Cargo.toml     the project manifest (name, edition, dependencies)
  src/main.rs    all the code of this lesson
```

## Challenge (optional)

Add a `"age"` command that prints the age again. Then add `"double"` that multiplies the age by two using a new function `fn double(n: i32) -> i32`.

## Read next

- `CHANGES.md`: the exact commands, nothing hidden.
- `KNOWLEDGE.md`: the "book" part. Read it with `src/main.rs` open next to it.
- Then lesson 01: [`../rdx-01-create-dioxus-project`](../rdx-01-create-dioxus-project/README.md)
