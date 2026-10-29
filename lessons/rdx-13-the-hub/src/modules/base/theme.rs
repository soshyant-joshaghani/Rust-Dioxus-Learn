//! Light / dark theme: a setting that is shared by the whole app and saved.
//!
//! It reuses everything from lesson 12:
//!
//! ```text
//!   Settings --impl Persist--> save_value("settings", ..) / load_value("settings")
//!   ThemeStore (Copy, holds a Signal) --use_context_provider--> every component
//! ```
//!
//! The challenge of lesson 12 asked: "how many lines of storage code does a second
//! saved type need?" Answer: none. The `Persist` trait paid for itself.

use dioxus::prelude::*;

use crate::modules::base::storage::{self, ParseError, Persist};

const SAVE_NAME: &str = "settings";

// ---------------------------------------------------------------------------
// Data
// ---------------------------------------------------------------------------

/// Which colours to use. `Default` = `Dark` (the `#[default]` marker).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    Dark,
    Light,
}

impl Theme {
    pub fn toggled(self) -> Theme {
        match self {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        }
    }

    /// The CSS class that switches the colour variables (see `assets/main.css`).
    pub fn css_class(self) -> &'static str {
        match self {
            Theme::Dark => "dark",
            Theme::Light => "light",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Settings {
    pub theme: Theme,
}

impl Persist for Settings {
    fn to_text(&self) -> String {
        format!("theme={}\n", self.theme.css_class())
    }

    fn from_text(text: &str) -> Result<Settings, ParseError> {
        let mut settings = Settings::default();

        for line in text.lines().map(str::trim).filter(|line| !line.is_empty()) {
            let (key, value) = line.split_once('=').ok_or_else(|| ParseError::MissingEquals(line.to_string()))?;

            match (key.trim(), value.trim()) {
                ("theme", "dark") => settings.theme = Theme::Dark,
                ("theme", "light") => settings.theme = Theme::Light,
                ("theme", other) => {
                    return Err(ParseError::InvalidValue { key: "theme".to_string(), value: other.to_string() });
                }
                _ => {} // unknown key: ignore, as in Progress
            }
        }
        Ok(settings)
    }
}

// ---------------------------------------------------------------------------
// The shared store
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct ThemeStore {
    settings: Signal<Settings>,
}

impl ThemeStore {
    pub fn get(&self) -> Theme {
        self.settings.read().theme
    }

    /// Flip the theme and save the settings.
    pub fn toggle(mut self) {
        let next = Settings { theme: self.get().toggled() };
        self.settings.set(next);

        // Saving can fail (disk full...). The theme still changed on screen, so we
        // do not interrupt the player; a failure is simply not remembered.
        spawn(async move {
            let _ = storage::save_value(SAVE_NAME, &next).await;
        });
    }
}

/// Call once in `App`: creates the store, provides it, loads the saved theme.
pub fn use_theme_provider() -> ThemeStore {
    let mut settings = use_signal(Settings::default);
    let store = use_context_provider(|| ThemeStore { settings });

    use_future(move || async move {
        // `if let Ok(Some(saved))` handles "loaded something" and ignores both
        // "nothing saved" and "could not load": the default theme is a fine fallback.
        if let Ok(Some(saved)) = storage::load_value::<Settings>(SAVE_NAME).await {
            settings.set(saved);
        }
    });

    store
}

pub fn use_theme() -> ThemeStore {
    use_context::<ThemeStore>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggling_twice_returns_to_the_start() {
        assert_eq!(Theme::Dark.toggled().toggled(), Theme::Dark);
        assert_eq!(Theme::Dark.toggled(), Theme::Light);
    }

    #[test]
    fn settings_roundtrip() {
        let s = Settings { theme: Theme::Light };
        assert_eq!(Settings::from_text(&s.to_text()), Ok(s));
        assert_eq!(Settings::from_text(&Settings::default().to_text()), Ok(Settings::default()));
    }

    #[test]
    fn empty_settings_use_the_default_theme() {
        assert_eq!(Settings::from_text(""), Ok(Settings::default()));
    }

    #[test]
    fn an_unknown_theme_is_an_error() {
        assert_eq!(
            Settings::from_text("theme=purple"),
            Err(ParseError::InvalidValue { key: "theme".to_string(), value: "purple".to_string() })
        );
    }

    #[test]
    fn unknown_keys_are_ignored() {
        assert_eq!(Settings::from_text("volume=3\ntheme=light\n").unwrap().theme, Theme::Light);
    }
}
