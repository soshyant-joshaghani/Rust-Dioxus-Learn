//! Progress: what the player has achieved, and how it is saved.
//!
//! TWO PARTS
//!
//! 1. `Progress` is plain data + rules ("best of", counters) + its text format.
//! 2. `ProgressStore` is the Dioxus side: one shared copy for the whole app,
//!    loaded at start, saved after every change.
//!
//! THE FLOW
//!
//! ```text
//!   app start
//!      |  use_progress_provider()
//!      v
//!   load "progress" ---> Ok(Some(text)) --> Progress::from_text --> data signal
//!                   `--> Ok(None)       --> keep Progress::default()   (first run)
//!                   `--> Err(e)         --> notice signal: show the problem
//!
//!   a game ends
//!      |  store.update(|p| p.record_guess(5))
//!      v
//!   change the data signal --> spawn { save "progress" } --Err--> notice signal
//! ```
//!
//! THE FILE FORMAT (so you can open and read it yourself):
//!
//! ```text
//!   guess_best_tries=5
//!   rps_rounds=12
//!   tictactoe_games=3
//!   snake_best=7
//! ```

use dioxus::prelude::*;

use crate::modules::base::storage::{self, ParseError, Persist};

/// The name under which progress is saved (file `progress.txt`, or key `foxg:progress`).
const SAVE_NAME: &str = "progress";

// ---------------------------------------------------------------------------
// Part 1: the data
// ---------------------------------------------------------------------------

/// `Copy`: four small numbers. Reading the progress hands out a copy, so nobody
/// can change the stored value by accident.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Progress {
    /// Fewest tries needed to win the guessing game. `None` = never won.
    pub guess_best_tries: Option<u32>,
    /// Rock Paper Scissors rounds played, all time.
    pub rps_rounds: u32,
    /// Tic-Tac-Toe games finished (won or drawn).
    pub tictactoe_games: u32,
    /// Highest Snake score.
    pub snake_best: u32,
}

impl Progress {
    /// Record a won guessing game. Keeps the SMALLER number of tries.
    pub fn record_guess(&mut self, tries: u32) {
        self.guess_best_tries = Some(match self.guess_best_tries {
            Some(best) => best.min(tries),
            None => tries,
        });
    }

    pub fn record_rps_round(&mut self) {
        self.rps_rounds += 1;
    }

    pub fn record_tictactoe_game(&mut self) {
        self.tictactoe_games += 1;
    }

    /// Record a finished Snake game. Keeps the LARGER score.
    pub fn record_snake(&mut self, score: u32) {
        self.snake_best = self.snake_best.max(score);
    }
}

/// The text format. `Persist` is defined in `modules/base/storage.rs`.
impl Persist for Progress {
    fn to_text(&self) -> String {
        let mut text = String::new();
        // `if let` writes the line only when there IS a best.
        if let Some(best) = self.guess_best_tries {
            text.push_str(&format!("guess_best_tries={best}\n"));
        }
        text.push_str(&format!("rps_rounds={}\n", self.rps_rounds));
        text.push_str(&format!("tictactoe_games={}\n", self.tictactoe_games));
        text.push_str(&format!("snake_best={}\n", self.snake_best));
        text
    }

    /// Reading is LENIENT on purpose:
    ///
    /// - missing keys keep their default (an old save file still loads),
    /// - unknown keys are ignored (a newer save file still loads),
    /// - blank lines are skipped.
    ///
    /// It is STRICT where it matters: a line without `=` or a value that is not a
    /// number is an error. We do not guess what damaged data meant.
    fn from_text(text: &str) -> Result<Progress, ParseError> {
        let mut progress = Progress::default();

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // `split_once('=')` -> Option<(&str, &str)>. `ok_or_else` turns None into our error.
            let (key, value) = line.split_once('=').ok_or_else(|| ParseError::MissingEquals(line.to_string()))?;

            // Parse the value once; the same error for every numeric key.
            let number: u32 = value.trim().parse().map_err(|_| ParseError::NotANumber {
                key: key.trim().to_string(),
                value: value.trim().to_string(),
            })?;

            match key.trim() {
                "guess_best_tries" => progress.guess_best_tries = Some(number),
                "rps_rounds" => progress.rps_rounds = number,
                "tictactoe_games" => progress.tictactoe_games = number,
                "snake_best" => progress.snake_best = number,
                _ => {} // unknown key: ignore
            }
        }
        Ok(progress)
    }
}

// ---------------------------------------------------------------------------
// Part 2: the shared store
// ---------------------------------------------------------------------------

/// Handle to the app-wide progress. `Copy`, because it only holds two signals
/// (which are `Copy` handles): pass it into any closure without `.clone()`.
#[derive(Clone, Copy)]
pub struct ProgressStore {
    data: Signal<Progress>,
    notice: Signal<Option<String>>, // a problem to show the player, if any
}

impl ProgressStore {
    /// The current progress (a copy).
    pub fn get(&self) -> Progress {
        *self.data.read()
    }

    /// The latest storage problem, if any.
    pub fn notice(&self) -> Option<String> {
        self.notice.read().clone() // `Option<String>` is not Copy: clone out of the guard
    }

    /// Change the progress and save it.
    ///
    /// `change` is a closure that receives `&mut Progress`. `impl FnOnce(...)`: any
    /// closure that will be called exactly once. `mut self` is fine: `ProgressStore`
    /// is `Copy`, so this does not use up the caller's handle.
    pub fn update(mut self, change: impl FnOnce(&mut Progress)) {
        {
            // The write guard lives only inside these braces...
            let mut guard = self.data.write();
            change(&mut guard);
        }
        // ...so reading the signal again to save it is safe.
        self.save();
    }

    fn save(mut self) {
        let value = self.get(); // a copy: no guard is alive while we await below
        spawn(async move {
            match storage::save_value(SAVE_NAME, &value).await {
                Ok(()) => self.notice.set(None),
                Err(e) => self.notice.set(Some(format!("Could not save progress: {e}"))),
            }
        });
    }
}

/// Call ONCE, near the top of the app. Creates the shared store, makes it available
/// to every component below (Dioxus "context"), and loads the saved progress.
pub fn use_progress_provider() -> ProgressStore {
    let mut data = use_signal(Progress::default);
    let mut notice = use_signal(|| None::<String>);

    // `use_context_provider` stores the value so that any DESCENDANT component can
    // fetch it with `use_context`. It is like a global variable that is only visible
    // to the part of the tree below this point.
    let store = use_context_provider(|| ProgressStore { data, notice });

    // Load once, at start. A task tied to this component (see lesson 11).
    use_future(move || async move {
        match storage::load_value::<Progress>(SAVE_NAME).await {
            Ok(Some(saved)) => data.set(saved),
            Ok(None) => {} // first run: nothing saved yet
            Err(e) => notice.set(Some(format!("Could not load progress: {e}"))),
        }
    });

    store
}

/// Get the store provided by `use_progress_provider` (call from any page).
/// Panics if no provider exists above this component: that is a programming error,
/// found the first time you open the page.
pub fn use_progress() -> ProgressStore {
    use_context::<ProgressStore>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn best_guess_keeps_the_smaller_number() {
        let mut p = Progress::default();
        p.record_guess(9);
        p.record_guess(5);
        p.record_guess(7);
        assert_eq!(p.guess_best_tries, Some(5));
    }

    #[test]
    fn snake_best_keeps_the_larger_score() {
        let mut p = Progress::default();
        p.record_snake(3);
        p.record_snake(10);
        p.record_snake(4);
        assert_eq!(p.snake_best, 10);
    }

    #[test]
    fn text_roundtrip() {
        let p = Progress { guess_best_tries: Some(6), rps_rounds: 12, tictactoe_games: 3, snake_best: 7 };
        assert_eq!(Progress::from_text(&p.to_text()), Ok(p));
    }

    #[test]
    fn roundtrip_without_a_best_guess() {
        let p = Progress::default();
        let text = p.to_text();
        assert!(!text.contains("guess_best_tries"));
        assert_eq!(Progress::from_text(&text), Ok(p));
    }

    #[test]
    fn empty_text_gives_the_defaults() {
        assert_eq!(Progress::from_text(""), Ok(Progress::default()));
        assert_eq!(Progress::from_text("\n  \n"), Ok(Progress::default()));
    }

    #[test]
    fn unknown_keys_and_spaces_are_tolerated() {
        let text = "future_feature=1\n  snake_best = 4 \n";
        let p = Progress::from_text(text).unwrap();
        assert_eq!(p.snake_best, 4);
    }

    #[test]
    fn a_line_without_equals_is_an_error() {
        assert_eq!(Progress::from_text("hello"), Err(ParseError::MissingEquals("hello".to_string())));
    }

    #[test]
    fn a_non_number_is_an_error_that_names_the_key() {
        assert_eq!(
            Progress::from_text("snake_best=lots"),
            Err(ParseError::NotANumber { key: "snake_best".to_string(), value: "lots".to_string() })
        );
        assert!(Progress::from_text("rps_rounds=-1").is_err()); // negative: not a u32
    }
}
