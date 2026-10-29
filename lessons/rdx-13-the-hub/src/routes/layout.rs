//! The frame around every page: header with navigation, the page, footer.
//!
//! WHY IS THIS IN `routes/` AND NOT IN `modules/base/`?
//!   The navigation must know the `Route` enum (where each link goes).
//!   `modules` is not allowed to depend on `routes` (see main.rs), so the
//!   piece that knows about routes lives in `routes`. The generic `Header`
//!   in `modules/base` just receives the links as `children`.

use dioxus::prelude::*;

use crate::modules::base::{footer::Footer, header::Header};
use crate::routes::Route;

/// The entries of the navigation bar.
///
/// An enum with only simple options, no data. It exists so that the nav bar
/// is written ONCE as a list instead of three copy-pasted links.
///
/// Derives: `Clone + Copy` (cheap to pass around by value), and `PartialEq`
/// because component props must be comparable (see lesson 02).
#[derive(Clone, Copy, PartialEq)]
enum NavItem {
    Home,
    Games,
    About,
}

/// All items in display order. A fixed-size array of three `NavItem`s.
const NAV_ITEMS: [NavItem; 3] = [NavItem::Home, NavItem::Games, NavItem::About];

impl NavItem {
    /// The text shown on the link.
    ///
    /// `match` must cover EVERY variant. Add `Settings` to the enum and this
    /// function will refuse to compile until you decide what its label is.
    fn label(self) -> &'static str {
        match self {
            NavItem::Home => "Home",
            NavItem::Games => "Games",
            NavItem::About => "About",
        }
    }

    /// Is this nav item the section we are in? `Games` also covers the pages of
    /// individual games, so a plain `==` is not enough any more.
    ///
    /// `matches!(value, pattern | pattern)` is a `match` that answers yes/no.
    fn is_active(self, current: &Route) -> bool {
        match self {
            NavItem::Home => matches!(current, Route::Home {}),
            NavItem::Games => current.is_in_games(),
            NavItem::About => matches!(current, Route::About {}),
        }
    }

    /// Where the link goes: convert a NavItem into a `Route`.
    fn route(self) -> Route {
        match self {
            NavItem::Home => Route::Home {},
            NavItem::Games => Route::Games {},
            NavItem::About => Route::About {},
        }
    }
}

/// Wraps every page. `Outlet::<Route>` is the hole where the current page appears.
#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div { class: "page",
            Header {
                for item in NAV_ITEMS {
                    NavLink { item }
                }
            }
            main { Outlet::<Route> {} }
            Footer {}
        }
    }
}

/// One link of the navigation bar, highlighted when its page is open.
#[component]
fn NavLink(item: NavItem) -> Element {
    // The route we are on right now (a hook: it re-renders us when the route changes).
    let current: Route = use_route();
    let target = item.route();

    // `&current` BORROWS the route: `is_active` only needs to look at it.
    let class = if item.is_active(&current) { "nav-link active" } else { "nav-link" };

    rsx! {
        Link { to: target, class: "{class}", "{item.label()}" }
    }
}
