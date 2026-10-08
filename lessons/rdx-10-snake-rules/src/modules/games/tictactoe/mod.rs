//! Tic-Tac-Toe: the RULES. No screen code.
//!
//! The board is an ARRAY of 9 cells, numbered like a phone keypad read left to
//! right, top to bottom:
//!
//! ```text
//!    0 | 1 | 2
//!   ---+---+---
//!    3 | 4 | 5
//!   ---+---+---
//!    6 | 7 | 8
//! ```
//!
//! A move is "put the current player's mark on cell N". Everything that can go
//! wrong with that is a `MoveError`; everything that can result is a `Status`.
//!
//! BORROWING MAP (who may change what):
//!
//! ```text
//!   Board::play(&mut self, i)          needs a MUTABLE borrow: it changes the board
//!     |-- put_mark(&mut [Cell], ...)    gets a mutable borrow of just the cells
//!     `-- winner_of(&[Cell])            gets a shared borrow (read only), as a slice
//!   Board::cell(&self, i)               shared borrow: read only
//! ```

use std::fmt;

use crate::modules::games::Game;

// ---------------------------------------------------------------------------
// Small types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

impl Player {
    /// The opponent.
    pub fn other(self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Player::X => "X",
            Player::O => "O",
        }
    }
}

/// One square. An enum whose second variant CARRIES DATA: whose mark it is.
/// `Default` = `Empty`, so `[Cell::default(); 9]` is an empty board.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Cell {
    #[default]
    Empty,
    Taken(Player),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    InProgress,
    Won(Player),
    Draw,
}

/// Why a move was refused. The board is NOT changed when you get one of these.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveError {
    OutOfBounds(usize),
    Occupied(usize),
    GameOver,
}

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoveError::OutOfBounds(i) => write!(f, "There is no cell {i}."),
            MoveError::Occupied(i) => write!(f, "Cell {i} is already taken."),
            MoveError::GameOver => write!(f, "The game is over. Start a new one."),
        }
    }
}

impl std::error::Error for MoveError {}

// ---------------------------------------------------------------------------
// Winning lines
// ---------------------------------------------------------------------------

/// The 8 ways to win: 3 rows, 3 columns, 2 diagonals.
/// An array of 8 arrays of 3 indexes. Fixed size, known at compile time.
const LINES: [[usize; 3]; 8] = [
    [0, 1, 2], [3, 4, 5], [6, 7, 8], // rows
    [0, 3, 6], [1, 4, 7], [2, 5, 8], // columns
    [0, 4, 8], [2, 4, 6],            // diagonals
];

// ---------------------------------------------------------------------------
// The board
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Board {
    cells: [Cell; 9],
    next: Player,
    status: Status,
}

impl Board {
    pub fn new() -> Board {
        Board { cells: [Cell::Empty; 9], next: Player::X, status: Status::InProgress }
    }

    /// Place the current player's mark on `index`.
    ///
    /// Order of the checks matters: first reject, only then change anything.
    /// So a refused move leaves the board exactly as it was.
    pub fn play(&mut self, index: usize) -> Result<Status, MoveError> {
        // 1. is the game still running?
        if self.status != Status::InProgress {
            return Err(MoveError::GameOver);
        }

        // 2. is it a real, free cell?
        match self.cells.get(index) {
            None => return Err(MoveError::OutOfBounds(index)), // `get` returns Option: no panic
            Some(Cell::Taken(_)) => return Err(MoveError::Occupied(index)),
            Some(Cell::Empty) => {}
        }

        // 3. change the board.
        // `&mut self.cells` is a mutable borrow of ONE field. While `put_mark` runs,
        // nothing else can touch `self.cells`.
        put_mark(&mut self.cells, index, self.next);

        // 4. what is the situation now? `&self.cells` (an array) is accepted where a
        // slice `&[Cell]` is expected.
        self.status = if let Some(winner) = winner_of(&self.cells) {
            Status::Won(winner)
        } else if is_full(&self.cells) {
            Status::Draw
        } else {
            self.next = self.next.other(); // only switch turns while the game goes on
            Status::InProgress
        };

        Ok(self.status)
    }

    /// Read one cell. `None` if the index is outside the board.
    pub fn cell(&self, index: usize) -> Option<Cell> {
        self.cells.get(index).copied() // Option<&Cell> -> Option<Cell> (Cell is Copy)
    }

    pub fn next_player(&self) -> Player {
        self.next
    }

    pub fn status(&self) -> Status {
        self.status
    }

    /// Clear the board. Assigning a whole new Board DROPS the old one.
    pub fn reset(&mut self) {
        *self = Board::new(); // `*self` = "the value behind the &mut reference"
    }
}

// Clippy asks for this: a `new()` without arguments should also be the `Default`.
impl Default for Board {
    fn default() -> Board {
        Board::new()
    }
}

impl Game for Board {
    fn is_over(&self) -> bool {
        self.status() != Status::InProgress
    }

    fn summary(&self) -> String {
        match self.status() {
            Status::InProgress => format!("{}'s turn", self.next_player().label()),
            Status::Won(player) => format!("{} wins!", player.label()),
            Status::Draw => String::from("Draw."),
        }
    }
}

// ---------------------------------------------------------------------------
// Helper functions that work on SLICES
// ---------------------------------------------------------------------------
//
// A slice `&[Cell]` is "a borrowed view of some consecutive cells". It carries a
// pointer and a length. A function that takes `&[Cell]` works for an array of 9,
// a Vec of 9, or a part of a bigger board, and it cannot change the data.

/// Write a mark. `&mut [Cell]`: a mutable view, so we may assign into it.
fn put_mark(cells: &mut [Cell], index: usize, player: Player) {
    cells[index] = Cell::Taken(player); // index is validated by the caller
}

/// Who owns all three cells of a line?  `None` when nobody does.
fn line_owner(cells: &[Cell], line: [usize; 3]) -> Option<Player> {
    // Destructure the array of three indexes into three names.
    let [a, b, c] = line;

    // The pattern `Cell::Taken(player)` BINDS the player out of the first cell;
    // the `if` guard then checks that the other two cells hold the very same value.
    match cells[a] {
        Cell::Taken(player) if cells[b] == cells[a] && cells[c] == cells[a] => Some(player),
        _ => None,
    }
}

/// Does any of the 8 lines belong to one player?
fn winner_of(cells: &[Cell]) -> Option<Player> {
    for line in LINES {
        if let Some(player) = line_owner(cells, line) {
            return Some(player);
        }
    }
    None
}

/// Every cell taken? `iter().all(...)` is an iterator that stops at the first `false`.
fn is_full(cells: &[Cell]) -> bool {
    cells.iter().all(|cell| *cell != Cell::Empty)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Play a list of moves and expect each one to be accepted.
    fn play_all(board: &mut Board, moves: &[usize]) {
        for &index in moves {
            board.play(index).expect("move should be legal");
        }
    }

    #[test]
    fn players_alternate_starting_with_x() {
        let mut board = Board::new();
        assert_eq!(board.next_player(), Player::X);
        board.play(0).unwrap();
        assert_eq!(board.next_player(), Player::O);
        board.play(1).unwrap();
        assert_eq!(board.next_player(), Player::X);
        assert_eq!(board.cell(0), Some(Cell::Taken(Player::X)));
        assert_eq!(board.cell(1), Some(Cell::Taken(Player::O)));
    }

    #[test]
    fn row_wins() {
        let mut board = Board::new();
        play_all(&mut board, &[0, 3, 1, 4]);
        assert_eq!(board.play(2), Ok(Status::Won(Player::X)));
    }

    #[test]
    fn column_wins() {
        let mut board = Board::new();
        play_all(&mut board, &[1, 0, 2, 3, 4, 6]);
        // O has 0, 3, 6: the first column
        assert_eq!(board.status(), Status::Won(Player::O));
    }

    #[test]
    fn diagonal_wins() {
        let mut board = Board::new();
        play_all(&mut board, &[0, 1, 4, 2]);
        assert_eq!(board.play(8), Ok(Status::Won(Player::X)));

        let mut other = Board::new();
        play_all(&mut other, &[2, 0, 4, 1]);
        assert_eq!(other.play(6), Ok(Status::Won(Player::X)));
    }

    #[test]
    fn full_board_without_winner_is_a_draw() {
        let mut board = Board::new();
        // X O X
        // X O O
        // O X X
        play_all(&mut board, &[0, 1, 2, 4, 3, 5, 7, 6]);
        assert_eq!(board.play(8), Ok(Status::Draw));
    }

    #[test]
    fn occupied_cell_is_refused_and_turn_is_kept() {
        let mut board = Board::new();
        board.play(4).unwrap();
        assert_eq!(board.play(4), Err(MoveError::Occupied(4)));
        assert_eq!(board.next_player(), Player::O); // still O's turn
    }

    #[test]
    fn out_of_bounds_is_refused() {
        let mut board = Board::new();
        assert_eq!(board.play(9), Err(MoveError::OutOfBounds(9)));
        assert_eq!(board.play(usize::MAX), Err(MoveError::OutOfBounds(usize::MAX)));
        assert_eq!(board.cell(9), None);
    }

    #[test]
    fn no_moves_after_the_game_is_over() {
        let mut board = Board::new();
        play_all(&mut board, &[0, 3, 1, 4, 2]);
        assert_eq!(board.play(8), Err(MoveError::GameOver));
        // the winner stays the winner
        assert_eq!(board.status(), Status::Won(Player::X));
    }

    #[test]
    fn reset_gives_a_fresh_board() {
        let mut board = Board::new();
        play_all(&mut board, &[0, 3, 1, 4, 2]);
        board.reset();
        assert_eq!(board.status(), Status::InProgress);
        assert_eq!(board.next_player(), Player::X);
        assert_eq!(board.cell(0), Some(Cell::Empty));
    }

    #[test]
    fn error_texts() {
        assert_eq!(MoveError::Occupied(3).to_string(), "Cell 3 is already taken.");
    }
}
