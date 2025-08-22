#![allow(unused)]

// Constants are immutable values known at compile time.
const NUM: u32 = 1;

fn main() {
    // Variable are immutable by default
    let x: i32 = -1;
    // This will not compile
    // x += 1;

    // Mutable variable
    let mut y: i32 = 1;
    y += 1;

    // Type inference: the compiler can infer types in most cases.
    let x = -1;

    // Shadowing: you can redeclare a variable with the same name.
    let x: i32 = -1;
    let x = x + 1;
    let x = true;

    // Type placeholder - compiler will infer the type
    let x: _ = true;

    // Printing values with `println!` macro:
    let x = 1;
    println!("x: {}", x);
    // Inline
    println!("x: {x}");
    // Positional
    println!("{0} + {0} = {1}", x, x + x);
    // Debug - useful for complex types
    println!("x: {:?}", x);
    // Debug with easy to read line breaks
    println!("x: {:#?}", x);
}
