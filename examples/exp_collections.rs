//! EXPERIMENT: collections and iterators (Vec, VecDeque, HashMap, HashSet).
//!
//! Run:  cargo run --example exp_collections
//!
//! Which collection? Ask what you do MOST often:
//!   Vec<T>        add/remove at the END, index by position       (the default choice)
//!   VecDeque<T>   add/remove at BOTH ends                         (Snake's body)
//!   HashMap<K,V>  look up a value by key
//!   HashSet<T>    "have I seen this?"

use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    // -- 1. Vec ------------------------------------------------------------------
    let mut v = vec![10, 20, 30];
    v.push(40);
    println!("1. {v:?} len={} first={:?} last={:?}", v.len(), v.first(), v.last());
    println!("   v.get(9) = {:?} (no panic)", v.get(9));
    v.remove(0); // removes the FIRST element: every other element shifts left (slow for big vecs)
    println!("   after remove(0): {v:?}");

    // -- 2. VecDeque: Snake's body -------------------------------------------------
    let mut snake: VecDeque<(i32, i32)> = VecDeque::new();
    snake.push_back((5, 5)); // tail
    snake.push_front((6, 5)); // new head
    snake.push_front((7, 5)); // another head
    println!("2. snake = {snake:?}");
    let tail = snake.pop_back(); // the snake moves: drop the tail
    snake.push_front((8, 5)); // ...and add a new head
    println!("   dropped {tail:?}, now {snake:?}, head = {:?}", snake.front());
    println!("   contains (7,5)? {}", snake.contains(&(7, 5)));

    // -- 3. HashMap: counting ---------------------------------------------------------
    let text = "fox owl fox hare fox owl";
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        // `entry` finds the slot or creates it; `or_insert(0)` gives a &mut to the value.
        *counts.entry(word).or_insert(0) += 1;
    }
    let mut sorted: Vec<(&&str, &u32)> = counts.iter().collect();
    sorted.sort(); // HashMap order is random: sort before printing
    println!("3. counts = {sorted:?}");
    println!("   fox -> {:?}, bear -> {:?}", counts.get("fox"), counts.get("bear"));

    // -- 4. HashSet: uniqueness ---------------------------------------------------------
    let seen: HashSet<i32> = [3, 1, 3, 2, 1].into_iter().collect();
    let mut unique: Vec<_> = seen.iter().copied().collect();
    unique.sort();
    println!("4. unique = {unique:?}, contains 2? {}", seen.contains(&2));

    // -- 5. iterator pipelines -------------------------------------------------------------
    let numbers = [1, 2, 3, 4, 5, 6];
    let even_squares: Vec<i32> = numbers
        .iter() // yields &i32
        .filter(|n| *n % 2 == 0) // keep even
        .map(|n| n * n) // square
        .collect(); // run the pipeline and build a Vec
    println!("5. even squares = {even_squares:?}");

    let total: i32 = numbers.iter().sum();
    let biggest = numbers.iter().max(); // Option<&i32>
    let first_big = numbers.iter().find(|n| **n > 3); // Option<&i32>
    let any_neg = numbers.iter().any(|n| *n < 0);
    let all_pos = numbers.iter().all(|n| *n > 0);
    println!("   sum={total} max={biggest:?} first>3={first_big:?} any_neg={any_neg} all_pos={all_pos}");

    // `take(n)` and `enumerate`, both used in Snake:
    for (i, n) in numbers.iter().take(3).enumerate() {
        println!("   #{i} -> {n}");
    }

    // Iterators are LAZY: nothing runs until a consumer asks.
    let lazy = numbers.iter().map(|n| {
        println!("   (computing {n})");
        n * 2
    });
    println!("   created the iterator, nothing printed yet");
    let first_two: Vec<i32> = lazy.take(2).collect(); // only 2 elements are computed
    println!("   first_two = {first_two:?}");

    // -- 6. ownership and iterators ------------------------------------------------------------
    let names = vec![String::from("Fox"), String::from("Owl")];
    for name in &names {
        // borrows each element
        println!("6. borrowed {name}");
    }
    println!("   names still usable: {names:?}");
    for name in names {
        // moves every element out: `names` is gone afterwards
        println!("   owned {name}");
    }
    // println!("{names:?}");   // ERROR: borrow of moved value: `names`
}
