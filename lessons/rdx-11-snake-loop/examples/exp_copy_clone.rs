//! EXPERIMENT: Copy vs Clone vs Move, with our own types.
//!
//! Run:  cargo run --example exp_copy_clone
//!
//! Three behaviours when you write `let b = a;` or pass `a` to a function:
//!   Copy  : the bits are duplicated, `a` stays valid      (numbers, small enums)
//!   Move  : ownership transfers, `a` is gone               (String, Vec, most structs)
//!   Clone : YOU ask for a real duplicate with `.clone()`   (when Copy is not allowed)

// All fields are Copy -> the struct may be Copy.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Contains a String -> NOT allowed to be Copy (copying would hide an allocation).
// Clone is allowed: the duplicate is explicit.
#[derive(Debug, Clone)]
struct Player {
    name: String,
    position: Point,
}

// #[derive(Clone, Copy)]
// struct Broken { name: String }   // ERROR: the trait `Copy` cannot be implemented: `String` is not Copy

fn describe_point(p: Point) -> String {
    format!("({}, {})", p.x, p.y)
}

fn describe_player(p: Player) -> String {
    // takes the Player BY VALUE: the caller's Player is moved in
    format!("{} at {}", p.name, describe_point(p.position))
}

fn main() {
    // -- Copy ----------------------------------------------------------------
    let a = Point { x: 1, y: 2 };
    let b = a; // copy
    println!("1. a = {a:?}, b = {b:?} (both valid)");

    let text = describe_point(a); // `a` is copied into the function
    println!("2. {text}; a is still {a:?}");

    // -- Move ----------------------------------------------------------------
    let fox = Player { name: String::from("Fox"), position: a };
    let other = fox; // MOVE: fox is gone
    // println!("{fox:?}");     // ERROR: borrow of moved value: `fox`
    println!("3. other = {other:?}");

    // -- Clone ---------------------------------------------------------------
    let copy = other.clone(); // a deep, independent duplicate (allocates a new String)
    println!("4. clone = {copy:?}");

    // Changing the clone does not affect the original:
    let mut changed = other.clone();
    changed.name.push_str("-2");
    changed.position.x = 99;
    println!("5. original = {other:?}");
    println!("   changed  = {changed:?}");

    // -- passing by value moves; a clone keeps ours -----------------------------
    let summary = describe_player(other.clone()); // we keep `other`
    println!("6. {summary}; still have {other:?}");
    let summary2 = describe_player(other); // now `other` is moved
    println!("7. {summary2}");
    // println!("{other:?}");   // ERROR: `other` was moved into describe_player

    // -- the better way: borrow --------------------------------------------------
    // If a function only needs to LOOK, take a reference: no clone, no move.
    fn describe_player_ref(p: &Player) -> String {
        format!("{} at {}", p.name, describe_point(p.position))
    }
    println!("8. {}; and `copy` is still ours: {}", describe_player_ref(&copy), copy.name);

    // -- Copy things inside non-Copy things --------------------------------------
    // Reading a Copy field out of a borrowed struct just copies it:
    let p: Point = copy.position;
    println!("9. position copied out: {p:?}; copy.name still intact: {}", copy.name);
}
