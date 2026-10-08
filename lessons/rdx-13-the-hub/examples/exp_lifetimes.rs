//! EXPERIMENT: lifetimes, "how long does a borrow stay valid?"
//!
//! Run:  cargo run --example exp_lifetimes
//!
//! A LIFETIME is the span of the program during which a reference is valid.
//! You almost never write them: the compiler infers them. You write them only
//! when a function or struct hands out a reference and the compiler cannot tell
//! WHICH input the output is borrowed from.

// -- 1. the problem -----------------------------------------------------------------
// This function returns one of its two inputs. Which one? The caller needs to know
// how long the result may be used, so the signature says it with a lifetime name `'a`:
//
//     fn longest<'a>(a: &'a str, b: &'a str) -> &'a str
//
// Read it as: "the result lives no longer than BOTH inputs".
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

// -- 2. elision: the cases where you do NOT write it -----------------------------------
// One input reference: the output must come from it. The compiler fills in the lifetime.
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

// -- 3. a struct that BORROWS ---------------------------------------------------------------
// `Line<'a>` holds a reference. The `'a` says: "a Line cannot outlive the text it points into".
struct Line<'a> {
    key: &'a str,
    value: &'a str,
}

impl<'a> Line<'a> {
    // `split_once` returns slices INTO `text`: no String is allocated. This is how
    // `Progress::from_text` works, and why it is cheap.
    fn parse(text: &'a str) -> Option<Line<'a>> {
        let (key, value) = text.split_once('=')?;
        Some(Line { key, value })
    }
}

// -- 4. 'static ------------------------------------------------------------------------------------
// String literals live for the entire program: type `&'static str`.
// That is why `GameInfo { title: "Snake" }` needs no ownership code.
fn greeting() -> &'static str {
    "hello"
}

fn main() {
    // 1.
    let a = String::from("fox");
    let b = String::from("hare");
    let winner = longest(&a, &b);
    println!("1. longest = {winner}");

    // The result is tied to BOTH inputs. This does not compile (un-comment to read):
    //
    // let result;
    // {
    //     let short_lived = String::from("owl");
    //     result = longest(&a, &short_lived);
    // }                                  // `short_lived` dies here
    // println!("{result}");              // ERROR: `short_lived` does not live long enough

    // 2.
    println!("2. first word = '{}'", first_word("snake game over"));

    // 3.
    let saved = String::from("snake_best=7");
    if let Some(line) = Line::parse(&saved) {
        println!("3. key = {}, value = {} (both borrowed from `saved`)", line.key, line.value);
    }
    // `saved` must outlive every `Line` made from it. Dropping it early is an error:
    //
    // let line = Line::parse(&saved).unwrap();
    // drop(saved);                       // ERROR: cannot move out of `saved` because it is borrowed
    // println!("{}", line.key);

    // 4.
    println!("4. {}", greeting());

    // A dangling reference can never be created in safe Rust:
    //
    // fn dangle() -> &String { let s = String::from("x"); &s }   // ERROR: missing lifetime / returns a
    //                                                           // reference to data owned by this function
    // The fix is to return the String itself (move ownership OUT):
    fn no_dangle() -> String {
        String::from("x")
    }
    println!("   no_dangle = {}", no_dangle());
}
