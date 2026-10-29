# Lesson 13 - CHANGES

Previous: lesson 12 (four games + saved progress).

## Commands

```bash
cp -r rdx-12-persistence rdx-13-the-hub
cd rdx-13-the-hub
# add/edit files below, then
cargo clippy --all-targets
cargo test
cargo check --target wasm32-unknown-unknown
dx serve
cargo run --example exp_lifetimes
cargo run --example exp_smart_pointers
```

## Files

| File | Change |
|------|--------|
| `Cargo.toml` | name only. **Dependencies of the whole course: `dioxus` (feature `router`). Nothing else.** |
| `src/config/mod.rs` | `LESSON_TAG` removed; added `VERSION` and `platform_name()` |
| `src/modules/base/theme.rs` | **new**: `Theme`, `Settings`, `impl Persist for Settings`, `ThemeStore`, `use_theme_provider`, `use_theme`, 5 tests |
| `src/modules/base/theme_toggle.rs` | **new**: `ThemeToggle` button |
| `src/modules/base/mod.rs` | `pub mod theme; pub mod theme_toggle;` |
| `src/modules/base/header.rs` | shows `v{VERSION}` instead of the lesson tag; renders `ThemeToggle {}` |
| `src/modules/base/storage.rs` | `ParseError` variants renamed to `MissingEquals`, `NotANumber`, `InvalidValue` (new). `progress.rs` follows. |
| `src/app.rs` | `use_theme_provider()`; the router is wrapped in `div { class: "app dark|light" }` |
| `src/routes/about.rs` | **rewritten**: intro, version/platform/data line, what each game teaches, where to read more |
| `assets/main.css` | `.app`, `.app.light` (variables), `.toggle`, `code` |
| `examples/exp_lifetimes.rs`, `exp_smart_pointers.rs` | **new** |

## The theme in four lines

```rust
// app.rs
let theme = use_theme_provider();
rsx! { div { class: "app {theme.get().css_class()}", Router::<Route> {} } }
```
```css
.app.light { --bg: #f4f5f9; --panel: #ffffff; --text: #1b1f2a; ... }
```
Every colour in the stylesheet is `var(--name)`, so switching the class re-colours everything. The file `settings.txt` contains `theme=light`.

## Why the enum variants were renamed

clippy (`enum_variant_names`): when **all** variants of an enum start with the same word (`BadLine`, `BadNumber`, `BadValue`), the prefix says nothing. In lesson 12 two variants did not trigger it; the third one did. Names now describe the *problem*: `MissingEquals`, `NotANumber`, `InvalidValue`.

## Verified in a real browser while writing this lesson

1. Fresh start: wrapper class `app dark`.
2. `/about`: shows `Version 0.1.0 | running on web | data: browser storage (localStorage)`.
3. Click the toggle: class becomes `app light`; `localStorage["foxg:settings"]` is `theme=light`.
4. Reload: still light (screenshot taken), toggle now labelled "Dark".

Also: `cargo test` 63 passed; clippy silent; `cargo check --target wasm32-unknown-unknown` silent; both experiments run.

## Not verified

- Desktop and Android **runs** with saved files (only compiled/tested at the function level, see lesson 12).
- `dx bundle` release packaging.
