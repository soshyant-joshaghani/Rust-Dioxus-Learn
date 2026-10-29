# Lesson 08 - Tic-Tac-Toe

> Goal: a two-player board game. This lesson is about **borrowing**: `&self` vs `&mut self`, passing `&mut` to helper functions, **slices**, and how Rust refuses an illegal move before it can corrupt the board.

## What you build

`/games/tictactoe`: a 3x3 grid. X starts, players alternate by clicking, the top line says whose turn it is, who won, or "Draw". Clicking a taken cell shows an error and does **not** change whose turn it is.

## What you learn

| Rust | Project |
|------|---------|
| arrays `[Cell; 9]`, nested arrays `[[usize; 3]; 8]` | the board and the 8 winning lines |
| enum carrying data: `Cell::Taken(Player)` | a cell is empty **or** owned by a player |
| `&self` / `&mut self` / `&mut [T]` / `&[T]` | `play`, `cell`, `put_mark`, `winner_of` |
| **slices** `&[Cell]` | one helper works on any run of cells |
| `Option` from `get`, no panics on bad input | `cell()`, `play()` bounds check |
| validate first, change later | refused moves leave the board untouched |
| `iter().all`, `enumerate`, `copied` | first real iterator use |
| `RefCell` (runtime borrow check) | why signals behave as they do |

## Run

```bash
cd lessons/rdx-08-tic-tac-toe
cargo test                          # 27 tests
dx serve                            # /games/tictactoe
cargo run --example exp_borrowing
```

## Files

```
src/modules/games/tictactoe/mod.rs   Player, Cell, Status, MoveError, Board (+10 tests)
src/routes/games/tictactoe.rs        the page (3x3 grid of buttons)
src/routes/mod.rs  games/mod.rs      Route::TicTacToe, route_for
assets/main.css                      .board, .cell
examples/exp_borrowing.rs            borrow rules, slices, RefCell
```

## Challenge

Add a **computer opponent** that plays O: after X moves successfully, pick the first empty cell (`cells.iter().position(|c| *c == Cell::Empty)`). Where does this code belong: the page or the `Board`? Why?

## Read next

`CHANGES.md` -> `KNOWLEDGE.md` -> [lesson 09](../rdx-09-traits-and-generics/README.md).
