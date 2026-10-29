//! EXPERIMENT: closures, what they capture, and Fn / FnMut / FnOnce.
//!
//! Run:  cargo run --example exp_closures
//!
//! A closure is a function without a name that can USE variables from the
//! place where it is written ("capture" them). Every event handler in
//! Dioxus (`onclick: move |_| ...`) is a closure.

fn main() {
    // -- 1. the simplest closure -----------------------------------------
    let add_one = |x: i32| x + 1; // parameters between | |
    println!("1. add_one(4) = {}", add_one(4));

    // -- 2. capture by BORROW (shared) ------------------------------------
    let name = String::from("Fox");
    let greet = || println!("2. hello {name}"); // only reads `name`: borrows it
    greet();
    greet();
    println!("   name is still usable: {name}");

    // -- 3. capture by MUTABLE borrow: FnMut ------------------------------
    // The closure CHANGES `count`, so it is an FnMut and the closure variable
    // itself must be declared `mut`.
    let mut count = 0;
    let mut click = || {
        count += 1;
        println!("3. clicked {count} time(s)");
    };
    click();
    click();
    // println!("{count}");   // ERROR if used here, while `click` may still run later:
    //                        // `count` is mutably borrowed by the closure.
    //                        // After the LAST use of `click`, it is free again:
    println!("   after the last click: count = {count}");

    // -- 4. `move`: the closure takes OWNERSHIP of what it captures --------
    let title = String::from("Snake");
    let show = move || println!("4. playing {title}");
    show();
    // println!("{title}");   // ERROR: `title` was moved into the closure

    // -- 5. `move` + Copy types: the closure gets its own COPY -------------
    // i32 is Copy. The closure has a private copy; the original is unaffected.
    // (Dioxus `Signal<T>` is also Copy: every handler gets a handle to the
    // SAME shared data, which is why signals are pleasant to use in closures.)
    let mut score = 10;
    let mut bump = move || {
        score += 1;
        score
    };
    println!("5. bump() = {}, bump() = {}", bump(), bump());
    println!("   original score is still {score}");

    // -- 6. FnOnce: a closure that can run only ONE time -------------------
    // It moves a captured value OUT (here: returns the String), so a second
    // call would have nothing left to return.
    let secret = String::from("hidden");
    let reveal = move || secret;
    let value = reveal();
    // let again = reveal();  // ERROR: closure cannot be called twice, `secret` was moved out
    println!("6. revealed {value}");

    // -- 7. closures as arguments -------------------------------------------
    // A function can accept ANY closure that matches a signature:
    fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
        f(f(x))
    }
    println!("7. apply_twice(add_one, 5) = {}", apply_twice(add_one, 5));
    println!("   apply_twice(|n| n * 3, 2) = {}", apply_twice(|n| n * 3, 2));

    // Iterator methods take closures too:
    let doubled: Vec<i32> = [1, 2, 3].iter().map(|n| n * 2).collect();
    println!("   map/collect = {doubled:?}");
}
