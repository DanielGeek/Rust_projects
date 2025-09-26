use codecrafters_git::{
    cat_file::cat_file,
    init::init,
    hash_object::hash_object,
};
use std::env;

fn main() {
    // Uncomment this block to pass the first stage
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "init" => init(),
        "cat-file" => cat_file(&args[2..]),
        "hash-object" => hash_object(&args[2..]),
        _ => println!("unknown command: {}", args[1]),
    }
}
