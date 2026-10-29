//! EXPERIMENT: threads, channels, and shared state (native Rust only).
//!
//! Run:  cargo run --example exp_threads_channels
//!
//! In the Snake page the "game loop" is an async task on ONE thread. This lab
//! shows the other classic way to build it with OS threads, and why Rust makes
//! that safe. (On the web this approach is unavailable: a browser tab runs Rust
//! on a single thread. That is why the page uses async + `sleep_ms` instead.)

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // -- 1. spawn a thread and wait for it ------------------------------------------
    // `move` gives the thread its own copy/ownership of what it uses.
    let handle = thread::spawn(move || {
        6 * 7 // the closure's return value comes back through `join`
    });
    // `join()` waits for the thread and returns Result: Err if the thread panicked.
    let result = handle.join().expect("thread panicked");
    println!("1. thread returned {result}");

    // -- 2. a channel: send values between threads -------------------------------------
    // `mpsc` = multiple producers, single consumer. `tx` sends, `rx` receives.
    // The VALUE IS MOVED through the channel: after `send`, the sender no longer owns it.
    let (tx, rx) = mpsc::channel::<u32>();

    // A "ticker" thread: sends 1, 2, 3, 4, 5 with a pause, like a game timer.
    let ticker = thread::spawn(move || {
        for tick in 1..=5 {
            thread::sleep(Duration::from_millis(30)); // OK on a thread of our own
            if tx.send(tick).is_err() {
                break; // the receiver is gone: stop
            }
        }
        // `tx` is dropped here: the channel closes, so the receiver's loop ends
    });

    // The "game" side: wait for ticks. `for tick in rx` ends when all senders are dropped.
    let mut total = 0;
    for tick in rx {
        println!("2. tick {tick}");
        total += tick;
    }
    ticker.join().unwrap();
    println!("   total = {total}");

    // -- 3. shared state: Arc<Mutex<T>> -----------------------------------------------------
    // Two threads want to change the SAME counter.
    //   Arc   = atomically reference counted pointer: many owners of one value
    //   Mutex = only one thread at a time may hold the lock and touch the data
    // Rust refuses to compile shared mutation WITHOUT these (a "data race" is impossible).
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for _ in 0..4 {
        let counter = Arc::clone(&counter); // a new owner (cheap: bumps a counter)
        handles.push(thread::spawn(move || {
            for _ in 0..1_000 {
                // `lock()` returns a guard (like a signal's write guard!). The lock is
                // released when the guard is dropped, at the end of the statement.
                *counter.lock().unwrap() += 1;
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("3. counter = {} (4 threads x 1000, never lost an update)", *counter.lock().unwrap());
    println!("   Arc strong count now: {}", Arc::strong_count(&counter));

    // What the compiler stops you from doing (try them):
    //
    // let mut plain = 0;
    // thread::spawn(|| plain += 1);          // ERROR: closure may outlive `plain` (needs `move`)
    //
    // let text = String::from("hi");
    // thread::spawn(move || println!("{text}"));
    // println!("{text}");                    // ERROR: `text` was moved into the thread
    //
    // let rc = std::rc::Rc::new(5);
    // thread::spawn(move || println!("{rc}")); // ERROR: `Rc` cannot be sent between threads
    //                                          // (use `Arc`: it counts atomically)
}
