//! Rock Paper Scissors: the RULES. No screen code.
//!
//! The star of this module is the ENUM. A move is exactly one of three things,
//! an outcome is exactly one of three things, and `match` makes the compiler
//! prove that every case is handled.
//!
//! ```text
//!   player: Move  ---\
//!                     judge(player, computer) --> Outcome { Win | Lose | Draw }
//!   computer: Move --/
//!
//!   RpsGame::play(player_move)
//!       1. computer picks a Move with the Rng
//!       2. judge() decides the Outcome
//!       3. the Score is updated, the Round is remembered
//!       4. the Round is returned
//! ```

use std::fmt;

use crate::modules::base::random::Rng;

// ---------------------------------------------------------------------------
// Move: an enum with three plain variants
// ---------------------------------------------------------------------------

/// `Copy` + `Clone`: a Move is one tiny value (one byte), so passing it around
/// by value is as cheap as passing an integer. Without `Copy` we would have to
/// write `.clone()` or borrow it everywhere, for no benefit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    /// Every move, in a fixed order. An `associated constant`: it belongs to the
    /// type, not to a value. Used to draw the buttons and to pick a random move.
    pub const ALL: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];

    /// Text for the UI. Returns `&'static str`: the texts are literals inside the
    /// program, so no allocation and no ownership question.
    pub fn label(self) -> &'static str {
        match self {
            Move::Rock => "Rock",
            Move::Paper => "Paper",
            Move::Scissors => "Scissors",
        }
    }
}

// Display = how a Move prints with `{}`. We reuse `label()` so the text lives in one place.
impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

// ---------------------------------------------------------------------------
// Outcome and judge()
// ---------------------------------------------------------------------------

/// The result of a round, from the PLAYER's point of view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Win,
    Lose,
    Draw,
}

/// Decide who wins.
///
/// This is a pure FUNCTION: same input, same output, no state, no side effects.
/// That makes it trivial to test completely (all 9 combinations).
///
/// It `match`es on a TUPLE `(player, computer)`: Rust can take several values
/// apart at once. `|` joins patterns that share one result. There is NO `_`
/// arm: the nine combinations are listed, and the compiler checks that none is
/// missing. Delete one line and it will not compile.
pub fn judge(player: Move, computer: Move) -> Outcome {
    use Move::{Paper, Rock, Scissors}; // a local `use` saves typing `Move::` everywhere

    match (player, computer) {
        (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Outcome::Draw,
        (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Outcome::Win,
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Outcome::Lose,
    }
}

// ---------------------------------------------------------------------------
// Round and Score: small structs
// ---------------------------------------------------------------------------

/// What happened in one round. All fields are `Copy`, so the struct can be
/// `Copy` too: handing a `Round` to the screen does not move it away from us.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Round {
    pub player: Move,
    pub computer: Move,
    pub outcome: Outcome,
}

/// Running totals. `Default` gives `Score { wins: 0, losses: 0, draws: 0 }`
/// for free (the default of a number is 0).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Score {
    pub wins: u32,
    pub losses: u32,
    pub draws: u32,
}

impl Score {
    pub fn rounds(&self) -> u32 {
        self.wins + self.losses + self.draws
    }

    fn record(&mut self, outcome: Outcome) {
        // `match` with a statement in each arm. `+= 1` needs `&mut self`.
        match outcome {
            Outcome::Win => self.wins += 1,
            Outcome::Lose => self.losses += 1,
            Outcome::Draw => self.draws += 1,
        }
    }
}

// ---------------------------------------------------------------------------
// The game
// ---------------------------------------------------------------------------

/// The state of a whole match.
///
/// NOT `Copy`: it contains an `Rng`, which has a changing state. Copying a
/// generator would give two generators producing the SAME numbers: a bug.
/// It is `Clone` only (an explicit, visible duplicate).
#[derive(Debug, Clone)]
pub struct RpsGame {
    rng: Rng,
    score: Score,
    last: Option<Round>, // `None` before the first round
}

impl RpsGame {
    pub fn new(seed: u64) -> RpsGame {
        RpsGame { rng: Rng::new(seed), score: Score::default(), last: None }
    }

    /// Play one round and return what happened.
    pub fn play(&mut self, player: Move) -> Round {
        // Pick an index 0..=2 and look it up in the array.
        // `as usize`: array indexes must be `usize`.
        let index = self.rng.range(0, 2) as usize;
        let computer = Move::ALL[index];

        let outcome = judge(player, computer);
        self.score.record(outcome);

        let round = Round { player, computer, outcome };
        self.last = Some(round); // `Round` is Copy: `last` gets a copy, `round` stays ours
        round
    }

    /// Returns a COPY of the totals (`Score` is Copy), so the caller cannot change ours.
    pub fn score(&self) -> Score {
        self.score
    }

    pub fn last_round(&self) -> Option<Round> {
        self.last
    }

    /// Reset the totals but keep the generator going.
    pub fn reset_score(&mut self) {
        self.score = Score::default();
        self.last = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_combination_is_judged_correctly() {
        use Move::*;
        use Outcome::*;
        // A table of (player, computer, expected). One loop, nine checks.
        let table = [
            (Rock, Rock, Draw),
            (Rock, Paper, Lose),
            (Rock, Scissors, Win),
            (Paper, Rock, Win),
            (Paper, Paper, Draw),
            (Paper, Scissors, Lose),
            (Scissors, Rock, Lose),
            (Scissors, Paper, Win),
            (Scissors, Scissors, Draw),
        ];
        for (player, computer, expected) in table {
            assert_eq!(judge(player, computer), expected, "{player} vs {computer}");
        }
    }

    #[test]
    fn judging_is_symmetric() {
        // If I win against you, you lose against me, and draws are mutual.
        for a in Move::ALL {
            for b in Move::ALL {
                let mirrored = match judge(b, a) {
                    Outcome::Win => Outcome::Lose,
                    Outcome::Lose => Outcome::Win,
                    Outcome::Draw => Outcome::Draw,
                };
                assert_eq!(judge(a, b), mirrored);
            }
        }
    }

    #[test]
    fn score_always_adds_up() {
        let mut game = RpsGame::new(5);
        for i in 0..50 {
            game.play(Move::ALL[i % 3]);
        }
        assert_eq!(game.score().rounds(), 50);
    }

    #[test]
    fn computer_uses_all_three_moves() {
        let mut game = RpsGame::new(11);
        let mut seen = [false; 3];
        for _ in 0..300 {
            let round = game.play(Move::Rock);
            let index = Move::ALL.iter().position(|m| *m == round.computer).unwrap();
            seen[index] = true;
        }
        assert_eq!(seen, [true, true, true]);
    }

    #[test]
    fn last_round_is_remembered_and_reset_clears_it() {
        let mut game = RpsGame::new(1);
        assert_eq!(game.last_round(), None);
        let round = game.play(Move::Paper);
        assert_eq!(game.last_round(), Some(round));
        game.reset_score();
        assert_eq!(game.last_round(), None);
        assert_eq!(game.score(), Score::default());
    }

    #[test]
    fn move_prints_its_label() {
        assert_eq!(Move::Scissors.to_string(), "Scissors");
    }
}
