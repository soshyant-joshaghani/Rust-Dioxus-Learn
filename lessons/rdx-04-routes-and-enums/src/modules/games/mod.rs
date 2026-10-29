//! Everything about games. For now only the catalogue (the list of games).
//! In later lessons each game gets its own submodule here: `guess`, `rps`...
//!
//! Module layout:
//!   games/mod.rs        <- you are here: GameInfo, GAMES
//!   games/game_card.rs  <- how one game is drawn in the list

pub mod game_card;

/// What we know about one game.
///
/// `pub struct` makes the TYPE visible; each field also needs its own `pub`.
/// Privacy is per item: a `pub struct` with private fields can only be built
/// by code inside this module.
pub struct GameInfo {
    pub title: &'static str,
    pub blurb: &'static str,
    pub concept: &'static str,
}

/// The catalogue. Order here is the order on the screen.
pub const GAMES: [GameInfo; 4] = [
    GameInfo { title: "Number Guessing", blurb: "Find the secret number.", concept: "Result + match" },
    GameInfo { title: "Rock Paper Scissors", blurb: "Beat the computer.", concept: "enums" },
    GameInfo { title: "Tic-Tac-Toe", blurb: "Three in a row.", concept: "arrays + &mut" },
    GameInfo { title: "Snake", blurb: "Eat, grow, do not crash.", concept: "collections + loop" },
];
