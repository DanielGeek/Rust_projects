// 77. Combinators

fn main() {
    let words = vec!["apple", "banana", "grape", "orange", "pear"];
    // let mut result: Vec<String> = vec![];

    // for word in words {
    //     if word.starts_with("a") || word.starts_with("b") {
    //         let uppercase_word = word.to_uppercase();
    //         result.push(uppercase_word);
    //     }
    // }
    // println!("Result: {:?}", result);

    let result: Vec<String> = words
        .into_iter()
        .filter(|&word |word.starts_with("a") || word.starts_with("b"))
        .map(|word | word.to_uppercase())
        .collect();
    println!("Result: {:?}", result);
}

