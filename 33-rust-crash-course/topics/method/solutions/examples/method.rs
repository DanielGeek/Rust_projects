#![allow(unused)]

// Struct and enum methods
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    // Associated functions - static methods
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    // Methods
    fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

fn main() {
    let mut p = Point::new(1.0, 1.0);
    p.move_to(2.0, 0.0);
    println!("{:?}", p);
}
