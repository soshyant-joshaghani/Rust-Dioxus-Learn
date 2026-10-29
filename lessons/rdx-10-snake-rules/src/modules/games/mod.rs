//! Everything about games: the catalogue, the shared `Game` trait, and (one
//! folder per game) the rules.
//!
//! Module layout:
//!   games/mod.rs        <- you are here: Game trait, describe(), GameInfo, GAMES
//!   games/game_card.rs  <- how one game is drawn in the list
//!   games/guess/        <- the RULES of Number Guessing (no screen code)
//!   games/rps/          <- the RULES of Rock Paper Scissors
//!   games/tictactoe/    <- the RULES of Tic-Tac-Toe
//!   games/snake/        <- the RULES of Snake

pub mod game_card;
pub mod guess;
pub mod rps;
pub mod snake;
pub mod tictactoe;

// ---------------------------------------------------------------------------
// The Game trait
// ---------------------------------------------------------------------------

/// What every game of the hub can tell the outside world.
///
/// WHY A TRAIT?
///   Problem : the three games have different types (`GuessGame`, `RpsGame`,
///             `Board`) but every page wants to ask the same two questions:
///             "is it over?" and "give me a one-line status".
///   Without : one `match`/function per game type, copied into every page; a
///             fourth game means editing all of them.
///   With    : a trait is a PROMISE ("this type can answer these questions").
///             Each game implements it once; shared code works with ANY type
///             that keeps the promise (see `describe` below).
///
/// Only the two methods without a body must be written by implementors.
pub trait Game {
    /// Has the round ended (won, drawn, finished)?
    fn is_over(&self) -> bool;

    /// A short status for the player, e.g. "X's turn".
    fn summary(&self) -> String;

    /// A DEFAULT method: it has a body, so implementors get it for free
    /// (and may override it). It is built only from the two required methods.
    fn headline(&self) -> String {
        if self.is_over() {
            format!("Finished. {}", self.summary())
        } else {
            self.summary()
        }
    }
}

/// A GENERIC function: it works for every type `G` that implements `Game`.
///
/// `<G: Game>` reads "for any type G, as long as G: Game". `&G` borrows it.
///
/// HOW IT COMPILES ("monomorphization")
///   Rust makes one specialised copy per type you actually use:
///   `describe::<GuessGame>`, `describe::<RpsGame>`, `describe::<Board>`.
///   Each copy is as fast as hand-written code (no lookup at run time).
///   The price is a slightly bigger program.
pub fn describe<G: Game>(game: &G) -> String {
    game.headline()
}

// ---------------------------------------------------------------------------
// The catalogue
// ---------------------------------------------------------------------------

/// What we know about one game.
pub struct GameInfo {
    /// Short id used to find the game's page. `modules` does not know about
    /// routes (dependency rule), so the page that knows both maps slug -> Route.
    pub slug: &'static str,
    pub title: &'static str,
    pub blurb: &'static str,
    pub concept: &'static str,
}

/// The catalogue. Order here is the order on the screen.
pub const GAMES: [GameInfo; 4] = [
    GameInfo { slug: "guess", title: "Number Guessing", blurb: "Find the secret number.", concept: "structs + Result" },
    GameInfo { slug: "rps", title: "Rock Paper Scissors", blurb: "Beat the computer.", concept: "enums + match" },
    GameInfo { slug: "tictactoe", title: "Tic-Tac-Toe", blurb: "Three in a row.", concept: "arrays + &mut" },
    GameInfo { slug: "snake", title: "Snake", blurb: "Eat, grow, do not crash.", concept: "collections + loop" },
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::games::guess::GuessGame;
    use crate::modules::games::rps::RpsGame;
    use crate::modules::games::tictactoe::Board;

    /// A fake game used only in tests: the trait lets us test generic code
    /// without any real game.
    struct Dummy(bool);

    impl Game for Dummy {
        fn is_over(&self) -> bool {
            self.0
        }
        fn summary(&self) -> String {
            String::from("dummy")
        }
    }

    #[test]
    fn default_method_uses_the_required_ones() {
        assert_eq!(Dummy(false).headline(), "dummy");
        assert_eq!(Dummy(true).headline(), "Finished. dummy");
    }

    #[test]
    fn describe_works_for_every_game_type() {
        assert_eq!(describe(&GuessGame::new(5)), "Tries: 0");
        assert_eq!(describe(&Board::new()), "X's turn");
        assert!(describe(&RpsGame::new(1)).starts_with("Rounds: 0"));
    }

    #[test]
    fn different_types_can_share_one_list_with_dyn() {
        // `&dyn Game` = "a reference to SOME type that implements Game, decided at run time".
        let guess = GuessGame::new(5);
        let board = Board::new();
        let all: [&dyn Game; 2] = [&guess, &board];
        assert!(all.iter().all(|game| !game.is_over()));
    }
}
