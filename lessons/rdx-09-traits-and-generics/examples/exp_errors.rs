//! EXPERIMENT: building an error type, `From`, and the `?` operator.
//!
//! Run:  cargo run --example exp_errors
//!
//! Goal: read a "width x height" text like "3x4" and compute the area.
//! Several different things can go wrong. We want ONE error type for all of them.

use std::fmt;
use std::num::ParseIntError;

// One enum, one variant per way to fail.
#[derive(Debug)]
enum AreaError {
    MissingX,                 // no 'x' in the text
    BadNumber(ParseIntError), // a part was not a number; keep the original reason
    Zero,                     // a side was 0
}

impl fmt::Display for AreaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AreaError::MissingX => write!(f, "expected the form WIDTHxHEIGHT"),
            AreaError::BadNumber(e) => write!(f, "bad number: {e}"),
            AreaError::Zero => write!(f, "a side cannot be zero"),
        }
    }
}

impl std::error::Error for AreaError {}

// This is what lets `?` convert a ParseIntError into an AreaError automatically.
impl From<ParseIntError> for AreaError {
    fn from(e: ParseIntError) -> AreaError {
        AreaError::BadNumber(e)
    }
}

// ---- WITHOUT `?` (long and noisy) ------------------------------------------
fn area_long(text: &str) -> Result<u32, AreaError> {
    let (w, h) = match text.split_once('x') {
        Some(parts) => parts,
        None => return Err(AreaError::MissingX),
    };
    let w: u32 = match w.parse() {
        Ok(n) => n,
        Err(e) => return Err(AreaError::BadNumber(e)),
    };
    let h: u32 = match h.parse() {
        Ok(n) => n,
        Err(e) => return Err(AreaError::BadNumber(e)),
    };
    if w == 0 || h == 0 {
        return Err(AreaError::Zero);
    }
    Ok(w * h)
}

// ---- WITH `?` (same behaviour) ---------------------------------------------
fn area(text: &str) -> Result<u32, AreaError> {
    // `ok_or(...)` turns an Option into a Result: None -> Err(MissingX).
    let (w, h) = text.split_once('x').ok_or(AreaError::MissingX)?;
    let w: u32 = w.parse()?; // ParseIntError -> AreaError through `From`
    let h: u32 = h.parse()?;
    if w == 0 || h == 0 {
        return Err(AreaError::Zero);
    }
    Ok(w * h)
}

// `main` can return a Result too. `Box<dyn Error>` means "any error type":
// the box holds it on the heap, `dyn` means "decided at run time". Handy at the
// top of a program, too vague for library code.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    for text in ["3x4", "3-4", "ax4", "0x4"] {
        println!("{text:>5}: long = {:?}, short = {:?}", area_long(text), area(text));
    }

    // Printing an error for a human uses Display:
    if let Err(e) = area("3-4") {
        println!("message: {e}");
    }

    // `?` in main: the first Err ends the program and prints the error.
    println!("area of 5x5 = {}", area("5x5")?);
    // Un-comment to see `main` fail with the Display/Debug of our error:
    // println!("{}", area("5y5")?);
    Ok(())
}
