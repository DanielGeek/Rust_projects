#![allow(unused)]

fn main() {
    let x: Option<u32> = Some(1);

    match x {
        Some(i) => println!("match {i}"),
        _ => {}
    }

    // if let
    if let Some(i) = x {
        println!("if let {i}");
    }
}
