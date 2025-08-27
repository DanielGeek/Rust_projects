mod math {
    pub fn min(x: u32, y: u32) -> u32 {
        if x <= y { x } else { y }
    }

    pub fn max(x: u32, y: u32) -> u32 {
        if x >= y { x } else { y }
    }
}

mod util {
    pub mod log {
        pub fn debug(s: &str) {
            println!("DEBUG: {s}");
        }
    }

    pub mod vec {
        pub fn first(v: &[u32]) -> Option<u32> {
            let n = v.len();
            if n > 0 { Some(v[0]) } else { None }
        }

        pub fn last(v: &[u32]) -> Option<u32> {
            let n = v.len();
            if n > 0 { Some(v[n - 1]) } else { None }
        }
    }
}

use my_modules::my;
use my_modules::foo;

fn main() {
    // Call the main function from the my module
    my::main();
    
    // Call the print function from the foo module
    foo::print();
    
    // You can also directly access other public functions
    println!("--- Direct calls ---");
    my::print();
    my::a::print();
    println!("Built: {}", my::a::build(42));

    // util::log::debug(&format!("min: {}", math::min(1, 2)));
    // util::log::debug(&format!("max: {}", math::max(1, 2)));
}
