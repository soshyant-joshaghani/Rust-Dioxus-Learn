# Lesson 01 - Create a Dioxus project

> Goal: understand what Dioxus is, create a project with the `dx` tool, and run the **same untouched app** on the web, on the desktop, and on an Android device.

We do **not** change any code in this lesson. The point is a working toolchain on every target. If this lesson works, every later lesson only needs `dx serve`.

## What is Dioxus?

Dioxus is a Rust library for building user interfaces, in the spirit of React: you write *components* (functions) that return a tree of HTML-like elements, and Dioxus updates the screen when your *state* changes.

One codebase, several **renderers**:

| Platform | How it shows the UI | Cargo feature |
|----------|---------------------|---------------|
| Web | compiled to WebAssembly, runs in the browser | `web` |
| Desktop | a native window with a system webview | `desktop` |
| Mobile (Android / iOS) | a native app with a system webview | `mobile` |

`dx` is the Dioxus command line tool: it creates projects, compiles them for the right target, serves them with hot reload, and bundles installers.

## What you will do

1. Install `dx` and the web target.
2. `dx new` a project.
3. `dx serve` it as **web**, then **desktop**.
4. Install the Android SDK + NDK and `dx serve --platform android`.

## Quick commands

```bash
cargo install dioxus-cli            # once (or: cargo binstall dioxus-cli)
rustup target add wasm32-unknown-unknown

cd lessons/rdx-01-create-dioxus-project
dx serve                            # web (default feature), opens http://localhost:8080
dx serve --platform desktop         # native window
dx serve --platform android         # emulator or phone (needs the Android setup)
```

Details, every command, and every environment variable: `CHANGES.md`.

## Files you get from `dx new`

```
rdx-01-create-dioxus-project/
  Cargo.toml     dependency on `dioxus` + one feature per platform
  Dioxus.toml    settings for the dx tool (app name, web title...)
  assets/        favicon, a picture, main.css
  src/main.rs    the whole app: `App` and `Hero` components
```

Read `src/main.rs` once, but don't worry about understanding `rsx!` yet. Lesson 02 takes it apart.

## Done when

- [ ] `dx serve` shows the Dioxus welcome page in your browser.
- [ ] `dx serve --platform desktop` shows the same page in a window.
- [ ] `dx serve --platform android` shows it on an emulator/device. *(If Android is not available on your machine yet, finish web + desktop and come back; nothing in the next lessons depends on it.)*

## Read next

`CHANGES.md` for the exact steps, `KNOWLEDGE.md` for how it all fits together, then [lesson 02](../rdx-02-components-and-rsx/README.md).
