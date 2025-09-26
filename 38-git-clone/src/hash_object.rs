use sha1::Sha1;

pub fn hash_object(args: &[String]) {
    match args[0].as_str() {
        "-w" => {
            let filename = &args[1];
            let file = std::fs::read(filename).unwrap();
            let sha = get_sha(&file);
        }
        _ => println!("unknown option: {}", args[0]),
    }
}

fn get_sha(file: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(file);
    let hash = hasher.finalize();
}
