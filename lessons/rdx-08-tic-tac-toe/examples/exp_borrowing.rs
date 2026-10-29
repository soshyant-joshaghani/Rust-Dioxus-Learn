//! EXPERIMENT: borrowing rules, slices, and runtime borrow checking.
//!
//! Run:  cargo run --example exp_borrowing
//!
//! THE RULE: at any moment a value has EITHER
//!     any number of shared references (&T)      -> many readers
//!   OR exactly one mutable reference (&mut T)   -> one writer
//! never both. The compiler checks this before the program ever runs.

use std::cell::RefCell;

fn main() {
    // -- 1. many readers -----------------------------------------------------
    let board = [1, 2, 3];
    let a = &board;
    let b = &board; // two shared borrows at once: fine
    println!("1. {a:?} {b:?}");

    // -- 2. one writer -------------------------------------------------------
    let mut cells = [0, 0, 0];
    let writer = &mut cells;
    writer[0] = 9;
    // println!("{cells:?}");   // ERROR while `writer` is still used below:
    //                          // cannot borrow `cells` as immutable because it is also borrowed as mutable
    writer[1] = 8;
    println!("2. after the writer is done: {cells:?}"); // fine: the last use of `writer` was above

    // -- 3. a reader blocks the writer ---------------------------------------
    let mut score = vec![1, 2, 3];
    let first = &score[0]; // shared borrow of the vector
    // score.push(4);          // ERROR: cannot borrow `score` as mutable because it is also borrowed as immutable
    //                         // (push might move the memory, which would leave `first` dangling!)
    println!("3. first = {first}");
    score.push(4); // fine now: `first` is no longer used
    println!("   {score:?}");

    // -- 4. slices: borrowed views -------------------------------------------
    let cells = [10, 20, 30, 40, 50, 60, 70, 80, 90];
    let top_row: &[i32] = &cells[0..3]; // a view of 3 elements, no copy
    let whole: &[i32] = &cells; // an array can be viewed as a slice too
    println!("4. top_row = {top_row:?}, len = {}, whole len = {}", top_row.len(), whole.len());
    println!("   sum of row = {}", sum(top_row));
    println!("   sum of all = {}", sum(whole)); // the SAME function accepts both

    // A mutable slice lets a function change part of an array:
    let mut grid = [0; 9];
    fill(&mut grid[3..6], 7); // fill only the middle row
    println!("   grid = {grid:?}");

    // -- 5. safe indexing -----------------------------------------------------
    let items = [1, 2, 3];
    println!("5. items.get(1) = {:?}, items.get(7) = {:?}", items.get(1), items.get(7));
    // println!("{}", items[7]);  // PANIC at run time: index out of bounds

    // -- 6. RefCell: the same rule, checked at RUN time -------------------------
    // Dioxus signals work like this: many parts of the program hold a handle to
    // the same data, so the compiler cannot prove who borrows what. `RefCell`
    // keeps a counter and checks the rule while the program runs.
    let shared = RefCell::new(vec![1, 2, 3]);

    {
        let reader1 = shared.borrow(); // shared borrow #1
        let reader2 = shared.borrow(); // shared borrow #2: fine
        println!("6. two readers: {} and {}", reader1.len(), reader2.len());

        // A writer while readers exist is refused. `try_borrow_mut` reports it as an
        // error value; plain `borrow_mut()` would PANIC ("already borrowed").
        match shared.try_borrow_mut() {
            Ok(_) => println!("   writer allowed (unexpected)"),
            Err(_) => println!("   writer refused while readers exist"),
        }
    } // readers dropped here

    shared.borrow_mut().push(4); // fine: nobody else is borrowing now
    println!("   after push: {:?}", shared.borrow());
}

/// Takes a slice: works for arrays, vectors and parts of them. Cannot change anything.
fn sum(values: &[i32]) -> i32 {
    let mut total = 0;
    for value in values {
        total += value; // `value` is &i32; `+=` accepts a reference
    }
    total
}

/// Takes a MUTABLE slice: may change the elements, but not the length.
fn fill(values: &mut [i32], with: i32) {
    for value in values.iter_mut() {
        *value = with; // `*` writes THROUGH the reference
    }
}
