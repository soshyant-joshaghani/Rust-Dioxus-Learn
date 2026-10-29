//! Snake: the RULES. No screen code, no clock, no keyboard.
//!
//! The game is a state machine that moves forward ONE STEP at a time:
//!
//! ```text
//!                 turn(dir)                      step()
//!   (player) ----------------> next_direction ------------+
//!                                                         v
//!   body: VecDeque<Pos>   head ... ... tail      compute the new head
//!                                                  |
//!             wall?  -------------------------------+--> Dead(Wall)
//!             own body? ----------------------------+--> Dead(SelfBite)
//!             food? -> grow (keep the tail), score+1, new food
//!             else  -> move (add a new head, drop the tail)
//! ```
//!
//! WHO CALLS `step()`?
//!   Not this module. Lesson 10's page calls it from a button, lesson 11's page
//!   from a timer. The rules do not care: that is why they can be tested with
//!   a plain loop in `cargo test`.
//!
//! COLLECTION CHOICE: `VecDeque<Pos>` (a double-ended queue)
//!   Each step adds a cell at the FRONT (new head) and usually removes one at
//!   the BACK (old tail). A `Vec` is fast only at the back; removing from its
//!   front moves every element. `VecDeque` is fast at both ends.

use std::collections::VecDeque;
use std::fmt;

use crate::modules::base::random::Rng;
use crate::modules::games::Game;

pub const DEFAULT_WIDTH: i32 = 16;
pub const DEFAULT_HEIGHT: i32 = 12;

const TICK_START_MS: u32 = 150;
const TICK_STEP_MS: u32 = 5;
const TICK_MIN_MS: u32 = 60;

// ---------------------------------------------------------------------------
// Coordinates and directions
// ---------------------------------------------------------------------------

/// A cell of the board. `x` grows to the right, `y` grows DOWNWARD (like on a screen).
///
/// `i32` and not `usize`: the cell left of column 0 is `x = -1`. With an unsigned
/// type that value cannot exist, and stepping off the board would be an
/// arithmetic overflow instead of a clean "outside the board" check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl Pos {
    /// The neighbouring cell in `direction`. Returns a NEW Pos (Pos is Copy).
    pub fn step(self, direction: Direction) -> Pos {
        match direction {
            Direction::Up => Pos { x: self.x, y: self.y - 1 },
            Direction::Down => Pos { x: self.x, y: self.y + 1 },
            Direction::Left => Pos { x: self.x - 1, y: self.y },
            Direction::Right => Pos { x: self.x + 1, y: self.y },
        }
    }
}

// ---------------------------------------------------------------------------
// State, outcomes, errors
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cause {
    Wall,
    SelfBite,
}

/// Where the game stands. A small STATE MACHINE:
///
/// ```text
///   Running --(hit wall / bite self)--> Dead(cause)     (final)
///   Running --(board completely full)-> Won             (final)
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Running,
    Dead(Cause),
    Won,
}

/// What ONE successful `step()` did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOutcome {
    Moved,
    Ate,
    Died(Cause),
    Won,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnakeError {
    /// `step()` or `turn()` after the game ended.
    GameOver,
    /// Turning straight back into your own neck.
    Reverse,
}

impl fmt::Display for SnakeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SnakeError::GameOver => write!(f, "The game is over."),
            SnakeError::Reverse => write!(f, "You cannot turn back into yourself."),
        }
    }
}

impl std::error::Error for SnakeError {}

/// What is on a cell? Used by the page to choose a colour. A plain enum so the
/// page does not need to know how the snake is stored.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Head,
    Body,
    Food,
}

// ---------------------------------------------------------------------------
// The game
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct SnakeGame {
    width: i32,
    height: i32,
    /// Front = head, back = tail. Never empty while the game exists.
    body: VecDeque<Pos>,
    /// The direction of the last step actually taken.
    direction: Direction,
    /// The direction the player wants for the NEXT step.
    next_direction: Direction,
    food: Pos,
    score: u32,
    status: Status,
    rng: Rng,
}

impl SnakeGame {
    /// Standard board.
    pub fn from_seed(seed: u64) -> SnakeGame {
        SnakeGame::with_size(DEFAULT_WIDTH, DEFAULT_HEIGHT, seed)
    }

    /// Any board size (tests use tiny ones). The snake starts with length 1 in
    /// the middle, heading right.
    pub fn with_size(width: i32, height: i32, seed: u64) -> SnakeGame {
        assert!(width >= 2 && height >= 1, "board too small");

        let head = Pos { x: width / 2, y: height / 2 };
        let mut body = VecDeque::new();
        body.push_back(head);

        let mut game = SnakeGame {
            width,
            height,
            body,
            direction: Direction::Right,
            next_direction: Direction::Right,
            food: head, // placeholder, replaced right below
            score: 0,
            status: Status::Running,
            rng: Rng::new(seed),
        };
        game.food = game.random_free_cell().expect("a new board always has a free cell");
        game
    }

    /// Ask for a new direction. It takes effect at the next `step()`.
    pub fn turn(&mut self, direction: Direction) -> Result<(), SnakeError> {
        if self.status != Status::Running {
            return Err(SnakeError::GameOver);
        }
        // Compare with the direction of the LAST STEP, not with a pending turn:
        // otherwise "Up then Left" pressed quickly while moving Right would sneak a
        // reversal through.
        if self.body.len() > 1 && direction == self.direction.opposite() {
            return Err(SnakeError::Reverse);
        }
        self.next_direction = direction;
        Ok(())
    }

    /// Advance the game by one tick.
    ///
    /// `Result<StepOutcome, SnakeError>`: dying is a normal OUTCOME (`Ok(Died)`),
    /// stepping a finished game is a CALLER MISTAKE (`Err(GameOver)`), for
    /// example a late timer tick. The caller can simply ignore that error.
    pub fn step(&mut self) -> Result<StepOutcome, SnakeError> {
        if self.status != Status::Running {
            return Err(SnakeError::GameOver);
        }

        self.direction = self.next_direction;
        let head = self.body[0]; // indexing a VecDeque is allowed; [0] is the front
        let new_head = head.step(self.direction);

        // 1. wall?
        if !self.inside(new_head) {
            return Ok(self.die(Cause::Wall));
        }

        // 2. growing or moving? We grow exactly when we land on the food.
        let eating = new_head == self.food;

        // 3. own body? When we only MOVE, the tail cell is free again this step, so it
        //    must not count. `take(n)` looks at the first n cells (head side).
        let blocking = if eating { self.body.len() } else { self.body.len() - 1 };
        if self.body.iter().take(blocking).any(|cell| *cell == new_head) {
            return Ok(self.die(Cause::SelfBite));
        }

        // 4. all clear: apply the move.
        self.body.push_front(new_head);
        if !eating {
            self.body.pop_back(); // drop the tail; the length stays the same
            return Ok(StepOutcome::Moved);
        }

        // 5. ate: the tail stays, so the snake is one longer.
        self.score += 1;
        match self.random_free_cell() {
            Some(cell) => {
                self.food = cell;
                Ok(StepOutcome::Ate)
            }
            None => {
                // no free cell left: the snake fills the whole board
                self.status = Status::Won;
                Ok(StepOutcome::Won)
            }
        }
    }

    fn die(&mut self, cause: Cause) -> StepOutcome {
        self.status = Status::Dead(cause);
        StepOutcome::Died(cause)
    }

    fn inside(&self, pos: Pos) -> bool {
        (0..self.width).contains(&pos.x) && (0..self.height).contains(&pos.y)
    }

    /// A random cell that is not part of the snake, or `None` when the board is full.
    ///
    /// We BUILD the list of free cells and pick one by index, instead of drawing
    /// random cells until one is free: that loop could, in theory, run forever on
    /// an almost full board.
    fn random_free_cell(&mut self) -> Option<Pos> {
        let mut free: Vec<Pos> = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Pos { x, y };
                if !self.body.contains(&pos) {
                    free.push(pos);
                }
            }
        }

        if free.is_empty() {
            return None;
        }
        let index = self.rng.range(0, free.len() as u32 - 1) as usize;
        Some(free[index])
    }

    /// How many milliseconds to wait before the next step.
    ///
    /// Speed is a RULE of the game (it depends on the score), so it lives here,
    /// and the timer in the page just asks. 150 ms at the start, 5 ms faster per
    /// point, never faster than 60 ms.
    pub fn tick_ms(&self) -> u32 {
        // `saturating_sub` stops at 0 instead of wrapping around or panicking.
        TICK_START_MS.saturating_sub(self.score * TICK_STEP_MS).max(TICK_MIN_MS)
    }

    // ---- read-only access for the page -------------------------------------

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn tile_at(&self, pos: Pos) -> Tile {
        if self.body[0] == pos {
            Tile::Head
        } else if self.body.contains(&pos) {
            Tile::Body
        } else if self.food == pos {
            Tile::Food
        } else {
            Tile::Empty
        }
    }
}

impl Game for SnakeGame {
    fn is_over(&self) -> bool {
        self.status() != Status::Running
    }

    fn summary(&self) -> String {
        match self.status() {
            Status::Running => format!("Score: {}", self.score()),
            Status::Dead(Cause::Wall) => format!("You hit the wall. Score: {}", self.score()),
            Status::Dead(Cause::SelfBite) => format!("You bit yourself. Score: {}", self.score()),
            Status::Won => format!("You filled the board! Score: {}", self.score()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A small board where the test controls where the food is.
    fn game(width: i32, height: i32) -> SnakeGame {
        let mut game = SnakeGame::with_size(width, height, 1);
        game.food = Pos { x: 0, y: 0 }; // out of the way unless the test moves it
        game
    }

    fn head(game: &SnakeGame) -> Pos {
        game.body[0]
    }

    #[test]
    fn moves_one_cell_in_the_current_direction() {
        let mut g = game(10, 10);
        let start = head(&g);
        assert_eq!(g.step(), Ok(StepOutcome::Moved));
        assert_eq!(head(&g), Pos { x: start.x + 1, y: start.y });
        assert_eq!(g.body.len(), 1);
    }

    #[test]
    fn turning_changes_the_next_step() {
        let mut g = game(10, 10);
        let start = head(&g);
        g.turn(Direction::Down).unwrap();
        g.step().unwrap();
        assert_eq!(head(&g), Pos { x: start.x, y: start.y + 1 });
    }

    #[test]
    fn cannot_reverse_when_longer_than_one() {
        let mut g = game(10, 10);
        g.food = g.body[0].step(Direction::Right); // eat to become length 2
        g.step().unwrap();
        assert_eq!(g.body.len(), 2);
        assert_eq!(g.turn(Direction::Left), Err(SnakeError::Reverse));
        assert_eq!(g.turn(Direction::Up), Ok(()));
    }

    #[test]
    fn eating_grows_scores_and_moves_the_food() {
        let mut g = game(10, 10);
        let target = g.body[0].step(Direction::Right);
        g.food = target;
        assert_eq!(g.step(), Ok(StepOutcome::Ate));
        assert_eq!(g.score(), 1);
        assert_eq!(g.body.len(), 2);
        assert_ne!(g.food, target);
        assert!(!g.body.contains(&g.food), "food must not appear on the snake");
    }

    #[test]
    fn hitting_the_wall_ends_the_game() {
        let mut g = game(4, 3); // head starts at x = 2, so x = 4 is the wall: two steps
        assert_eq!(g.step(), Ok(StepOutcome::Moved));
        assert_eq!(g.step(), Ok(StepOutcome::Died(Cause::Wall)));
        assert_eq!(g.status(), Status::Dead(Cause::Wall));
        assert!(g.is_over());
    }

    #[test]
    fn nothing_works_after_death() {
        let mut g = game(4, 3);
        g.step().unwrap();
        g.step().unwrap();
        assert_eq!(g.step(), Err(SnakeError::GameOver));
        assert_eq!(g.turn(Direction::Up), Err(SnakeError::GameOver));
    }

    /// A snake of length 4 curled into a square (head first):
    ///
    /// ```text
    ///   y=4:  tail(5,4)  head(4,4)       <- the head is heading Up, it came from (4,5)
    ///   y=5:  (5,5)      (4,5)
    /// ```
    fn curled_snake(g: &mut SnakeGame) {
        g.body = VecDeque::from(vec![
            Pos { x: 4, y: 4 }, // head
            Pos { x: 4, y: 5 },
            Pos { x: 5, y: 5 },
            Pos { x: 5, y: 4 }, // tail
        ]);
        g.direction = Direction::Up;
        g.next_direction = Direction::Up;
    }

    #[test]
    fn biting_yourself_ends_the_game() {
        let mut g = game(10, 10);
        // Five cells: head (4,4), then (4,5), (3,5), (3,4), (3,3).
        g.body = VecDeque::from(vec![
            Pos { x: 4, y: 4 },
            Pos { x: 4, y: 5 },
            Pos { x: 3, y: 5 },
            Pos { x: 3, y: 4 },
            Pos { x: 3, y: 3 },
        ]);
        g.direction = Direction::Up;
        g.next_direction = Direction::Up;
        g.turn(Direction::Left).unwrap(); // head (4,4) -> (3,4), which is body
        assert_eq!(g.step(), Ok(StepOutcome::Died(Cause::SelfBite)));
    }

    #[test]
    fn following_your_own_tail_is_allowed() {
        let mut g = game(10, 10);
        curled_snake(&mut g);
        g.turn(Direction::Right).unwrap(); // head (4,4) -> (5,4): the cell the tail is leaving
        assert_eq!(g.step(), Ok(StepOutcome::Moved));
        assert_eq!(g.status(), Status::Running);
    }

    #[test]
    fn filling_the_board_wins() {
        // 2x1 board: the snake starts at x = 1, the only food cell is x = 0.
        let mut g = SnakeGame::with_size(2, 1, 3);
        g.turn(Direction::Left).unwrap(); // length 1, so turning around is allowed
        assert_eq!(g.food, Pos { x: 0, y: 0 });
        assert_eq!(g.step(), Ok(StepOutcome::Won));
        assert_eq!(g.status(), Status::Won);
    }

    #[test]
    fn food_never_spawns_on_the_snake() {
        for seed in 0..50 {
            let g = SnakeGame::from_seed(seed);
            assert!(!g.body.contains(&g.food), "seed {seed}");
        }
    }

    #[test]
    fn same_seed_same_first_food() {
        assert_eq!(SnakeGame::from_seed(8).food, SnakeGame::from_seed(8).food);
    }

    #[test]
    fn tiles_describe_the_board() {
        let g = game(10, 10);
        assert_eq!(g.tile_at(g.body[0]), Tile::Head);
        assert_eq!(g.tile_at(Pos { x: 0, y: 0 }), Tile::Food);
        assert_eq!(g.tile_at(Pos { x: 9, y: 9 }), Tile::Empty);
    }

    #[test]
    fn the_game_speeds_up_but_never_below_the_minimum() {
        let mut g = game(10, 10);
        assert_eq!(g.tick_ms(), 150);
        g.score = 4;
        assert_eq!(g.tick_ms(), 130);
        g.score = 18;
        assert_eq!(g.tick_ms(), 60); // 150 - 90 = 60
        g.score = 1_000;
        assert_eq!(g.tick_ms(), 60); // would underflow without saturating_sub
    }

    #[test]
    fn summary_describes_each_state() {
        let mut g = game(4, 3);
        assert_eq!(g.summary(), "Score: 0");
        g.step().unwrap();
        g.step().unwrap();
        assert_eq!(g.summary(), "You hit the wall. Score: 0");
    }
}
