# Lesson 08 - CHANGES

Previous: lesson 07 (two games, rules in modules).

## Commands

```bash
cp -r rdx-07-rock-paper-scissors rdx-08-tic-tac-toe
cd rdx-08-tic-tac-toe
mkdir src/modules/games/tictactoe
# add/edit files below, then
cargo clippy --all-targets
cargo test
dx serve
cargo run --example exp_borrowing
```

## Files

| File | Change |
|------|--------|
| `Cargo.toml` | name only |
| `src/config/mod.rs` | `LESSON_TAG = "lesson 08"` |
| `src/modules/games/tictactoe/mod.rs` | **new**: `Player`, `Cell`, `Status`, `MoveError`, `LINES`, `Board`, slice helpers, 10 tests |
| `src/modules/games/mod.rs` | `pub mod tictactoe;` |
| `src/routes/games/tictactoe.rs` | **new**: the page |
| `src/routes/games/mod.rs` | `pub mod tictactoe;`, `"tictactoe" => Some(Route::TicTacToe {})` |
| `src/routes/mod.rs` | `Route::TicTacToe` at `/games/tictactoe`; `is_in_games` lists it |
| `assets/main.css` | `.board`, `.cell` |
| `examples/exp_borrowing.rs` | **new** |

No new dependencies. No `random` needed: this game has no randomness, so the page creates its `Board` immediately (no `Option`, no `spawn`).

## Interesting code

The core method, validate then change:

```rust
pub fn play(&mut self, index: usize) -> Result<Status, MoveError> {
    if self.status != Status::InProgress { return Err(MoveError::GameOver); }
    match self.cells.get(index) {
        None                 => return Err(MoveError::OutOfBounds(index)),
        Some(Cell::Taken(_)) => return Err(MoveError::Occupied(index)),
        Some(Cell::Empty)    => {}
    }
    put_mark(&mut self.cells, index, self.next);
    self.status = if let Some(winner) = winner_of(&self.cells) { Status::Won(winner) }
                  else if is_full(&self.cells) { Status::Draw }
                  else { self.next = self.next.other(); Status::InProgress };
    Ok(self.status)
}
```

The page's click handler:

```rust
let mut click = move |index: usize| {
    let result = board.write().play(index);   // guard lives only for this statement
    match result { Ok(_) => error.set(String::new()), Err(e) => error.set(e.to_string()) }
};
```

## Build problems met

1. `clippy::collapsible_if` on a nested `if let` + `if`. Rewritten as a `match` with a **guard** (`Cell::Taken(player) if cells[b] == cells[a] && ... => Some(player)`), which also reads better.
2. `Board::new()` without `Default` triggers `clippy::new_without_default`; an `impl Default for Board` was added.

## Verify

1. `cargo test`: `27 passed`.
2. In the browser: X and O alternate; clicking a used cell shows "Cell N is already taken." and the turn does not change; after a win every further click shows "The game is over."; New game resets.
3. `cargo run --example exp_borrowing`: six sections. Un-comment each `ERROR` line once.
4. Break the rule on purpose: in `play`, move the `put_mark(...)` line above the `match self.cells.get(index)`. Which tests fail?
