//! Where random numbers come from.
//!
//! THE PROBLEM
//!   Rust's standard library has NO random number generator (the usual
//!   answer is the `rand` crate, but this course allows only `dioxus`).
//!   The clock is not an option either: `SystemTime::now()` panics on the web.
//!
//! THE WAY OUT
//!   Every Dioxus platform runs our UI inside a browser or a webview, and
//!   JavaScript always has `Math.random()`. `document::eval` runs a snippet of
//!   JavaScript and gives the result back to Rust.
//!
//! Lesson 06 builds a tiny generator of our own on top of this seed.

use dioxus::prelude::*;

/// Ask JavaScript for a random number between 0 (inclusive) and 2^32 (exclusive).
///
/// WHY `async`?
///   The JavaScript runs outside our Rust code and answers later, so we have to
///   WAIT for the answer. `async fn` returns a "future": a promise-like value
///   that you `.await`. A component cannot wait while it draws, so callers
///   use `spawn(async move { ... })` (see the guessing game).
///
/// WHY NOT RETURN `Result<u64, ...>`?
///   Because there is a sensible fallback. If JavaScript fails (it should
///   never), the game still starts, just not randomly. The `match` below makes
///   that decision visible instead of hiding it in `.unwrap()`.
pub async fn random_seed() -> u64 {
    // `.join::<f64>()` waits for the script to finish and reads its `return`
    // value as an f64. It yields `Result<f64, EvalError>`.
    let answer = document::eval("return Math.random();").join::<f64>().await;

    match answer {
        // `as u64` converts the number type; the fractional part is cut off.
        Ok(fraction) => (fraction * 4_294_967_296.0) as u64,
        Err(_) => 12_345,
    }
}
