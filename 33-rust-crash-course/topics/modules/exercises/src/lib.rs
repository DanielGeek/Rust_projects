#![allow(unused)]

pub mod foo {
    pub fn print() {
        println!("foo");
    }
}

// This is the file
// - foo
// - my
// - a

pub mod my {
    use super::foo;

    pub mod a {
        pub fn print() {
            println!("a::print");
        }

        pub fn build(_i: i32) -> String {
            "built".to_string()
        }

        pub fn print_foo() {
            super::super::foo::print();
        }
    }

    pub fn print() {
        println!("rust");
    }

    fn private_print() {
        a::print();
    }

    pub fn main() {
        print();
        a::print();
        
        let s = a::build(1);
        println!("Built: {}", s);

        a::print_foo();
    }
}