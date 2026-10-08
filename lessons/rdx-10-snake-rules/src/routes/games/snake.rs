//! `/games/snake`: Snake, version 1 (turn by turn).
//!
//! There is no clock and no keyboard handling yet. You turn with the arrow
//! buttons and press "Step" to advance ONE tick. That is on purpose: it proves
//! the rules module works before time enters the picture. Lesson 11 replaces
//! the Step button with a timer and the arrow buttons with the keyboard.

use dioxus::prelude::*;

use crate::modules::base::hooks::use_seeded;
use crate::modules::base::ui::card::Card;
use crate::modules::games::snake::{Direction, Pos, SnakeGame, Tile};
use crate::modules::games::describe;
use crate::routes::Route;

/// The CSS class of one tile. A `match` on an enum again.
fn tile_class(tile: Tile) -> &'static str {
    match tile {
        Tile::Empty => "tile",
        Tile::Head => "tile head",
        Tile::Body => "tile body",
        Tile::Food => "tile food",
    }
}

#[component]
pub fn Snake() -> Element {
    let (mut game, restart) = use_seeded(SnakeGame::from_seed);

    // `turn` and `step` return Results. In this page we ignore the errors:
    // `let _ = ...` says "I know this returns a Result and I choose not to use it".
    // (A turn refused because of a reversal is simply not applied.)
    let mut turn = move |direction: Direction| {
        if let Some(g) = game.write().as_mut() {
            let _ = g.turn(direction);
        }
    };
    let mut step = move || {
        if let Some(g) = game.write().as_mut() {
            let _ = g.step();
        }
    };

    // Read everything the view needs while the guard is alive, then let it go:
    // the grid becomes a plain Vec of owned CSS classes.
    let (status, width, rows) = match game.read().as_ref() {
        None => (String::from("Starting..."), 1, Vec::new()),
        Some(g) => {
            // Nested loops build a Vec<Vec<&'static str>>: rows of tile classes.
            let mut rows = Vec::new();
            for y in 0..g.height() {
                let mut row = Vec::new();
                for x in 0..g.width() {
                    row.push(tile_class(g.tile_at(Pos { x, y })));
                }
                rows.push(row);
            }
            (describe(g), g.width(), rows)
        }
    };

    rsx! {
        Link { to: Route::Games {}, class: "muted", "< All games" }
        h2 { class: "title", "Snake" }

        Card {
            p { class: "message", "{status}" }

            // The grid has `width` equal columns (CSS custom property used by .snake-board).
            div { class: "snake-board", style: "--cols: {width}",
                for row in rows {
                    for class in row {
                        div { class }
                    }
                }
            }

            div { class: "row",
                button { class: "btn secondary", onclick: move |_| turn(Direction::Left), "Left" }
                button { class: "btn secondary", onclick: move |_| turn(Direction::Up), "Up" }
                button { class: "btn secondary", onclick: move |_| turn(Direction::Down), "Down" }
                button { class: "btn secondary", onclick: move |_| turn(Direction::Right), "Right" }
            }
            div { class: "row",
                button { class: "btn", onclick: move |_| step(), "Step" }
                button { class: "btn secondary", onclick: move |_| restart(), "New game" }
            }
        }
    }
}
