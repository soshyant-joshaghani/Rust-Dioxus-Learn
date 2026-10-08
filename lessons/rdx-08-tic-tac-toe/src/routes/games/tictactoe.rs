//! `/games/tictactoe`: two players on one screen.
//!
//! The whole game is one `Board` inside one signal.
//!
//!   click cell i -> board.write().play(i) -> Result<Status, MoveError>
//!                      Ok  -> board changed, page redraws
//!                      Err -> board unchanged, we show the error text

use dioxus::prelude::*;

use crate::modules::base::ui::card::Card;
use crate::modules::games::tictactoe::{Board, Cell, Status};
use crate::routes::Route;

/// Text for the top line, from the board's status. `match` on an enum with data.
fn status_text(board: &Board) -> String {
    match board.status() {
        Status::InProgress => format!("{}'s turn", board.next_player().label()),
        Status::Won(player) => format!("{} wins!", player.label()),
        Status::Draw => String::from("Draw."),
    }
}

#[component]
pub fn TicTacToe() -> Element {
    // No random numbers here, so no `Option`: `Board::new` is called at once.
    // `use_signal` runs the closure only on the first render.
    let mut board = use_signal(Board::new);
    let mut error = use_signal(String::new);

    // One handler for all nine buttons: it receives the cell index.
    let mut click = move |index: usize| {
        // WRITE guard in a short block, then touch other signals after it.
        let result = board.write().play(index);

        match result {
            Ok(_) => error.set(String::new()),
            Err(e) => error.set(e.to_string()),
        }
    };

    // Everything the view needs is read here, once: the guard on `board` is
    // a temporary that ends with each statement.
    let status = status_text(&board.read());
    let cells: Vec<Cell> = (0..9).map(|i| board.read().cell(i).unwrap_or_default()).collect();

    rsx! {
        Link { to: Route::Games {}, class: "muted", "< All games" }
        h2 { class: "title", "Tic-Tac-Toe" }

        Card {
            p { class: "message", "{status}" }

            div { class: "board",
                // `enumerate` pairs every cell with its index: (0, cell0), (1, cell1)...
                for (index, cell) in cells.into_iter().enumerate() {
                    button {
                        class: "cell",
                        onclick: move |_| click(index),
                        match cell {
                            Cell::Empty => "",
                            Cell::Taken(player) => player.label(),
                        }
                    }
                }
            }

            p { class: "muted", "{error}" }
            button {
                class: "btn secondary",
                onclick: move |_| {
                    board.write().reset();
                    error.set(String::new());
                },
                "New game"
            }
        }
    }
}
