pub fn first<A, B>(t: (A, B)) -> A {
    t.0
}

pub fn last<A, B>(t: (A, B)) -> B {
    t.1
}

#[derive(Debug)]
pub struct Rectangle {
    pub top: u32,
    pub left: u32,
    pub width: u32,
    pub height: u32,
}
