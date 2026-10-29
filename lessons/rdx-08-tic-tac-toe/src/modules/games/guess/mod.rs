//! Number Guessing: the RULES, with no screen code at all.
//!
//! In lesson 05 these rules were tangled into the component. Now they live
//! here as plain Rust that can be tested with `cargo test`.
//!
//! WHAT THIS MODULE OFFERS (its public surface)
//!
//! ```text
//!   GuessGame::new(secret)            start a game with a known secret
//!   GuessGame::from_seed(seed)        start a game with a random secret
//!   game.guess_text("42")             -> Result<Hint, GuessError>
//!   game.guess(42)                    -> Result<Hint, GuessError>
//!   game.tries() / game.is_won()      read the state
//! ```
//!
//! WHY `Result<Hint, GuessError>` AND NOT JUST `Hint`?
//!   A guess can be WRONG in two different ways: a mistake by the player
//!   ("abc", 500, playing after winning) or a normal game outcome (too low).
//!   Mistakes are not outcomes. Returning `Result` keeps them apart, so the
//!   caller cannot accidentally treat "not a number" as "too low", and the
//!   compiler forces the caller to deal with the error case.

use std::fmt;
use std::num::ParseIntError;

use crate::modules::base::random::Rng;

pub const MIN: u32 = 1;
pub const MAX: u32 = 100;

// ---------------------------------------------------------------------------
// Outcomes and errors: two enums
// ---------------------------------------------------------------------------

/// A valid guess compared with the secret.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hint {
    TooLow,
    TooHigh,
    Correct,
}

/// Everything that can go wrong when guessing. One enum, one variant per reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuessError {
    /// The text was not a whole number (or was negative).
    NotANumber,
    /// A number, but outside `MIN..=MAX`. Carries the offending number.
    OutOfRange(u32),
    /// The game is already won; start a new one.
    GameOver,
}

// `Display` is the trait behind `{}` formatting and `.to_string()`.
// Implementing it lets us turn an error into text for the player.
impl fmt::Display for GuessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GuessError::NotANumber => write!(f, "That is not a whole number."),
            GuessError::OutOfRange(n) => write!(f, "{n} is outside {MIN} to {MAX}."),
            GuessError::GameOver => write!(f, "You already won. Start a new game."),
        }
    }
}

// Marks `GuessError` as a "real" error type so it works with `?` and with
// `Box<dyn Error>` (see examples/exp_errors.rs). Needs `Debug` + `Display`.
impl std::error::Error for GuessError {}

// `From` tells Rust how to CONVERT one type into another. With this, the `?`
// operator in `guess_text` can turn a `ParseIntError` into our `GuessError`
// automatically. We drop the details of the parse error: the player only needs
// to know "not a number".
impl From<ParseIntError> for GuessError {
    fn from(_: ParseIntError) -> GuessError {
        GuessError::NotANumber
    }
}

// ---------------------------------------------------------------------------
// The game state: a struct with methods
// ---------------------------------------------------------------------------

/// One round of the game.
///
/// All fields are PRIVATE. The only way to change `tries` or `won` is through
/// `guess`, so they can never disagree with each other (an "invariant":
/// `won` is true only if the last valid guess was `Correct`).
#[derive(Debug, Clone)]
pub struct GuessGame {
    secret: u32,
    tries: u32,
    won: bool,
}

impl GuessGame {
    /// Start a round with a chosen secret (used by tests and by `from_seed`).
    pub fn new(secret: u32) -> GuessGame {
        GuessGame { secret, tries: 0, won: false }
    }

    /// Start a round with a random secret derived from `seed`.
    pub fn from_seed(seed: u64) -> GuessGame {
        let mut rng = Rng::new(seed); // `rng` is a local owner; `range` borrows it mutably
        GuessGame::new(rng.range(MIN, MAX))
    }

    /// Guess from text typed by the player.
    ///
    /// `&mut self`: this call can CHANGE the game, so the caller needs a
    /// mutable borrow. `text: &str` is only read, so it is borrowed.
    pub fn guess_text(&mut self, text: &str) -> Result<Hint, GuessError> {
        // `?` means: "if this is an Err, return it from the function right now
        // (converting it with `From` if needed); if Ok, give me the value".
        // This one character replaces a whole `match`.
        let number: u32 = text.trim().parse()?;
        self.guess(number)
    }

    /// Guess with a number.
    pub fn guess(&mut self, number: u32) -> Result<Hint, GuessError> {
        if self.won {
            return Err(GuessError::GameOver);
        }
        if !(MIN..=MAX).contains(&number) {
            return Err(GuessError::OutOfRange(number));
        }

        // Only VALID guesses count as tries.
        self.tries += 1;

        let hint = if number < self.secret {
            Hint::TooLow
        } else if number > self.secret {
            Hint::TooHigh
        } else {
            self.won = true;
            Hint::Correct
        };
        Ok(hint)
    }

    /// `&self`: read-only. These getters are the only way outside code can see
    /// the private fields.
    pub fn tries(&self) -> u32 {
        self.tries
    }

    pub fn is_won(&self) -> bool {
        self.won
    }
}

// ---------------------------------------------------------------------------
// Tests. They are compiled ONLY for `cargo test`, never into the app.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn low_high_and_correct() {
        let mut game = GuessGame::new(50);
        assert_eq!(game.guess(10), Ok(Hint::TooLow));
        assert_eq!(game.guess(90), Ok(Hint::TooHigh));
        assert_eq!(game.guess(50), Ok(Hint::Correct));
        assert!(game.is_won());
        assert_eq!(game.tries(), 3);
    }

    #[test]
    fn text_that_is_not_a_number_is_an_error() {
        let mut game = GuessGame::new(50);
        assert_eq!(game.guess_text("abc"), Err(GuessError::NotANumber));
        assert_eq!(game.guess_text(""), Err(GuessError::NotANumber));
        assert_eq!(game.guess_text("-5"), Err(GuessError::NotANumber));
    }

    #[test]
    fn text_is_trimmed() {
        let mut game = GuessGame::new(50);
        assert_eq!(game.guess_text("  50 \n"), Ok(Hint::Correct));
    }

    #[test]
    fn out_of_range_is_an_error_and_not_counted() {
        let mut game = GuessGame::new(50);
        assert_eq!(game.guess(0), Err(GuessError::OutOfRange(0)));
        assert_eq!(game.guess(101), Err(GuessError::OutOfRange(101)));
        assert_eq!(game.tries(), 0);
    }

    #[test]
    fn cannot_guess_after_winning() {
        let mut game = GuessGame::new(7);
        assert_eq!(game.guess(7), Ok(Hint::Correct));
        assert_eq!(game.guess(8), Err(GuessError::GameOver));
        assert_eq!(game.tries(), 1);
    }

    #[test]
    fn from_seed_is_repeatable_and_in_range() {
        let a = GuessGame::from_seed(99);
        let b = GuessGame::from_seed(99);
        assert_eq!(a.secret, b.secret); // tests may read private fields
        assert!((MIN..=MAX).contains(&a.secret));
    }

    #[test]
    fn error_messages_are_readable() {
        assert_eq!(GuessError::OutOfRange(500).to_string(), "500 is outside 1 to 100.");
    }
}
