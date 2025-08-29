#![allow(unused)]

use std::collections::HashMap;

fn main() {
    // map
    let vals: Vec<u32> = vec![1, 2, 3, 4, 5];
    let v: Vec<u32> = vals
        .iter()
        .map(|x| {
            // use { } for multi line
            x * 2
        })
        .collect();

    println!("map: {:?}", v);

    // collect
    let vals: Vec<(&str, u32)> = vec![("a", 1), ("b", 2), ("c", 3)];
    // Only need to change the type to collect iterator items into different data structures.
    let v: Vec<(String, u32)> = vals.iter().map(|v| (v.0.to_string(), v.1 + 10)).collect();
    let v: HashMap<String, u32> = vals.iter().map(|v| (v.0.to_string(), v.1 + 10)).collect();

    println!("collect: {:?}", v);

    // Chaining map and filter
    let vals: Vec<u32> = vec![1, 2, 3, 4, 5];
    let v: Vec<u32> = vals
        .into_iter()
        .filter(|x| *x <= 3)
        .map(|x| x * 2)
        .collect();

    println!("filter and then map: {:?}", v);
}
