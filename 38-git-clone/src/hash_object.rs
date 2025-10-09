use std::{fs::DirBuilder, io::Write};

use flate2::{Compression, write::ZlibEncoder};
use hex::ToHex;
use sha1::{Digest, Sha1};

pub fn hash_object(args: &[String]) {
    match args[0].as_str() {
        "-w" => {
            let filename = &args[1];
            let file = std::fs::read(filename).unwrap();
            let sha = get_sha(&file);

            create_folder(&sha);
            compress(&file);
            print_sha(&sha);
        }
        _ => println!("unknown option: {}", args[0]),
    }
}

fn get_sha(file: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(file);
    hasher.finalize().encode_hex::<String>()
}

fn compress(file: &[u8]) {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(file).unwrap();
}

fn create_folder(sha: &str) {
    // normally we should find git folder in case we are nested in. Let's yolo because why not? :)
    let path = format!(".git/objects/{}", &sha[0..2]);
    // we need to handle the case that the folder already exists. If that is so, we don't want to crash.
    DirBuilder::new().recursive(true).create(path).unwrap();
}

fn print_sha(sha: &str) {
    println!("{sha}");
}
