//! Lesson 02: components and `rsx!`.
//!
//! The page is built from small functions called COMPONENTS:
//!
//!   App
//!   |-- Header
//!   |-- Hero
//!   |-- GameGrid
//!   |   `-- GameCard   (one per game, repeated by a `for` loop)
//!   `-- Footer
//!
//! Everything lives in this one file on purpose. Lesson 03 splits it into modules.

use dioxus::prelude::*;

// `asset!` tells `dx` that this file belongs to the app. The value is a small
// handle (`Asset`) that we pass to the page, and dx fills in the final URL.
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

// ---------------------------------------------------------------------------
// Data
// ---------------------------------------------------------------------------

/// What we know about one game. A `struct` groups named values ("fields").
///
/// `&'static str` is text that lives for the whole program (string literals
/// written in the source code are of this kind), so the struct needs no
/// ownership logic: it just points at text baked into the program.
struct GameInfo {
    title: &'static str,
    blurb: &'static str,
    concept: &'static str, // the Rust idea this game will teach us
}

/// The games of the hub. A `const` array: fixed size (4), fixed content.
/// Only the info exists today, the games themselves arrive in later lessons.
const GAMES: [GameInfo; 4] = [
    GameInfo { title: "Number Guessing", blurb: "Find the secret number.", concept: "Result + match" },
    GameInfo { title: "Rock Paper Scissors", blurb: "Beat the computer.", concept: "enums" },
    GameInfo { title: "Tic-Tac-Toe", blurb: "Three in a row.", concept: "arrays + &mut" },
    GameInfo { title: "Snake", blurb: "Eat, grow, do not crash.", concept: "collections + loop" },
];

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() {
    // `launch` starts the renderer chosen by the cargo feature (web/desktop/mobile)
    // and keeps running until the window or tab is closed.
    dioxus::launch(App);
}

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

/// The root component. It has no inputs; it only assembles the page.
///
/// A component is a function that returns `Element` ("a piece of UI").
/// `#[component]` is an attribute macro that prepares the function so it can
/// be written like a tag: `App {}`.
#[component]
fn App() -> Element {
    // `rsx!` looks like HTML but it is Rust code checked by the compiler.
    // A typo in an attribute name is a compile error, not a silent bug.
    rsx! {
        // `document::Link` puts a <link> into the page <head>.
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        // Our own components are used exactly like HTML tags, but start with
        // a capital letter. That is how rsx tells them apart from HTML.
        div { class: "page",
            Header {}
            Hero {}
            GameGrid {}
            Footer {}
        }
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        div { class: "header",
            span { class: "brand", "FoxG Game Hub" }
            span { class: "tag", "lesson 02" }
        }
    }
}

#[component]
fn Hero() -> Element {
    // A plain Rust value used inside the markup with `{name}`.
    let games_count = GAMES.len();

    rsx! {
        div { class: "hero",
            h1 { "Learn Rust by playing" }
            // Text in rsx is a string literal. `{games_count}` is replaced by the value,
            // exactly like in `println!("{games_count}")`.
            p { "{games_count} small games, one Rust idea each." }
        }
    }
}

#[component]
fn GameGrid() -> Element {
    rsx! {
        div { class: "grid",
            // A normal Rust `for` loop, written inside the markup.
            //
            // `GAMES.iter()` BORROWS every element (type: &GameInfo).
            // Nothing is copied or moved; GAMES stays intact.
            for game in GAMES.iter() {
                // Props are passed like HTML attributes. Each prop gets an OWNED value:
                // `.to_string()` makes a new String from the &'static str.
                // (Why owned? See KNOWLEDGE.md section "Props and ownership".)
                GameCard {
                    title: game.title.to_string(),
                    blurb: game.blurb.to_string(),
                    concept: game.concept.to_string(),
                }
            }
        }
    }
}

/// A component WITH inputs ("props").
///
/// The function arguments become the props. Dioxus generates a hidden struct
/// `GameCardProps { title, blurb, concept }` from them. Props must be
/// `Clone + PartialEq`, because Dioxus compares old and new props to decide
/// whether this component needs to be drawn again.
///
/// The Strings are OWNED by the component: the parent created them and
/// MOVED them in. When the component is gone, they are freed.
#[component]
fn GameCard(title: String, blurb: String, concept: String) -> Element {
    rsx! {
        div { class: "card",
            h3 { "{title}" }
            p { "{blurb}" }
            span { class: "tag", "{concept}" }
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        div { class: "footer", "Built with Rust and Dioxus." }
    }
}
