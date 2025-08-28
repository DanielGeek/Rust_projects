#![allow(unused)]

// Trait basic
struct Solidity {
    version: String,
}

struct Vyper {
    version: String,
}

trait Compiler {
    fn compile(&self, file_path: &str) -> String;
    fn help(&self) -> String {
        // Default implementation
        "Good luck".to_string()
    }
}

impl Compiler for Solidity {
    fn compile(&self, file_path: &str) -> String {
        format!("solc (version: {}) {}", self.version, file_path)
    }
}

impl Compiler for Vyper {
    fn compile(&self, file_path: &str) -> String {
        format!("vyper (version: {}) {}", self.version, file_path)
    }
}

// Input lang is any type that implements the Compiler trait
fn compile(lang: &impl Compiler, file_path: &str) -> String {
    lang.compile(file_path)
}

fn main() {
    let sol = Solidity {
        version: "0.8".to_string(),
    };
    let vy = Vyper {
        version: "0.4".to_string(),
    };

    println!("Help! {:?}", sol.help());
    println!("Compile {:?}", sol.compile("Hello.sol"));

    println!("{}", compile(&sol, "Hello.sol"));
    println!("{}", compile(&vy, "Hello.vy"));
}
