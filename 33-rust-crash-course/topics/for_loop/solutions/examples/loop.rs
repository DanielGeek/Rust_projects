#![allow(unused)]

fn main() {
    // Loop
    let mut i: u32 = 0;
    loop {
        println!("loop {}", i);
        i += 1;

        if i > 5 {
            break;
        }
    }

    // While loop
    let mut i: u32 = 0;
    while i <= 5 {
        println!("while loop {}", i);
        i += 1;
    }

    // For loop
    for i in 0..5 {
        println!("for loop {}", i);
    }

    // Loop array
    let xs = [1, 2, 3];

    // usize
    let n: usize = xs.len();
    for i in 0..n {
        // This will not compile
        // i is usize
        // let k = i + 1u32;
        println!("{}", xs[i]);
    }

    // Loop using iterator
    let v = vec![1, 2, 3];

    // iter - can loop vector multiple times
    for x in v.iter() {
        println!("for loop iter {}", x);
    }

    for x in v.iter() {
        //
    }

    // into_iter - can only loop once
    for x in v {
        //
    }

    /*
    for x in v {
        //
    }
    */

    // Return value from loop
    let mut i: u32 = 0;
    let v = loop {
        i += 1;
        if i > 3 {
            break "i > 3";
        }
    };
    println!("return value from loop {}", v);
}
