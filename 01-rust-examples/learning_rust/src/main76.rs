
// Passes Mutexes between Threads

// use std::{sync::{Arc, Barrier}, thread};

// fn main() {
//     let mut threads = Vec::new();
//     let barrier = Arc::new(Barrier::new(5));

//     for i in 0..3 {
//         let barrier = barrier.clone();
//         let t = thread::spawn(move || {
//             println!("Before wait {}", i);
//             barrier.wait();
//             println!("After wait {}", i);
//         });
//         threads.push(t);
//     }

//     for t in threads {
//         t.join().unwrap();
//     }
// }

use std::sync::{Arc, Mutex, Barrier};
use std::thread;

fn main() {
    let mut threads = Vec::new();
    let barrier = Arc::new(Barrier::new(3));
    let data = Arc::new(vec![
        vec![1,2,3,4,5,6],
        vec![1,2,3,4,5,6],
        vec![1,2,3,4,5,6],
    ]);

    let result = Arc::new(Mutex::new(0));
    for i in 0..3 {
        let barrier = barrier.clone();
        let data = data.clone();
        let result = result.clone();
        let t = thread::spawn(move || {

            let mut x = result.lock().unwrap();
            *x = data[i][0..3].iter().sum();
            // let x: i32 = data[i][0..3].iter().sum();
            // *result.lock().unwrap() += x;
            println!("Thread {} Part 1 is done", i);
            barrier.wait();

            // let x:i32 = data[i][3..6].iter().sum();
            // *result.lock().unwrap() += x;
            *x = data[i][3..6].iter().sum();
            println!("Thread {} is complete", i);
        });
        threads.push(t);
    }
    for t in threads {
        t.join().unwrap();
    }
    println!("The final value of the result is {}", *result.lock().unwrap());
}