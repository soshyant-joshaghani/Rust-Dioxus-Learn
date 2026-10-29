//! Everything about games: the catalogue (the list of games).
//! In later lessons each game gets its own submodule here: `guess`, `rps`...
//!
//! Module layout:
//!   games/mod.rs        <- you are here: GameInfo, GAMES
//!   games/game_card.rs  <- how one game is drawn in the list

pub mod game_card;

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
    GameInfo { slug: "guess", title: "Number Guessing", blurb: "Find the secret number.", concept: "Result + match" },
    GameInfo { slug: "rps", title: "Rock Paper Scissors", blurb: "Beat the computer.", concept: "enums" },
    GameInfo { slug: "tictactoe", title: "Tic-Tac-Toe", blurb: "Three in a row.", concept: "arrays + &mut" },
    GameInfo { slug: "snake", title: "Snake", blurb: "Eat, grow, do not crash.", concept: "collections + loop" },
];
