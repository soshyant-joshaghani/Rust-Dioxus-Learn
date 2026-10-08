//! Waiting, without freezing the screen.
//!
//! THE PROBLEM
//!   A game loop needs "do something, wait 150 ms, do it again".
//!   The obvious tool, `std::thread::sleep`, BLOCKS the thread. On the web there
//!   is only one thread and it also draws the page: sleeping on it freezes the
//!   whole tab (and WebAssembly cannot spawn threads by default anyway).
//!
//! THE WAY OUT
//!   An `async` wait. While we `.await`, the thread is free to draw and handle
//!   clicks; when the time is up, our code continues right after the `.await`.
//!   Rust's standard library has no timer for this, and the usual crates
//!   (`tokio`, `gloo-timers`) are off-limits in this course. But every Dioxus
//!   platform has JavaScript, and JavaScript has `setTimeout`.
//!
//! WHAT THE RUST CODE DOES
//!
//! ```text
//!   sleep_ms(150).await
//!      |  evaluates:  new Promise(resolve => setTimeout(resolve, 150))
//!      |  Rust code is PAUSED here (the UI keeps running)
//!      v  150 ms later the promise resolves, Rust continues
//! ```

use dioxus::prelude::*;

/// Wait for `ms` milliseconds.
///
/// Returns nothing: a failed timer would only make the game tick immediately,
/// and that is not worth an error type. (We `let _ =` the result on purpose.)
pub async fn sleep_ms(ms: u32) {
    // `format!` builds the JavaScript text. `{ms}` is a u32, so there is no way to
    // inject anything but digits into the script.
    let script = format!("await new Promise(resolve => setTimeout(resolve, {ms})); return true;");
    let _ = document::eval(&script).join::<bool>().await;
}
