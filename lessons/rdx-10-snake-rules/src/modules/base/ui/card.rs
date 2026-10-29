//! `Card`: a rounded box that wraps whatever you put inside.
//!
//! WHY THIS ABSTRACTION EXISTS
//!   Problem : several pages need the same "panel with a border" look.
//!   Without : every page repeats `div { class: "card", ... }`. Rename the
//!             class once and you must hunt down every copy.
//!   With    : the class name lives here, in one place.
//!   Why good: it adds almost nothing: one div. If an abstraction cannot be
//!             explained this simply, do not add it.

use dioxus::prelude::*;

/// `children` is a special prop: everything written between the braces of
/// `Card { ... }` arrives here as one `Element`.
///
/// Ownership: the parent builds the children and MOVES them into the card.
#[component]
pub fn Card(children: Element) -> Element {
    rsx! {
        div { class: "card", {children} }
    }
}
