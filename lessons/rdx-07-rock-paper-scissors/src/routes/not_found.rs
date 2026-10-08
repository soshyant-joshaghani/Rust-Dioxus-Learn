//! Any URL that no other route matches.

use dioxus::prelude::*;

use crate::routes::Route;

/// The prop `segments` comes from the URL: "/a/b" -> ["a", "b"].
/// It is a `Vec<String>`: a growable list of owned Strings.
#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    // `join` builds ONE new String out of the pieces, separated by "/".
    // `segments` is only borrowed for this call, so it is still usable afterwards.
    let path = segments.join("/");

    rsx! {
        div { class: "hero",
            h1 { "Not found" }
            p { "There is no page at /{path}" }
            Link { to: Route::Home {}, class: "btn", "Back home" }
        }
    }
}
