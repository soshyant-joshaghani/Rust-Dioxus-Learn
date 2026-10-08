//! EXPERIMENT: ownership, move, clone, borrow.
//!
//! Run:  cargo run --example exp_ownership
//!
//! This file is NOT part of the app. It is a tiny lab where each section
//! shows one behaviour of Rust. Un-comment the lines marked `// ERROR` one at
//! a time to read the compiler message. They are meant to fail.

fn main() {
    // -- 1. MOVE ----------------------------------------------------------
    // A String owns heap memory. Assigning it to another variable MOVES the
    // ownership; the old variable can no longer be used.
    let a = String::from("fox");
    let b = a; // ownership moves: a -> b
    // println!("{a}");        // ERROR: borrow of moved value: `a`
    println!("1. after move, b = {b}");

    // -- 2. CLONE ---------------------------------------------------------
    // `clone()` makes a real second copy, so both stay valid. It costs time
    // and memory, and it is always visible in the code.
    let c = b.clone();
    println!("2. b = {b}, c = {c} (two separate Strings)");

    // -- 3. COPY ----------------------------------------------------------
    // Small plain types (numbers, bool, char) implement the `Copy` trait:
    // "assignment copies the bits", nothing is moved.
    let x = 5;
    let y = x;
    println!("3. x = {x}, y = {y} (both usable: i32 is Copy)");

    // -- 4. FUNCTION TAKES OWNERSHIP --------------------------------------
    let d = String::from("owl");
    consume(d); // d is moved INTO the function
    // println!("{d}");        // ERROR: d was moved into `consume`
    println!("4. consume() took the String and dropped it");

    // -- 5. BORROW (shared reference) -------------------------------------
    // `&e` lends the value. The owner keeps ownership. Any number of
    // shared borrows may exist at the same time, but nobody can change the
    // value while they exist.
    let e = String::from("hare");
    let len = length_of(&e);
    println!("5. '{e}' has {len} bytes (we only borrowed it)");

    // -- 6. MUTABLE BORROW ------------------------------------------------
    // `&mut` lends with permission to change. Rule: while a `&mut` exists,
    // NO other reference to the value may exist.
    let mut f = String::from("wolf");
    add_exclamation(&mut f);
    println!("6. after add_exclamation: {f}");
    // let r1 = &mut f;
    // let r2 = &mut f;        // ERROR: cannot borrow `f` as mutable more than once
    // r1.push('x');
    // r2.push('y');

    // -- 7. RETURN MOVES OWNERSHIP BACK OUT -------------------------------
    let g = make_greeting("Fox");
    println!("7. {g} (created inside a function, moved out by the return)");
}

/// Takes ownership. The String is dropped at the end of this function.
fn consume(s: String) {
    println!("   consume got '{s}'");
}

/// Borrows. The caller keeps ownership.
/// We take `&str` (not `&String`): callers passing `&String` still work, because
/// Rust automatically views a `&String` as a `&str` ("deref coercion").
fn length_of(s: &str) -> usize {
    s.len()
}

/// Borrows mutably, so it may change the caller's value.
fn add_exclamation(s: &mut String) {
    s.push('!');
}

/// The String is created here and handed to the caller: a move OUT.
fn make_greeting(name: &str) -> String {
    format!("hello {name}")
}
