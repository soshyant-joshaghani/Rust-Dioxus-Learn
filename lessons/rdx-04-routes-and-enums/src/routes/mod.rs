//! The pages of the app, and the `Route` enum that maps URLs to pages.
//!
//! HOW A CLICK BECOMES A PAGE
//!
//! ```text
//!  user clicks <Link to: Route::About {}>
//!        |
//!        v
//!  router puts "/about" in the address bar (web) or its own history (desktop/mobile)
//!        |
//!        v
//!  router turns "/about" back into the value `Route::About {}`   (parsing)
//!        |
//!        v
//!  router draws:  AppLayout { Outlet -> About {} }
//! ```
//!
//! To add a page: (1) create the file, (2) `pub mod` it below,
//! (3) add ONE variant to `Route`. The compiler checks the rest.

use dioxus::prelude::*;

pub mod about;
pub mod games;
pub mod home;
pub mod layout;
pub mod not_found;

use about::About;
use games::Games;
use home::Home;
use layout::AppLayout;
use not_found::NotFound;

/// Every page the app can show is ONE variant of this enum.
///
/// An enum is "a value that is exactly one of these options". That is a
/// perfect model for "the current page": you can be on Home OR Games OR
/// About, never two at once, and never "nothing".
///
/// The derives:
///   Debug     : lets us print it with `{:?}` (useful when debugging)
///   Clone     : the router keeps copies
///   PartialEq : lets us compare two routes with `==` (used to highlight the active link)
///   Routable  : Dioxus macro that generates URL <-> enum conversion code
#[derive(Debug, Clone, PartialEq, Routable)]
#[rustfmt::skip]
pub enum Route {
    // Everything inside layout/end_layout is drawn inside `AppLayout`.
    #[layout(AppLayout)]
        #[route("/")]
        Home {},
        #[route("/games")]
        Games {},
        #[route("/about")]
        About {},
    #[end_layout]

    // Catch-all: any other URL. `segments` receives the path pieces, e.g.
    // "/a/b/c" -> vec!["a", "b", "c"]. This variant CARRIES DATA.
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
