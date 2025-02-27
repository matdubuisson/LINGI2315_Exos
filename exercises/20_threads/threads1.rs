// This program spawns multiple threads that each runs for at least 250ms, and
// each thread returns how much time it took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.

use std::{
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

fn main() {
    let mut handles = Vec::new();
    for i in 0..10 {
        let handle: JoinHandle<u128> = thread::spawn(move || {
            let start = Instant::now();

            if i % 2 == 0 {
                thread::sleep(Duration::from_millis(300 + i * 10));
            } else {
                thread::sleep(Duration::from_millis(250 + 100 - i)); 
            }
            
            println!("Thread {i} done");
            start.elapsed().as_millis()
        });
        handles.push(handle);
    }

    let mut results: Vec<u128> = Vec::new();
    for handle in handles {
        // TODO: C§ollect the results of all threads into the `results` vector.
        // Use the `JoinHandle` struct which is returned by `thread::spawn`.
        let value: u128 = handle.join().unwrap();
        results.push(value);

    }

    if results.len() != 10 {
        panic!("Oh no! Some thread isn't done yet!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("Thread {i} took {result}ms");
    }
}
