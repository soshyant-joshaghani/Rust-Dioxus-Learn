# Lesson 01 - KNOWLEDGE (the book part)

This lesson has little Rust and a lot of "how does the machinery fit together". Understanding the machinery now saves hours of confusion later.

---

## 1. How a UI library works (the 60-second version)

Every UI library solves one problem: **keep the screen in sync with your data**.

```
 state (your data)  --render-->  description of the UI  --diff-->  real screen
       ^                                                              |
       +------------------ events (click, key) -----------------------+
```

1. You keep **state** (a counter, a game board).
2. A **component** is a function from state to a *description* of the UI ("a div containing a button labelled 3").
3. When state changes, Dioxus calls the component again, compares the new description with the old one (a **virtual DOM** diff) and touches only what differs.
4. User actions (**events**) run your code, which changes state, which goes back to step 2.

You never say "set the text of that element to 4". You change the data, and the screen follows. This is called *declarative UI*, and it is why Dioxus feels like React.

## 2. One app, many renderers

Dioxus separates "what the UI is" (core, platform independent) from "how it is drawn" (renderer):

| Renderer crate | Result |
|----------------|--------|
| `dioxus-web` | Rust -> **WebAssembly**; updates the browser DOM directly |
| `dioxus-desktop` | a native window containing a **webview** (WebView2 on Windows, WebKit on macOS/Linux); Rust runs natively next to it |
| `dioxus-mobile` | same idea on Android / iOS |

Consequence you should remember: **on desktop and mobile your Rust code is native code, not WebAssembly**. It can use `std::fs`, threads and the clock. On the web it is WebAssembly in a sandbox: no files, no threads (by default), no `SystemTime::now()`. We will meet both worlds in lesson 12 and solve them with `cfg`.

### What is WebAssembly (wasm)?

A compact binary format that browsers can run at near-native speed. Rust compiles to it with the target `wasm32-unknown-unknown` ("32-bit wasm, no OS, no vendor"). `dx` then runs `wasm-bindgen` (generates the JavaScript glue) and bundles `index.html`, the `.wasm`, and your assets.

## 3. `dx` vs `cargo`

| Tool | Knows about | Use it for |
|------|-------------|-----------|
| `cargo` | Rust only | compile, test, dependencies |
| `dx` | Rust **plus** platforms, assets, hot reload, bundling | `dx serve`, `dx build`, `dx bundle` |

`dx` is not a replacement; it *calls cargo* with the right `--target` and `--features` and then post-processes the output. A plain `cargo run` still works for the desktop feature (`cargo run --no-default-features --features desktop`), but you lose hot reload and asset handling.

## 4. Cargo features (the `[features]` table)

```toml
[features]
default = ["web"]
web     = ["dioxus/web"]
desktop = ["dioxus/desktop"]
mobile  = ["dioxus/mobile"]
```

A **feature** is a named on/off switch for optional code or optional dependency features.

- `web = ["dioxus/web"]` reads: "turning on our `web` feature turns on the `web` feature of the `dioxus` dependency".
- `default` lists features enabled when nobody asks otherwise.
- `dx serve --platform desktop` runs cargo with `--no-default-features --features desktop`. So only one renderer is compiled in.

You can also test your own code against a feature with `#[cfg(feature = "desktop")]` (conditional compilation). We use `cfg` in lessons 11-13.

## 5. Dependencies and versions

```toml
dioxus = { version = "0.7.1", features = [] }
```

- A dependency is a **crate** from <https://crates.io>. Cargo downloads it (and everything it needs) the first time.
- `"0.7.1"` is a *caret* requirement: "any version >= 0.7.1 and < 0.8.0". Semantic versioning: for `0.x` the *minor* number is the breaking one.
- **`Cargo.lock`** records the exact versions that were chosen, so builds are reproducible. For an application (our case) commit it.

The project rule for this course: **`dioxus` is the only dependency.** Where other tutorials would pull `rand`, `serde`, `chrono`, `gloo-timers`..., we write the small piece ourselves with `std`. That's slower for the project and faster for learning, because you see how the thing works.

## 6. Reading `src/main.rs` (preview, details in lesson 02)

```rust
use dioxus::prelude::*;                         // 1

const FAVICON: Asset = asset!("/assets/favicon.ico");   // 2

fn main() {
    dioxus::launch(App);                        // 3
}

#[component]
fn App() -> Element {                           // 4
    rsx! {                                      // 5
        document::Link { rel: "icon", href: FAVICON }
        Hero {}
    }
}
```

1. `use ...::*` imports the **prelude**: the handful of names almost every Dioxus file needs (`component`, `rsx!`, `Element`, `use_signal`...). `*` is a glob import: "everything public from that module".
2. `asset!` is a macro that tells `dx` "this file is part of the app". `dx` copies it into the output and gives you a URL (often with a hash in the name for cache busting). A **const** is fine because the path is known at compile time.
3. `launch(App)` picks the renderer according to the enabled cargo feature, creates the window/page, and starts the event loop. The function **never returns** while the app runs.
4. `#[component]` is an *attribute macro* on a function. A component is simply a function returning `Element`.
5. `rsx!` is the HTML-like syntax. It is a macro that turns your tree into Rust code at compile time.

## 7. Hot reload

`dx serve` watches your files. For changes inside `rsx!` (text, classes, attributes) and CSS it patches the running app without a rebuild. If you change real Rust logic it recompiles and restarts. If something looks stale after a big edit, stop and restart `dx serve`.

## 8. Android in one picture

```
 your Rust code ---(rustc, target aarch64-linux-android)---> libapp.so
                                       ^ linker + C libs from the NDK
 Kotlin/Java shell (WebView activity) --Gradle (needs JDK)--> app.apk ---adb install---> phone
```

- **SDK**: the Android platform libraries + tools (`adb`, `emulator`, build-tools).
- **NDK**: C/C++ toolchain used to link native code.
- **JDK**: Gradle (the Android build system) runs on Java. Android Studio ships a JDK called `jbr`; point `JAVA_HOME` there.
- **adb**: Android Debug Bridge, the command line connection to a device.
- Four Rust targets because phones differ: `aarch64` (almost all modern phones), `armv7` (old), `x86_64`/`i686` (emulators on PC).

When something fails, the cause is nearly always one of: wrong path in an environment variable, terminal not reopened, missing Rust target, or device not authorized.

## 9. Experiments

1. In `assets/main.css` change `#0f1116` to another colour while `dx serve` runs. Watch hot reload.
2. In `src/main.rs` change the text of one `a { ... }` link. Watch it update without a rebuild.
3. Run `dx serve --platform web`, then `dx serve --platform desktop`, and compare how long the first compile and the second compile take. Why is the second faster? (Hint: dependencies are already compiled in the shared `target/`.)
4. Open `Cargo.lock` and search for `dioxus`. How many packages is the whole dependency tree?
5. Run `dx doctor` before and after installing the Android pieces.

## 10. Key vocabulary

crate, dependency, semantic versioning, `Cargo.lock`, feature flag, `cfg`, WebAssembly, `wasm32-unknown-unknown`, renderer, webview, virtual DOM, component, hot reload, SDK, NDK, JDK, adb.

## 11. Self-check

- Which part of your app is WebAssembly and which is native on desktop?
- What does `dx serve --platform desktop` do to the cargo features?
- Why do we commit `Cargo.lock` for an app?
- What does `dioxus = "0.7.1"` allow and forbid?
