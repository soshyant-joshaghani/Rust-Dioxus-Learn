//! Everything about games: the catalogue and (one folder per game) the rules.
//!
//! Module layout:
//!   games/mod.rs        <- you are here: GameInfo, GAMES
//!   games/game_card.rs  <- how one game is drawn in the list
//!   games/guess/        <- the RULES of Number Guessing (no screen code)
//!   games/rps/          <- the RULES of Rock Paper Scissors
//!   games/tictactoe/    <- the RULES of Tic-Tac-Toe
//!
//! The matching SCREEN of each game lives in `routes/games/`. Rules in
//! `modules`, pages in `routes`: the rules never know how they are displayed.

pub mod game_card;
pub mod guess;
pub mod rps;
pub mod tictactoe;

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
