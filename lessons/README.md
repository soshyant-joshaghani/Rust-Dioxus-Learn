# Rust + Dioxus, learned by building a game hub

Fourteen small lessons that grow from `Hello, world!` into a finished **FoxG Game Hub** (Home, Games, About) with four games, saved progress, and a light/dark theme. Every lesson teaches one Rust idea **by using it** on the hub, and every lesson is a complete, runnable crate.

The whole course uses **one dependency: `dioxus`**. Everything else is the Rust standard library.

## How to use a lesson

Each folder has three documents and a runnable snapshot:

| File | Purpose | Read it |
|------|---------|---------|
| `README.md` | what you build and learn, how to run, a challenge (short) | first |
| `CHANGES.md` | the exact commands and every file change since the previous lesson | while you type |
| `KNOWLEDGE.md` | the "book": explanations, diagrams, ownership notes, experiments, self-check | with the code open beside it |
| `src/`, `examples/` | the code of that lesson, heavily commented | always |

Suggested loop: read `README` -> follow `CHANGES` yourself in a copy of the previous lesson (or just read the diff) -> read `KNOWLEDGE` -> do two experiments -> answer the self-check.

## The path

| # | Folder | You build | Main Rust ideas |
|---|--------|-----------|-----------------|
| 00 | [rdx-00-create-rust-project](rdx-00-create-rust-project/README.md) | a tiny command-line greeter | cargo, variables, functions, `match`, `loop`, `Result` (a glimpse) |
| 01 | [rdx-01-create-dioxus-project](rdx-01-create-dioxus-project/README.md) | the template app running on web, desktop, Android | crates, features, wasm, the toolchain |
| 02 | [rdx-02-components-and-rsx](rdx-02-components-and-rsx/README.md) | first page and game cards | components, `rsx!`, structs, **ownership**, props |
| 03 | [rdx-03-modules-and-architecture](rdx-03-modules-and-architecture/README.md) | the real folder layout | modules, `pub`, `use`, dependency direction |
| 04 | [rdx-04-routes-and-enums](rdx-04-routes-and-enums/README.md) | Home / Games / About + router | **enums**, `match`, `Option`, `Result`, `derive` |
| 05 | [rdx-05-signals-and-closures](rdx-05-signals-and-closures/README.md) | **Number Guessing** (first game) | signals, **closures**, `async` basics, `Ordering` |
| 06 | [rdx-06-structs-results-and-tests](rdx-06-structs-results-and-tests/README.md) | rules split from the page | `impl`, custom errors, `?`, `From`, **tests** |
| 07 | [rdx-07-rock-paper-scissors](rdx-07-rock-paper-scissors/README.md) | **Rock Paper Scissors** | tuple patterns, `Copy` vs `Clone`, `Default` |
| 08 | [rdx-08-tic-tac-toe](rdx-08-tic-tac-toe/README.md) | **Tic-Tac-Toe** | arrays, slices, **borrowing**, `&mut`, `RefCell` |
| 09 | [rdx-09-traits-and-generics](rdx-09-traits-and-generics/README.md) | a shared `Game` trait, `use_seeded` | **traits**, generics, `dyn`, `'static` |
| 10 | [rdx-10-snake-rules](rdx-10-snake-rules/README.md) | **Snake** rules, stepped by hand | `VecDeque`, iterators, state machines |
| 11 | [rdx-11-snake-loop](rdx-11-snake-loop/README.md) | Snake in real time | `async`/timing, keyboard, threads and channels |
| 12 | [rdx-12-persistence](rdx-12-persistence/README.md) | saved progress | `std::fs`, `cfg`, your own trait, context |
| 13 | [rdx-13-the-hub](rdx-13-the-hub/README.md) | theme + About: the finished hub | lifetimes, smart pointers, architecture review |

## Experiments (`examples/`)

Tiny, std-only labs. They accumulate: lesson 13 contains all of them. Run one with `cargo run --example <name>`.

| Experiment | Introduced in | Shows |
|------------|---------------|-------|
| `exp_ownership` | 02 | move, clone, copy, borrow |
| `exp_modules` | 03 | privacy and paths |
| `exp_enums` | 04 | enums, `match`, `Option`, `Result` |
| `exp_closures` | 05 | capture, `move`, `Fn`/`FnMut`/`FnOnce` |
| `exp_errors` | 06 | error types, `From`, `?` |
| `exp_copy_clone` | 07 | `Copy` vs `Clone` vs move |
| `exp_borrowing` | 08 | shared/mutable borrows, slices, `RefCell` |
| `exp_traits_generics` | 09 | traits, generics, static vs dynamic dispatch |
| `exp_collections` | 10 | `Vec`, `VecDeque`, `HashMap`, `HashSet`, iterators |
| `exp_threads_channels` | 11 | threads, `mpsc`, `Arc<Mutex<T>>` |
| `exp_file_io` | 12 | `std::fs`, paths, `io::Error` |
| `exp_lifetimes` | 13 | lifetimes |
| `exp_smart_pointers` | 13 | `Box`, `Rc`, `RefCell` |

## Layout of this folder

```
rust-dioxus-learn/            the FINISHED hub (same as lesson 13)
  Cargo.toml  Dioxus.toml  assets/  src/  examples/
  .cargo/config.toml          one shared target/ folder for every lesson (compile Dioxus once)
  lessons/
    README.md                 this file
    rdx-00-... to rdx-13-...  one crate per lesson
```

## Commands you will use all the time

```bash
cargo check                       # compile without running: the fastest feedback
cargo clippy --all-targets        # the linter; the lessons keep it silent
cargo test                        # unit tests (no browser needed)
dx serve                          # web with hot reload
dx serve --platform desktop       # native window
dx serve --platform android       # emulator or phone (lesson 01 sets it up)
cargo run --example exp_ownership # any experiment
```

## Honest notes

- Written and checked on Windows 11 with `rustc 1.99`, `dx 0.7.10`, Dioxus `0.7.10`.
- Lessons 02-13 were compiled, linted (`cargo clippy --all-targets`) and tested (`cargo test`); the games and the saved progress were exercised in a real browser. The Android setup (lesson 01) follows the official guide but could **not** be run on the author's machine, and neither were the desktop/Android runs of saved files.
- The first compile of any lesson is slow (about 175 crates); later ones reuse the shared `target/` folder.
