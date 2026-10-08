//! App-wide constants and facts about the running program.
//!
//! One place to rename the app, read its version, or ask "which platform am I on?".

/// Shown in the page title and the header.
pub const APP_NAME: &str = "FoxG Game Hub";

/// The version from `Cargo.toml`, baked in at compile time.
///
/// `env!("CARGO_PKG_VERSION")` is a macro that Cargo feeds while compiling. If the
/// variable did not exist the build would fail: there is no run-time surprise.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Which client is running: `web`, `windows`, `macos`, `linux`, `android`, `ios`.
///
/// `cfg!(...)` is an ordinary `bool` decided at compile time. A web build runs as
/// WebAssembly, whose OS is "unknown", so we name it ourselves. Everything else
/// reports its OS through the standard library.
pub fn platform_name() -> &'static str {
    if cfg!(target_arch = "wasm32") {
        "web"
    } else {
        std::env::consts::OS
    }
}
