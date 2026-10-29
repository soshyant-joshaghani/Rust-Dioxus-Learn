//! EXPERIMENT: modules and privacy, in one file.
//!
//! Run:  cargo run --example exp_modules
//!
//! Real projects put modules in files (`mod games;` -> games.rs or games/mod.rs).
//! Here they are written inline with `mod name { ... }` so you can see the
//! whole structure at once. The rules are identical.

mod games {
    pub struct GameInfo {
        pub title: &'static str, // public field: anyone may read/construct
        score: u32,              // PRIVATE field: only this module may touch it
    }

    impl GameInfo {
        /// Because `score` is private, outside code MUST use this function to build a GameInfo.
        /// This is how a module protects its own rules (e.g. "score starts at 0").
        pub fn new(title: &'static str) -> GameInfo {
            GameInfo { title, score: 0 }
        }

        pub fn score(&self) -> u32 {
            self.score
        }

        pub fn add_point(&mut self) {
            self.score += 1;
        }
    }

    // Not `pub`: invisible outside `games`.
    fn secret_helper() -> u32 {
        42
    }

    pub fn answer() -> u32 {
        secret_helper() // OK: same module
    }

    // A nested module. Its parent can see only what is `pub` inside it.
    pub mod rules {
        pub fn max_players() -> u32 {
            // `super::` = the parent module. A child may use its parent's private items.
            super::secret_helper() / 21
        }
    }
}

use games::GameInfo; // bring a name into scope

fn main() {
    let mut g = GameInfo::new("Snake");
    g.add_point();
    println!("{} has {} point(s)", g.title, g.score());

    println!("answer() = {}", games::answer());
    println!("max players = {}", games::rules::max_players());

    // println!("{}", g.score);                      // ERROR: field `score` is private
    // let h = GameInfo { title: "x", score: 9 };    // ERROR: private field `score`
    // println!("{}", games::secret_helper());      // ERROR: function `secret_helper` is private
}
