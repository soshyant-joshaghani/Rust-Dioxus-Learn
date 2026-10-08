# Lesson 08 - KNOWLEDGE (the book part)

Read with `src/modules/games/tictactoe/mod.rs` and `examples/exp_borrowing.rs`.

---

## 1. The borrowing rule, stated once, carefully

> At any moment, for a given piece of data, you may have **either** any number of shared references (`&T`) **or** exactly one mutable reference (`&mut T`). Never both. And no reference may outlive the data it points to.

Why? Two readers cannot hurt each other. One writer alone cannot hurt anyone. A writer **plus** a reader can: the reader might see half-changed data, or a pointer that the writer just invalidated (`push` on a `Vec` may move its memory). Most security bugs in C/C++ are exactly this situation. Rust makes it a **compile error**.

### What counts as "at the same time"?

A borrow lasts from where it is created to its **last use** (not to the end of the block). This is called *non-lexical lifetimes*:

```rust
let writer = &mut cells;
writer[0] = 9;
writer[1] = 8;                 // last use of `writer`
println!("{cells:?}");         // fine: the mutable borrow has ended
```

`exp_borrowing.rs` sections 1-3 show each rule with its error.

## 2. `&self`, `&mut self`, `self` in `Board`

| Method | Receiver | Why |
|--------|----------|-----|
| `play` | `&mut self` | changes cells, turn and status |
| `cell`, `status`, `next_player` | `&self` | only reads; many readers may call them at once |
| `reset` | `&mut self` | replaces the whole board |

The signature **documents the permission**. When you see `&self` you know the call cannot change the board. The compiler enforces that promise.

### `*self = Board::new()`

Inside `reset(&mut self)`, `self` is a reference. `*self` means "the value it points to". Assigning to `*self` writes a new `Board` into the caller's variable and **drops** the old one. You are *replacing the contents*, not rebinding the reference.

## 3. Passing `&mut` into helpers

```rust
put_mark(&mut self.cells, index, self.next);
```

- `&mut self.cells` lends one field mutably. `self.next` is read in the same call: `Player` is `Copy`, so it is copied out first. Different fields can be borrowed independently.
- Inside `put_mark(cells: &mut [Cell], ...)` the function may assign `cells[index] = ...`, but it **cannot keep** the reference, store it somewhere, or return it. The borrow ends when the function returns.

Compare with passing `self.cells` by value: arrays of `Copy` items are `Copy`, so the function would get a **copy** of the board, change the copy, and the real board would stay unchanged. A classic bug; `&mut` is how you say "change *this* one".

## 4. Slices

```rust
fn is_full(cells: &[Cell]) -> bool
fn winner_of(cells: &[Cell]) -> Option<Player>
```

A **slice** `&[T]` is a borrowed view: a pointer to the first element + a length. Not an owner, nothing is copied.

| You have | You can pass it as a slice with |
|----------|---------------------------------|
| array `[Cell; 9]` | `&cells` or `&cells[..]` |
| `Vec<Cell>` | `&vec` or `&vec[..]` |
| part of either | `&cells[3..6]` (elements 3, 4, 5) |

Prefer `&[T]` over `&[T; 9]` or `&Vec<T>` in function parameters: one function serves all, and it states "I only need a run of cells". `&mut [T]` is the writable variant: it can change elements but **not the length**.

Ranges: `a..b` excludes `b`, `a..=b` includes it, `..` is everything, `a..` / `..b` are open.

## 5. Arrays: fixed size, inline

```rust
const LINES: [[usize; 3]; 8] = [[0,1,2], [3,4,5], ...];
cells: [Cell; 9]
[Cell::Empty; 9]          // repeat expression: nine copies (needs Cell: Copy)
```

The length is part of the type. `[Cell; 9]` and `[Cell; 8]` are different types. Arrays live inline (no heap) and are fast. When the size is not known at compile time, use `Vec<T>` (Snake, lesson 10).

`for line in LINES` iterates the array **by value** (edition 2021+); `[usize; 3]` is `Copy`, so each `line` is a copy.

`let [a, b, c] = line;` destructures an array into three variables.

## 6. Never panic on user input

```rust
match self.cells.get(index) {
    None => return Err(MoveError::OutOfBounds(index)),
    ...
}
```

`cells[index]` would **panic** when `index >= 9`: the program stops. `get(index)` returns `Option<&Cell>`; we turn `None` into an error value. A game logic function must be safe against *any* `usize`, because a UI bug or hostile input could send one. The test `play(usize::MAX)` proves it.

**Panic vs Result:** a panic means "the program is wrong". A `Result::Err` means "the request was wrong". Index from a loop you control: indexing is fine. Index from outside: `get`.

## 7. Validate first, mutate later

```
1. if game over  -> Err
2. if bad cell   -> Err
3. now change the board
4. recompute status
```

Because every `Err` return happens **before** any change, a refused move cannot leave a half-updated board. The test `occupied_cell_is_refused_and_turn_is_kept` pins that down. This pattern ("check, then act") makes methods *transactional* without any special machinery.

## 8. Enum carrying data, again

```rust
enum Cell { Empty, Taken(Player) }
```

A cell is **either empty or owned by a player**: there is no "Taken but nobody" state to check for. Compare `struct Cell { taken: bool, owner: Player }`, where `taken = false` with `owner = X` is meaningless but representable. **Make illegal states unrepresentable.**

Pattern matching reaches into the data:

```rust
match cells[a] {
    Cell::Taken(player) if cells[b] == cells[a] && cells[c] == cells[a] => Some(player),
    _ => None,
}
```

`#[derive(Default)]` with `#[default]` on `Empty` makes `Cell::default()` the empty cell: that is what `unwrap_or_default()` returns in the page for a missing cell.

## 9. Iterators in two lines

```rust
cells.iter().all(|cell| *cell != Cell::Empty)         // "is every cell taken?"
(0..9).map(|i| board.read().cell(i).unwrap_or_default()).collect::<Vec<Cell>>()
for (index, cell) in cells.into_iter().enumerate() { ... }
```

- An iterator is a lazy sequence; adaptors (`map`, `filter`, `enumerate`) describe a pipeline, **consumers** (`all`, `collect`, `for`) run it.
- `|cell| ...` are closures (lesson 05).
- `.iter()` yields `&T`, so we write `*cell` to compare values.
- `.copied()` turns `Option<&T>` into `Option<T>` for `Copy` types (used in `Board::cell`).
- `.collect()` needs to know the target collection: `Vec<Cell>` here (from the `let` type annotation).

## 10. `RefCell` and why signals "panic with already borrowed"

The compiler checks the borrow rule only when it can see all the borrows. Shared mutable data (a signal read by many components and closures) is the opposite case. `RefCell<T>` moves the check to run time:

| Method | Behaviour |
|--------|-----------|
| `borrow()` | shared borrow; panics if a writer exists |
| `borrow_mut()` | exclusive borrow; panics if anything else exists |
| `try_borrow_mut()` | returns `Result` instead of panicking |

A Dioxus `Signal` read guard / write guard follow the same idea. So in `tictactoe.rs`:

```rust
let result = board.write().play(index);
```

the write guard is a temporary that is dropped at the end of the statement, and only afterwards do we touch `error`. Never keep a guard while calling code that may touch the same signal.

## 11. Who owns what?

| Value | Owner |
|-------|-------|
| `Board` | the signal's storage (we hold a `Copy` handle) |
| `cells` array inside `Board` | the `Board` (inline, no heap) |
| `status_text(&board.read())` | page borrows the board for one call, receives an owned `String` |
| `cells: Vec<Cell>` in the page | the page; built from copies (`Cell: Copy`) so no guard is kept during drawing |
| the error `String` | moved into the `error` signal |

## 12. Experiments

1. `cargo run --example exp_borrowing`; un-comment each `ERROR`.
2. Change `put_mark(&mut self.cells, ...)` to `put_mark(self.cells, ...)` and read the error about types. Change its parameter to `[Cell; 9]` too and run the tests: which fail, and why? (Arrays are `Copy` here, so the function mutates a copy.)
3. Add the computer opponent from the challenge **inside `Board`**, as `pub fn play_computer(&mut self)`. Write the test first.
4. Make `LINES` a `Vec` built at run time. What do you lose? (Hint: `const` can be used anywhere; the array is in the binary.)
5. In the page, replace `board.write().play(index)` with `let mut b = board.write(); let result = b.play(index); error.set(...)` and make the handler touch `board.read()` while `b` is alive. Observe the panic message.
6. Replace the `for line in LINES` loop in `winner_of` by `LINES.iter().find_map(|line| line_owner(cells, *line))`. Same result; which is easier to read for you?

## 13. Key vocabulary

reference, shared / mutable borrow, borrow checker, non-lexical lifetime, slice, range, array, destructuring, `get`, panic, validate-then-mutate, illegal state, `RefCell`, guard, iterator adaptor, consumer.

## 14. Self-check

- State the borrowing rule in one sentence.
- Why does `put_mark` take `&mut [Cell]` and not `[Cell; 9]`?
- What is the difference between `cells[i]` and `cells.get(i)`?
- Why does `Cell::Taken(Player)` beat a struct with a `taken: bool` field?
- Why does `board.write().play(i)` not panic, but holding the guard in a variable and reading the board might?
