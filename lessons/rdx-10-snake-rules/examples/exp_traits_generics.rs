//! EXPERIMENT: traits, generics, static vs dynamic dispatch.
//!
//! Run:  cargo run --example exp_traits_generics

use std::fmt::Display;

// ---------------------------------------------------------------------------
// 1. A trait = a promise about behaviour
// ---------------------------------------------------------------------------
trait Animal {
    fn name(&self) -> String; // required
    fn sound(&self) -> String; // required

    // default method: free for implementors, built from the required ones
    fn speak(&self) -> String {
        format!("{} says {}", self.name(), self.sound())
    }
}

struct Dog;
struct Cat {
    lives: u8,
}

impl Animal for Dog {
    fn name(&self) -> String {
        String::from("Dog")
    }
    fn sound(&self) -> String {
        String::from("woof")
    }
}

impl Animal for Cat {
    fn name(&self) -> String {
        format!("Cat({} lives)", self.lives)
    }
    fn sound(&self) -> String {
        String::from("meow")
    }
    // override the default
    fn speak(&self) -> String {
        format!("{} purrs", self.name())
    }
}

// ---------------------------------------------------------------------------
// 2. Generics with a trait bound: STATIC dispatch
// ---------------------------------------------------------------------------
// One source function; the compiler writes a separate copy for Dog and for Cat
// ("monomorphization"). Fast, but each call knows its type at compile time.
fn announce<A: Animal>(animal: &A) {
    println!("2. {}", animal.speak());
}

// The same, written with `impl Trait` (shorter syntax for simple cases).
fn announce_short(animal: &impl Animal) {
    println!("   short form: {}", animal.speak());
}

// Several bounds with `+`, or a `where` clause when the list gets long:
fn show_twice<T>(value: T) -> String
where
    T: Display + Clone,
{
    format!("{value} {}", value.clone())
}

// ---------------------------------------------------------------------------
// 3. Trait objects: DYNAMIC dispatch
// ---------------------------------------------------------------------------
// A Vec holds ONE type. To store a Dog and a Cat together we erase the type:
// `Box<dyn Animal>` = "a heap box holding SOME Animal; find the right method at run time".
// Cost: one extra pointer jump per call (a "vtable" lookup).
fn all_speak(zoo: &[Box<dyn Animal>]) {
    for animal in zoo {
        println!("3. {}", animal.speak());
    }
}

// ---------------------------------------------------------------------------
// 4. A generic STRUCT
// ---------------------------------------------------------------------------
// `Stack<T>` works for any item type. `Vec<T>`, `Option<T>`, `Result<T, E>`
// and `Signal<T>` are all generic types written this way.
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack { items: Vec::new() }
    }
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    fn len(&self) -> usize {
        self.items.len()
    }
}

// Methods that exist ONLY when T has some ability:
impl<T: Display> Stack<T> {
    fn show(&self) -> String {
        let parts: Vec<String> = self.items.iter().map(|item| item.to_string()).collect();
        parts.join(", ")
    }
}

// ---------------------------------------------------------------------------
// 5. Implementing a standard-library trait: Iterator
// ---------------------------------------------------------------------------
struct Countdown(u32);

impl Iterator for Countdown {
    type Item = u32; // an ASSOCIATED TYPE: what this iterator produces

    fn next(&mut self) -> Option<u32> {
        if self.0 == 0 {
            None
        } else {
            self.0 -= 1;
            Some(self.0 + 1)
        }
    }
}

fn main() {
    println!("1. {}", Dog.speak());

    announce(&Dog);
    announce(&Cat { lives: 9 });
    announce_short(&Dog);

    println!("   {}", show_twice("fox"));
    println!("   {}", show_twice(42));

    let zoo: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat { lives: 7 })];
    all_speak(&zoo);

    let mut numbers = Stack::new(); // T inferred as i32 from the push below
    numbers.push(1);
    numbers.push(2);
    let mut words = Stack::new();
    words.push(String::from("a"));
    println!("4. numbers = [{}], words = [{}], len {}", numbers.show(), words.show(), numbers.len());
    println!("   pop = {:?}, pop = {:?}, pop = {:?}", numbers.pop(), numbers.pop(), numbers.pop());

    // Because Countdown implements Iterator, we get map/filter/sum/collect for free.
    let total: u32 = Countdown(4).map(|n| n * 10).sum();
    let list: Vec<u32> = Countdown(3).collect();
    println!("5. total = {total}, list = {list:?}");

    // Try these (each one is a compile error worth reading):
    // announce(&5);                 // the trait bound `i32: Animal` is not satisfied
    // let mixed = vec![Dog, Cat { lives: 1 }];   // a Vec holds one type: use Box<dyn Animal>
}
