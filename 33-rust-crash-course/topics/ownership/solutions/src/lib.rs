pub fn exercise_1() {
    let s = "rust".to_string();
    let s1 = s.clone();
    let s2 = s;
    println!("{s1}");
}

pub fn exercise_2() {
    let s = "rust".to_string();
    {
        let s1 = s.clone();
        println!("{s1}");
    }
    println!("{s}");
}

fn take(s: String) {
    println!("take {s}");
}

pub fn exercise_3() {
    let s = "rust".to_string();
    take(s.clone());
    println!("{s}");
    println!("{s}");
}
