//! `/games/snake`: Snake, version 2 (real time).
//!
//! The rules module is UNCHANGED apart from one new function (`tick_ms`).
//! The page now adds the three things a real-time game needs:
//!
//!   1. a LOOP that calls `step()` every few hundred milliseconds
//!   2. KEYBOARD input (and on-screen buttons for touch screens)
//!   3. a RUNNING flag: the game starts on the first key press, Space pauses
//!
//! ```text
//!        keyboard / buttons                       timer loop (use_future)
//!              |                                          |
//!      turn(direction)  <---- game: Signal<Option<SnakeGame>> ---->  step()
//!              |                       ^                  |
//!              `-- also sets running   |                  `-- reads `running`;
//!                                      |                      stops it on game over
//!                       page redraws whenever the signal changes
//! ```

use dioxus::prelude::*;

use crate::modules::base::hooks::use_seeded;
use crate::modules::base::time::sleep_ms;
use crate::modules::base::ui::card::Card;
use crate::modules::games::snake::{Direction, Pos, SnakeGame, Tile};
use crate::modules::games::{describe, Game};
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

/// Which direction does a key mean? `None` for every other key.
///
/// `Key` is an ENUM from Dioxus: `Key::ArrowUp`, `Key::Enter`, ... and
/// `Key::Character(String)` for letters. Matching on it is the same skill as
/// matching on our own enums. The `if` guard compares the letter inside.
fn direction_for_key(key: &Key) -> Option<Direction> {
    match key {
        Key::ArrowUp => Some(Direction::Up),
        Key::ArrowDown => Some(Direction::Down),
        Key::ArrowLeft => Some(Direction::Left),
        Key::ArrowRight => Some(Direction::Right),
        Key::Character(letter) if letter.eq_ignore_ascii_case("w") => Some(Direction::Up),
        Key::Character(letter) if letter.eq_ignore_ascii_case("s") => Some(Direction::Down),
        Key::Character(letter) if letter.eq_ignore_ascii_case("a") => Some(Direction::Left),
        Key::Character(letter) if letter.eq_ignore_ascii_case("d") => Some(Direction::Right),
        _ => None,
    }
}

#[component]
pub fn Snake() -> Element {
    let (mut game, restart) = use_seeded(SnakeGame::from_seed);
    // Is the game clock running? `false` = waiting for the first key, or paused.
    let mut running = use_signal(|| false);

    // ---- the game loop -------------------------------------------------------
    // `use_future` starts an async task when the component first appears and
    // CANCELS it when the component disappears (you navigate to another page).
    // So the loop can never keep running in the background.
    use_future(move || async move {
        loop {
            // How long to wait this tick? The game gets faster as the score grows.
            // IMPORTANT: this `read()` guard is a temporary: it is gone at the end of
            // the statement, BEFORE the `.await`. A guard held across an `.await` would
            // block every other reader/writer of the signal for the whole wait.
            let delay = game.read().as_ref().map_or(150, |g| g.tick_ms());
            sleep_ms(delay).await;

            if !running() {
                continue; // paused: skip this tick, wait again
            }

            // Take the write guard, step once, remember whether the game ended,
            // and let the guard go (end of the block) before touching `running`.
            let finished = {
                let mut slot = game.write();
                match slot.as_mut() {
                    Some(g) => {
                        let _ = g.step(); // Err(GameOver) cannot happen here; ignoring is fine
                        g.is_over()
                    }
                    None => false,
                }
            };
            if finished {
                running.set(false);
            }
        }
    });

    // ---- input -------------------------------------------------------------------
    let mut turn = move |direction: Direction| {
        // `accepted` is a plain bool: the write guard is gone by the next line.
        let accepted = match game.write().as_mut() {
            Some(g) => g.turn(direction).is_ok(),
            None => false,
        };
        if accepted {
            running.set(true); // the first valid turn starts the clock
        }
    };

    let on_key = move |event: KeyboardEvent| {
        // Without this the browser would also scroll the page on arrow keys / space.
        event.prevent_default();

        if let Some(direction) = direction_for_key(&event.key()) {
            turn(direction);
        } else if event.key() == Key::Character(" ".to_string()) {
            // Space toggles pause. `!running()` flips the bool.
            running.set(!running());
        }
    };

    // ---- what to draw ---------------------------------------------------------------
    let (status, width, rows) = match game.read().as_ref() {
        None => (String::from("Starting..."), 1, Vec::new()),
        Some(g) => {
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
    let hint = if running() { "Space pauses." } else { "Arrow keys or WASD to move. Space pauses." };

    rsx! {
        Link { to: Route::Games {}, class: "muted", "< All games" }
        h2 { class: "title", "Snake" }

        Card {
            p { class: "message", "{status}" }

            // A <div> only receives key events when it can take focus: `tabindex: "0"`.
            // `onmounted` gives us the element; we focus it right away so the keys work
            // without a click first.
            div {
                class: "snake-board",
                style: "--cols: {width}",
                tabindex: "0",
                onkeydown: on_key,
                onmounted: move |event| async move {
                    let _ = event.set_focus(true).await;
                },
                for row in rows {
                    for class in row {
                        div { class }
                    }
                }
            }
            p { class: "muted", "{hint}" }

            // The same controls as buttons, for phones and tablets without a keyboard.
            div { class: "dpad",
                button { class: "btn secondary up", onclick: move |_| turn(Direction::Up), "Up" }
                button { class: "btn secondary left", onclick: move |_| turn(Direction::Left), "Left" }
                button { class: "btn secondary right", onclick: move |_| turn(Direction::Right), "Right" }
                button { class: "btn secondary down", onclick: move |_| turn(Direction::Down), "Down" }
            }
            div { class: "row",
                button { class: "btn", onclick: move |_| running.set(!running()),
                    if running() { "Pause" } else { "Start" }
                }
                button {
                    class: "btn secondary",
                    onclick: move |_| {
                        restart();
                        running.set(false);
                    },
                    "New game"
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrows_and_letters_map_to_directions() {
        assert_eq!(direction_for_key(&Key::ArrowUp), Some(Direction::Up));
        assert_eq!(direction_for_key(&Key::ArrowLeft), Some(Direction::Left));
        assert_eq!(direction_for_key(&Key::Character("d".to_string())), Some(Direction::Right));
        assert_eq!(direction_for_key(&Key::Character("W".to_string())), Some(Direction::Up));
    }

    #[test]
    fn other_keys_mean_nothing() {
        assert_eq!(direction_for_key(&Key::Enter), None);
        assert_eq!(direction_for_key(&Key::Character("x".to_string())), None);
    }
}
