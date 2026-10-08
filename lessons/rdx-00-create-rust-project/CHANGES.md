# Lesson 00 - CHANGES

There is no previous lesson. This is the starting point.

## 1. Install the tools

Download `rustup-init.exe` from <https://rustup.rs> and run it with the default options (Windows needs the "Visual Studio C++ Build Tools" it offers to install).

Open a **new** terminal and verify:

```bash
rustc --version     # the compiler
cargo --version     # the build tool and package manager
rustup --version    # the toolchain installer / updater
```

Versions used while writing these lessons: `rustc 1.99.0`, `cargo 1.99.0`. Any recent stable works.

## 2. Create the project

```bash
cargo new rdx-00-create-rust-project
cd rdx-00-create-rust-project
```

Cargo created:

```
rdx-00-create-rust-project/
  Cargo.toml
  src/main.rs      <- contains: fn main() { println!("Hello, world!"); }
  .git/ + .gitignore (only when you are not already inside a git repository)
```

`Cargo.toml` at this point:

```toml
[package]
name = "rdx-00-create-rust-project"
version = "0.1.0"
edition = "2024"

[dependencies]
```

No dependencies. This lesson uses only the standard library.

## 3. Daily commands

```bash
cargo run        # compile + run (debug build)
cargo check      # compile without producing a program: the fastest "does it compile?"
cargo build      # compile only. Output: target/debug/<name>.exe
cargo build --release   # slow compile, fast program. Output: target/release/
cargo clippy     # extra advice from the linter (installed with rustup)
cargo fmt        # auto-format the code
```

Run `cargo run` once on the untouched project and see `Hello, world!`.

## 4. The change: `src/main.rs`

`Hello, world!` was replaced with the Fox Greeter program. Nothing else changed: `Cargo.toml` is untouched.

| Before | After |
|--------|-------|
| 3 lines, one `println!` | ~110 lines, 4 small functions + `main` |

New pieces, in order of appearance in the file:

1. `use std::io;` - bring the input/output module into scope.
2. `const APP_NAME`, `const CURRENT_YEAR` - fixed values.
3. `fn ask(question: &str) -> String` - print a question, read a line.
4. `fn age_in(year_now: i32, year_born: i32) -> i32` - plain arithmetic.
5. `fn ask_year() -> i32` - a `loop` + `match` on `parse` that repeats until valid.
6. `fn greet(name: &str, style: &str) -> String` - `match` on text.
7. `main` - ask name, ask year, shadow `year` into an age, run the command loop.

## 5. Verify

```bash
cargo run
```

Example session:

```
=== Fox Greeter ===
What is your name?
Fox
What year were you born?
abc
'abc' is not a number, try again.
What year were you born?
1990
Hello, Fox! You are about 36 years old.

Type: shout | whisper | count | quit
shout
FOX!!!
```

Also run `cargo clippy`. It should print nothing: no warnings.

## Where does this lesson end up?

Nowhere in the final game hub, on purpose. It is a warm-up. We reuse its **ideas** (parsing text into numbers, `match` on commands, a loop) in lesson 05 (Number Guessing).
