# Lesson 00 - KNOWLEDGE (the book part)

Open `src/main.rs` beside this file. Every section points at real lines.

---

## 1. What is Rust?

Rust is a **compiled, statically typed** language. Two sentences explain why people like it:

- It is as fast as C/C++ because there is no garbage collector and no hidden runtime.
- It is safe because the **compiler** refuses programs that could crash in memory-related ways (use after free, two writers at once, null pointers...).

The price: the compiler is strict and talks a lot. **Compiler errors are the teacher.** They are long, but they usually contain the fix. Read them from the top.

Three words you will hear constantly:

| Word | Meaning |
|------|---------|
| **crate** | one compilation unit: a program (binary crate) or a library (library crate). Our project is one crate. |
| **package** | what `Cargo.toml` describes: one or more crates plus metadata. |
| **module** | a namespace *inside* a crate (lesson 03). |

## 2. What is Cargo?

Cargo is three tools in one:

1. **Project creator**: `cargo new`.
2. **Build tool**: `cargo build / run / check / test`.
3. **Package manager**: downloads libraries ("crates") from <https://crates.io> that you list under `[dependencies]`.

### `Cargo.toml`

```toml
[package]
name = "rdx-00-create-rust-project"   # also the name of the .exe
version = "0.1.0"
edition = "2024"                      # which "dialect" of Rust (see below)

[dependencies]                        # empty: only the standard library
```

**Edition**: every ~3 years Rust ships an edition (2015, 2018, 2021, 2024) that may change syntax rules. Editions are *opt-in per crate* and crates of different editions work together. It is not a different language, it is a compatibility switch.

### What is in `target/`?

All build output. It can be huge and is safe to delete (`cargo clean`). It is never committed to git (see `.gitignore`).

### `check` vs `build` vs `run`

`cargo check` stops after type checking, which is where 95% of your errors are. Use it constantly; use `run` when you want to see the program.

### Debug vs release

`cargo build` is *debug*: fast to compile, slow program, extra safety checks (e.g. integer overflow panics). `--release` is the opposite. When your game feels slow, first check you are not measuring a debug build.

## 3. `fn main`

```rust
fn main() { ... }
```

Every binary crate starts executing here. `fn` declares a function, `()` is its parameter list (none), `{}` is the body. `main` returns nothing, which Rust writes as `()` - the **unit type**, an empty tuple. (Later lessons make `main`-like functions return `Result`.)

`println!` ends with `!` because it is a **macro**, not a function. Macros run at compile time and can do things functions cannot, such as accept any number of arguments and check the format string while compiling. You will meet two more soon: `format!` and `rsx!` (Dioxus).

## 4. Variables

```rust
let name = ask("What is your name?");   // immutable
let mut visits: u32 = 0;                // mutable, with an explicit type
```

- `let` creates a binding.
- **Immutable by default.** `name = ...` later would be a compile error. This is deliberate: when you read code, anything without `mut` is guaranteed to stay the same.
- `mut` is a visible warning flag: "this value changes".
- Types are **inferred** most of the time. Write them when the compiler cannot guess (`parse::<i32>()`) or to document intent (`: u32`).

### Shadowing

```rust
let year = ask_year();               // year is the birth year
let year = age_in(CURRENT_YEAR, year); // a NEW variable, also called year, now the age
```

Shadowing creates a new variable with the same name. The old one is hidden, not changed. It is allowed to change the type too (`let x = "5"; let x: i32 = x.parse().unwrap();`). Use it for step-by-step transformation; use `mut` when the *same* thing is updated in a loop.

### Constants

`const NAME: Type = value;` Always needs a type, is evaluated at compile time, can be declared anywhere (also outside functions), and is never `mut`.

## 5. Primitive types

| Type | Meaning | Note |
|------|---------|------|
| `i32` | signed 32-bit integer | the default integer type |
| `u32` | unsigned (>= 0) 32-bit | `u8 u16 u32 u64 u128 usize`, `i8 ... isize` |
| `f64` | 64-bit float | default float type |
| `bool` | `true` / `false` | |
| `char` | one Unicode character, in single quotes `'a'` | 4 bytes |
| `&str` | borrowed text (string slice) | `"hello"` literals are `&'static str` |
| `String` | owned, growable text | heap allocated |
| `()` | unit, "nothing" | |

`usize` is "an integer as large as a memory address". Indexing collections uses `usize`. You will see it from lesson 08.

Rust **never converts numbers silently**: `let a: i32 = 5; let b: i64 = a;` is an error. Write `i64::from(a)` or `a as i64`. This prevents a whole class of bugs.

### `String` vs `&str` (a first glimpse)

- `String` **owns** its text. You can grow it. When it goes out of scope, the memory is freed.
- `&str` is a **view** into text that lives somewhere else. Cheap to pass around, cannot be grown.

That is why `fn ask(question: &str) -> String`: it only *looks at* the question (borrow), but *creates* the answer so it must give an owned value back. The full story is lesson 02/03; for now remember: **parameters usually take `&str`, results that were created inside usually are `String`.**

## 6. Functions and expressions

```rust
fn age_in(year_now: i32, year_born: i32) -> i32 {
    year_now - year_born
}
```

- Parameter types are **required**. Return type follows `->`.
- **Rust is expression-based.** An expression produces a value; a statement does not. A block `{ ... }` is an expression whose value is its *last expression without a semicolon*.
- Adding a `;` to the last line turns it into a statement and the function would return `()`, which gives a type mismatch error. This is a classic first-day mistake. Try it: add `;` after `year_now - year_born`.
- `return x;` exists for early exits (see `ask_year`).

`if` and `match` are also expressions:

```rust
let kind = if age >= 18 { "adult" } else { "minor" };   // both branches same type
```

## 7. Control flow

| Construct | Used in this lesson |
|-----------|---------------------|
| `if / else` | (see the challenge) |
| `loop { ... break; }` | `ask_year`, the command loop. `loop` can also `break value`. |
| `while cond { }` | (later) |
| `for x in collection { }` | (later) |
| `match value { pattern => result, ... }` | `ask_year`, `greet`, the command loop |

### `match` is the star of Rust

- It must be **exhaustive**: every possible value is covered, or the code does not compile. That's why we need `_` / `other =>` at the end for text (infinite possibilities).
- `"shout" | "whisper"` matches either.
- `other => ...` binds the unmatched value to a name; `_` ignores it.
- Each arm is an expression; all arms must have the same type.

Why `command.as_str()`? `command` is a `String`; patterns like `"quit"` are `&str`. `as_str()` borrows the String as `&str` for the comparison. Without it you get a type mismatch error. A great first error to read.

## 8. `Result`: a first look

```rust
match text.parse::<i32>() {
    Ok(year) => return year,
    Err(_)   => println!("'{text}' is not a number, try again."),
}
```

`parse` can fail ("abc" is not a number), so instead of crashing or returning a magic value like `-1`, it returns `Result<i32, ParseIntError>`:

```rust
enum Result<T, E> { Ok(T), Err(E) }
```

It is an *enum* (lesson 04): a value that is **either** `Ok` carrying a number **or** `Err` carrying an error. You cannot use the number without first saying what to do in the error case. That's the design: **errors are values, and the compiler makes you look at them.**

Contrast `.expect("message")` in `ask`: "if Err, crash with this message". Handy while learning, and the honest label ("expect") tells readers a crash is possible. Lesson 06 builds proper error types.

## 9. Macros you used

- `println!("text {name} {1 + 1:?}")` - `{name}` inserts a variable by name. `{:?}` uses *debug* formatting (shows quotes for strings). `{}` alone takes the next argument.
- `format!(...)` - same, but returns a `String` instead of printing.

## 10. Ownership: the tiny footnote

In `ask`:

```rust
let mut line = String::new();             // `line` OWNS an empty String
io::stdin().read_line(&mut line)...       // lend it mutably, read_line fills it, we get it back
line.trim().to_string()                   // build a new owned copy, return it (move to the caller)
```

Who owns what, when:

| Moment | Owner |
|--------|-------|
| after `String::new()` | `line` (inside `ask`) |
| during `read_line(&mut line)` | still `line`; `read_line` has a temporary **mutable borrow** |
| after `to_string()` | the new String is owned by whoever receives the return value (`name` in `main`) |
| end of `ask` | `line` is dropped (memory freed automatically) |

You never wrote `free`. You never will. That is ownership at work, and lesson 02 opens it up fully.

## 11. Experiments

1. Remove `mut` from `let mut visits`. Read the error. The compiler even suggests the fix.
2. Add a semicolon to the last line of `age_in`. Read the error.
3. Delete the `_ =>` arm in `greet`. Read the error about non-exhaustive patterns.
4. In `main`, change `greet(&name, "normal")` to `greet(name, "normal")` (remove the `&`). Read how the compiler explains the type mismatch between `String` and `&str`.
5. `cargo build --release`, then compare the size of `target/debug/*.exe` and `target/release/*.exe`.

## 12. Key vocabulary

crate, package, edition, macro, binding, mutability, shadowing, expression, statement, unit `()`, `Result`, `match`, exhaustive.

## 13. Self-check

- Why does `let x = 5; x = 6;` fail, and what is the smallest change that fixes it?
- What is the difference between shadowing and mutation?
- Why does `ask` return `String` rather than `&str`?
- What happens if you forget a case in a `match`?
