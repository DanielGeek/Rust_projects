#![allow(unused)]
use std::cmp::PartialOrd;

// Enum
#[derive(Debug)]
enum MyOption<T> {
    Some(T),
    None,
}

// Enum - generic over 2 types
#[derive(Debug)]
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

// Struct - default type is u32
#[derive(Debug)]
struct Point<T = u32> {
    x: T,
    y: T,
}

// Multiple generic types
fn swap<A, B>(t: (A, B)) -> (B, A) {
    (t.1, t.0)
}

// Trait bound
fn max<T: PartialOrd>(s: &[T]) -> Option<&T> {
    if s.len() == 0 {
        return None;
    }

    let mut largest = &s[0];

    for item in s {
        if item > largest {
            largest = item;
        }
    }

    Some(largest)
}

fn main() {
    // Vector
    let v: Vec<u32> = vec![1, 2, 3];
    let v: Vec<i32> = vec![-1, 2, -3];
    // Let Rust infer the type
    let v: Vec<_> = vec![-1, 0, 1];

    let p0: Point<u32> = Point { x: 1, y: 0 };
    let p1: Point<f32> = Point { x: 1.23, y: 0.123 };

    // Max i32
    let nums = vec![33, 1, 22, 54, 25, 99, 10];
    let largest = max(&nums);
    println!("largest num: {:?}", largest);

    // Max char
    let chars = vec!['a', 'c', 'y', 'i', 'm'];
    let largest = max(&chars);
    println!("largest char: {:?}", largest);

    // Swap
    let t: (i32, bool) = (1, true);
    let s = swap(t);
    println!("swapped: {:?}", s);
}
