//! Lesson 03: the project architecture.
//!
//! `main.rs` is the CRATE ROOT. Every `mod x;` line below tells the compiler
//! "there is a module called x; load it from `x.rs` or `x/mod.rs`".
//! Files that are not declared with `mod` are NOT part of the program at all.
//!
//! The map of the project (read it top to bottom, it is also how data flows):
//!
//! ```text
//! main.rs            starts the app
//!   app.rs           root component: stylesheet + the page
//!   config/          constants of the app (names, tags)
//!   routes/          PAGES. One file per page.            (uses modules)
//!   modules/         BUILDING BLOCKS used by the pages.
//!     base/            shared by everything: layout + ui pieces
//!     games/           everything about games (data now, rules later)
//! ```
//!
//! Dependency rule: arrows only point DOWN the list.
//! routes -> modules -> config. `modules` never imports from `routes`.

mod app;
mod config;
mod modules;
mod routes;

fn main() {
    // `app::App` is the path to the component: module `app`, item `App`.
    dioxus::launch(app::App);
}
