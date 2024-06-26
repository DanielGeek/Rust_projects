// 16. Functions and Code Blocks

fn main() {
    my_fn("This is my function!");
    let str = "Function call with a variable";
    my_fn(str);

    let answer = multiplication(10, 15);

    let result = basic_math(10, 15);
    let (multiplication, addition, substraction) = basic_math(10, 15);

    let full_name = {
        let first_name = "Daniel";
        let last_name = "Angel";
        format!("{first_name} {last_name}")
    };
}

fn my_fn(s: &str) {
    println!("{s}");
}

fn multiplication(num1: i32, num2: i32) -> i32 {
    println!("Cumputing multiplication");
    num1 * num2
}

fn basic_math(num1: i32, num2: i32) -> (i32, i32, i32) {
    (num1 * num2, num1 + num2, num1 - num2)
}
