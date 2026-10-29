//! Randomness, in two layers.
//!
//!   1. `random_seed()`  asks JavaScript for ONE random number (async, needs a UI runtime)
//!   2. `Rng`            a tiny generator written in plain Rust, started from that seed
//!
//! WHY TWO LAYERS?
//!   Game rules should be plain Rust that you can test without a screen. If a
//!   rule called JavaScript directly it could not be unit-tested. So the rules
//!   take a SEED (a number) and use `Rng`; only the page calls `random_seed()`.
//!   Same seed in -> same numbers out, which is exactly what a test wants.

use dioxus::prelude::*;

// ---------------------------------------------------------------------------
// Layer 1: the seed (talks to JavaScript)
// ---------------------------------------------------------------------------

/// A random number in `0..2^32`, from JavaScript's `Math.random()`.
///
/// `async` because JavaScript answers later (see lesson 05). Returns a plain
/// `u64` with a fallback instead of a `Result`: starting a game must never
/// fail just because a seed could not be fetched.
pub async fn random_seed() -> u64 {
    let answer = document::eval("return Math.random();").join::<f64>().await;

    match answer {
        Ok(fraction) => (fraction * 4_294_967_296.0) as u64,
        Err(_) => 12_345,
    }
}

// ---------------------------------------------------------------------------
// Layer 2: the generator (pure Rust)
// ---------------------------------------------------------------------------

/// A small pseudo-random number generator ("xorshift64*").
///
/// "Pseudo-random": the numbers LOOK random but come from a formula. The
/// whole future sequence is decided by one hidden number, the `state`.
/// That is perfect for games (fast, repeatable in tests) and unsuitable for
/// passwords (predictable).
///
/// Owning struct: a `Rng` OWNS its state. To draw a number you need `&mut Rng`,
/// because drawing CHANGES the state. This is why the methods below take
/// `&mut self`.
#[derive(Debug, Clone)]
pub struct Rng {
    // Private field: nobody outside can put the generator into a bad state.
    // (xorshift must never have a state of 0, see `new`.)
    state: u64,
}

impl Rng {
    /// Create a generator from a seed. This is an "associated function": it has
    /// no `self`, so you call it as `Rng::new(7)`.
    pub fn new(seed: u64) -> Rng {
        // Mix the seed so that small, similar seeds (1, 2, 3...) give very
        // different starting states ("splitmix64" step).
        // `wrapping_*` = arithmetic that wraps around on overflow instead of
        // panicking. Hash-like math WANTS that.
        let mut z = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^= z >> 31;

        // The one forbidden state is 0 (it would stay 0 forever).
        if z == 0 {
            z = 0x2545_F491_4F6C_DD1D;
        }
        Rng { state: z }
    }

    /// The next raw 64-bit number. `&mut self`: this call changes the generator.
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12; // `>>` shift bits right, `^` xor: scramble the bits
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// A number from `min` to `max`, BOTH included.
    ///
    /// Precondition: `min <= max`. A violation is a programming mistake in the
    /// caller, not something the player can cause, so we `assert!` (crash with
    /// a message) instead of returning a `Result`.
    pub fn range(&mut self, min: u32, max: u32) -> u32 {
        assert!(min <= max, "Rng::range needs min <= max");
        let span = u64::from(max - min) + 1; // how many values are allowed
        // `%` = remainder. (A tiny bias exists when `span` does not divide 2^64
        // evenly; irrelevant for games.)
        min + (self.next_u64() % span) as u32
    }
}

#[cfg(test)]
mod tests {
    // `use super::*` = "everything from the parent module", so tests can see
    // private things too. This is the normal way to write unit tests in Rust.
    use super::*;

    #[test]
    fn same_seed_gives_same_sequence() {
        let mut a = Rng::new(42);
        let mut b = Rng::new(42);
        for _ in 0..100 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn different_seeds_give_different_numbers() {
        let mut a = Rng::new(1);
        let mut b = Rng::new(2);
        assert_ne!(a.next_u64(), b.next_u64());
    }

    #[test]
    fn range_stays_inside_bounds_and_hits_both_ends() {
        let mut rng = Rng::new(7);
        let mut seen_min = false;
        let mut seen_max = false;
        for _ in 0..1_000 {
            let n = rng.range(1, 6);
            assert!((1..=6).contains(&n), "{n} is outside 1..=6");
            seen_min |= n == 1;
            seen_max |= n == 6;
        }
        assert!(seen_min && seen_max);
    }

    #[test]
    fn seed_zero_is_safe() {
        let mut rng = Rng::new(0);
        assert_ne!(rng.next_u64(), 0);
    }
}
