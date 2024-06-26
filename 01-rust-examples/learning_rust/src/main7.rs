// 19. Control flow, Loops, For loops and while loops

fn main() {
    'outer: loop {
        println!("Simple loop");
        break 'outer;
    }

    let a = loop {
        break 5;
    };

    let vec = vec![45, 30, 85, 90, 41, 39];

    for i in vec {
        println!("{i}");
    }

    let mut num = 0;
    while num < 10 {
        num = num + 1;
    }
}
