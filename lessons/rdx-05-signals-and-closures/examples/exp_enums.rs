//! EXPERIMENT: enums, match, Option and Result.
//!
//! Run:  cargo run --example exp_enums
//!
//! An enum is a value that is exactly ONE of several variants.
//! Variants may carry data. `match` takes it apart safely.

// Simple variants, like a list of choices.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Light {
    Red,
    Yellow,
    Green,
}

// Variants that carry different data. Each variant is its own shape.
#[derive(Debug)]
enum Shape {
    Circle(f64),               // one number: the radius
    Rect { w: f64, h: f64 },   // named fields
    Dot,                       // nothing
}

impl Light {
    /// An enum can have methods. `match` has to list ALL variants.
    fn next(self) -> Light {
        match self {
            Light::Red => Light::Green,
            Light::Green => Light::Yellow,
            Light::Yellow => Light::Red,
        }
    }
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r, // `r` is a reference to the f64 inside
            Shape::Rect { w, h } => w * h,
            Shape::Dot => 0.0,
        }
    }
}

fn main() {
    // -- 1. simple enum + match ------------------------------------------
    let mut light = Light::Red;
    for _ in 0..4 {
        print!("{light:?} -> ");
        light = light.next();
    }
    println!("{light:?}");

    // -- 2. enum with data -----------------------------------------------
    let shapes = [Shape::Circle(2.0), Shape::Rect { w: 3.0, h: 4.0 }, Shape::Dot];
    for s in shapes.iter() {
        println!("{s:?} has area {}", s.area());
    }

    // -- 3. Option<T>: "a value or nothing" -------------------------------
    // Rust has NO null. The possibility of "nothing" is part of the TYPE:
    //     enum Option<T> { Some(T), None }
    let numbers = [10, 20, 30];
    let found: Option<&i32> = numbers.iter().find(|n| **n > 15); // Some(20)
    let missing: Option<&i32> = numbers.iter().find(|n| **n > 99); // None
    println!("found = {found:?}, missing = {missing:?}");

    // `match` on Option:
    match found {
        Some(n) => println!("first big number: {n}"),
        None => println!("no big number"),
    }

    // `if let`: when you only care about ONE variant.
    if let Some(n) = found {
        println!("if let: {n}");
    }

    // Convenient helpers: a default when None.
    println!("unwrap_or: {}", missing.copied().unwrap_or(-1));

    // -- 4. Result<T, E>: "success or an error" ---------------------------
    //     enum Result<T, E> { Ok(T), Err(E) }
    for text in ["42", "forty-two"] {
        match text.parse::<i32>() {
            Ok(n) => println!("'{text}' -> number {n}"),
            Err(e) => println!("'{text}' -> error: {e}"),
        }
    }

    // -- 5. matches! : a yes/no question about a variant -------------------
    println!("is red? {}", matches!(light, Light::Red));

    // -- 6. exhaustiveness -------------------------------------------------
    // Add a fourth variant `Blue` to `Light` and compile: every `match` that
    // forgot it fails with "non-exhaustive patterns". That is the point.
}
