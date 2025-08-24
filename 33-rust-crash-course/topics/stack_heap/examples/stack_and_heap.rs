#![allow(unused)]

// Memory
// Stack
// - Stores data of fixed size known at compile time
// - Fast
// - LIFO
// Heap
// - Stores data of unknown size at compile time
// - Slower than stack
// - Memory safety is enforced through Rust's ownership and borrowing rules

fn main() {
    // Stack examples - data sizes are known at compilation
    let x: i32 = 42;
    let y: [i32; 3] = [1, 2, 3];

    // Heap examples - data size can change during run time
    let s: String = "hello".to_string();
    let v: Vec<i32> = vec![1, 2, 3];

    // Explicitly allocate i32 into heap
    let boxed: Box<i32> = Box::new(99);
    println!("boxed {:?}", boxed);
}
