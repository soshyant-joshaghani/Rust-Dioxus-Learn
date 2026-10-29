//! The button in the header that switches between dark and light.

use dioxus::prelude::*;

use crate::modules::base::theme::{use_theme, Theme};

#[component]
pub fn ThemeToggle() -> Element {
    let theme = use_theme();

    // The label shows what a click will switch TO.
    let label = match theme.get() {
        Theme::Dark => "Light",
        Theme::Light => "Dark",
    };

    rsx! {
        button {
            class: "btn secondary toggle",
            onclick: move |_| theme.toggle(), // `theme` is Copy, so it is usable in the closure
            "{label}"
        }
    }
}
