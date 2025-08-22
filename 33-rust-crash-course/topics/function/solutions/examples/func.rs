#![allow(unused)]

fn add_with_return(x: u32, y: u32) -> u32 {
    return x + y;
}

fn add(x: u32, y: u32) -> u32 {
    // Implicitly returns the last statement without a semicolo
    x + y
}

// Function that doesn't return any output
fn print(s: String) {
    println!("{0}{0}{0}{0}{0}", s);
}

fn main() {
    let x: u32 = 1;
    let y: u32 = 2;
    let z: u32 = add(x, y);
    println!("{} + {} = {}", x, y, z);

    print("🐸".to_string());
}
