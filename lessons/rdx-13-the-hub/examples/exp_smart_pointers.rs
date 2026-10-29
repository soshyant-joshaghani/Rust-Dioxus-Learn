//! EXPERIMENT: smart pointers: Box, Rc, RefCell (and Arc, from lesson 11).
//!
//! Run:  cargo run --example exp_smart_pointers
//!
//! A SMART POINTER is a value that owns data elsewhere (usually on the heap) and
//! adds a rule: when to free it, who may share it, who may change it.
//!
//!   Box<T>         one owner, data on the heap
//!   Rc<T>          MANY owners (single thread); freed when the last owner goes
//!   Arc<T>         like Rc, but safe across threads
//!   RefCell<T>     change data through a shared reference; rules checked at RUN time
//!   Rc<RefCell<T>> many owners who can all change the same value (single thread)
//!
//! A Dioxus `Signal<T>` is, in spirit, a handle to shared data with run-time borrow
//! checks: the same idea as Rc<RefCell<T>>, with change notification added.

use std::cell::RefCell;
use std::rc::Rc;

// -- 1. Box: a recursive type needs a pointer --------------------------------------------
// A list that contains a list that contains a list... would have infinite size.
// `Box` puts the rest on the heap, so each node has a known size.
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn sum(&self) -> i32 {
        match self {
            List::Cons(value, rest) => value + rest.sum(), // `rest` is a &Box<List>; auto-deref
            List::Nil => 0,
        }
    }
}

// -- 2. Rc + RefCell: shared, changeable data -----------------------------------------------
// Imagine two parts of a game both holding the same score.
struct Scoreboard {
    score: Rc<RefCell<u32>>,
}

fn main() {
    // 1. Box
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("1. {list:?} sum = {}", list.sum());

    // 2. Rc: many owners
    let name = Rc::new(String::from("Fox"));
    println!("2. strong count after creation: {}", Rc::strong_count(&name)); // 1
    let owner_a = Rc::clone(&name); // NOT a deep copy: just one more owner, count +1
    {
        let owner_b = Rc::clone(&name);
        println!("   inside the block: {} owners ({owner_a}, {owner_b})", Rc::strong_count(&name)); // 3
    } // owner_b dropped: count -1
    println!("   after the block: {} owners", Rc::strong_count(&name)); // 2
    // An Rc gives shared READ access only. `*name = ...` is not allowed:
    // that is what RefCell adds.

    // 3. Rc<RefCell<T>>: many owners that may all write
    let shared = Rc::new(RefCell::new(0u32));
    let board_a = Scoreboard { score: Rc::clone(&shared) };
    let board_b = Scoreboard { score: Rc::clone(&shared) };

    *board_a.score.borrow_mut() += 10; // exclusive borrow, checked now, released at the end of the statement
    *board_b.score.borrow_mut() += 5;
    println!("3. both boards see {} and {}", board_a.score.borrow(), board_b.score.borrow());

    // The run-time check: a second mutable borrow while the first is alive PANICS.
    let first = shared.borrow_mut();
    let second = shared.try_borrow_mut(); // the polite version: returns Err instead of panicking
    println!("   second borrow while the first is alive: {}", if second.is_err() { "refused" } else { "allowed" });
    drop(first);
    println!("   after dropping the first: {}", if shared.try_borrow_mut().is_ok() { "allowed" } else { "refused" });

    // 4. Rc cannot cross threads; try it (un-comment):
    //
    // std::thread::spawn(move || println!("{name}"));
    // ERROR: `Rc<String>` cannot be sent between threads safely. Use `Arc` (lesson 11).

    // 5. Cycles leak: two Rc pointing at each other never reach count 0. Rust treats a leak
    //    as SAFE (not a crash), so the compiler will not stop you. `Weak<T>` is the tool
    //    for back-pointers; we do not need it here.
}
