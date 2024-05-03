// 20. Comments, Prints, Inputs, Variable Convention, Statics

fn main() {
    // The current line is a comment line
    // This is the second line of comment

    /* This is a
    multiple line
    comment
    */

    print!("This is a print command");
    print!("This is going to be printed on the same line");

    /*
    \n : Newline character.
    \t : Tab space.
    \r : Carriage Return.
    \" : Double quote.
    \\ : Backward slash.
     */

    println!("\n Will be printed after one empty line");
    println!("\t A tab space at the start");
    println!("This will be overwritten \r This text will only appear on the screen");
    
    println!("Prints double quotes \", Prints backslash \\");

    println!(
        "I am doing {2} from {1} years and i {0} it",
        "like", 12, "programming"
    );

    println!(
        "{language} is a system programming language which is cool to {activity} in.",
        activity = "code",
        language = "Rust"
    );

    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");

    let n: f64 = n.trim().parse().expect("invalid input");
    println!("{n}");

    let _number_one = 45.0;
    let x = 40_000;

    static WELCOME: &str = "Welcome to Rust";
    const PI: f32 = 3.14;

    let a = PI;
    let b = PI;

    let c = WELCOME;
    let d = WELCOME;
}
