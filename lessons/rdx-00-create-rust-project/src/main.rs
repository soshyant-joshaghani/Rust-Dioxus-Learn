//! Lesson 00: "Fox Greeter", a tiny command line program.
//!
//! We started from the `Hello, world!` that `cargo new` gives us and grew it
//! a little, so that every line below teaches one basic Rust idea.
//!
//! Read it top to bottom. The order is:
//!   1. constants and `use`
//!   2. small functions (`fn`)
//!   3. `main`, which wires everything together

use std::io; // `use` brings a name into scope. `io` is the standard input/output module.

// A constant. It never changes, its type must be written out, and by
// convention the name is SHOUTING_CASE. `&str` is "a borrowed piece of text".
const APP_NAME: &str = "Fox Greeter";

// Pretend "today" is this year. (Asking the clock for the year needs more
// code than this lesson wants. We will not need it later.)
const CURRENT_YEAR: i32 = 2026;

// ---------------------------------------------------------------------------
// Functions
// ---------------------------------------------------------------------------

/// Print `question`, then wait for the user to type a line and press Enter.
///
/// Returns the line WITHOUT the trailing newline.
///
/// Why `String` and not `&str` as the return type?
///   The text is created inside this function (`line`). When the function
///   ends, `line` would be destroyed, so we cannot hand out a *borrow* of it.
///   Instead we hand out the String itself: ownership MOVES to the caller.
fn ask(question: &str) -> String {
    println!("{question}"); // `{question}` inserts the variable into the text

    // `let` creates a variable. Variables are immutable by default;
    // `mut` is the opt-in that lets `read_line` append into it.
    let mut line = String::new();

    // `&mut line` lends `line` out mutably, only for the length of this call.
    // `read_line` returns a Result: Ok(bytes_read) or Err(problem).
    // `.expect("...")` means "if it is an Err, crash with this message".
    // That is fine for a toy; later lessons handle errors properly.
    io::stdin().read_line(&mut line).expect("could not read from the keyboard");

    // `trim` returns a borrowed &str view without spaces and the newline.
    // `to_string` copies it into a brand new owned String that we can return.
    line.trim().to_string()
}

/// Work out how old someone is.
///
/// The last expression has NO semicolon, so it is the return value.
/// (Writing `return a - b;` also works, but Rust code usually uses the
/// "last expression" style.)
fn age_in(year_now: i32, year_born: i32) -> i32 {
    year_now - year_born
}

/// Keep asking until the user types a whole number.
///
/// `loop` repeats forever until we `break` or `return`.
fn ask_year() -> i32 {
    loop {
        let text = ask("What year were you born?");

        // `parse::<i32>()` tries to turn text into a number.
        // It cannot promise success, so it returns `Result<i32, ParseIntError>`.
        // `match` forces us to handle BOTH cases; the compiler checks we did.
        match text.parse::<i32>() {
            Ok(year) => return year,
            Err(_) => println!("'{text}' is not a number, try again."),
        }
    }
}

/// Return the greeting in `style`.
///
/// `match` on text compares against each arm in order. `_` is the "anything
/// else" arm. Every arm must produce the same type (here: String).
fn greet(name: &str, style: &str) -> String {
    match style {
        "shout" => name.to_uppercase() + "!!!", // String + &str builds a new String
        "whisper" => format!("psst... {}", name.to_lowercase()), // format! = println! that returns a String
        _ => format!("Hello, {name}!"),
    }
}

// ---------------------------------------------------------------------------
// main
// ---------------------------------------------------------------------------

fn main() {
    println!("=== {APP_NAME} ===");

    let name = ask("What is your name?");

    // Shadowing: a NEW variable that reuses the name `year`.
    // It is not mutation. The old binding is simply hidden.
    let year = ask_year();
    let year = age_in(CURRENT_YEAR, year); // now `year` means "age"

    println!("{} You are about {year} years old.", greet(&name, "normal"));

    // A small command loop. `mut` because `visits` changes every round.
    let mut visits: u32 = 0;
    loop {
        let command = ask("\nType: shout | whisper | count | quit");

        match command.as_str() {
            "shout" | "whisper" => println!("{}", greet(&name, &command)),
            "count" => {
                visits += 1; // `+=` is only allowed because of `mut`
                println!("You asked for the count {visits} time(s).");
            }
            "quit" => break, // leave the loop, and then main ends
            other => println!("I do not know '{other}'."),
        }
    }

    println!("Bye, {name}!");
}
