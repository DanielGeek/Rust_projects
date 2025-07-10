use std::io::stdin;

use eyre::{Context, Result};

pub fn run() -> Result<()> {
    println!("Welcome to AI Todo agent!");

    loop {
        println!("What is your command?");

        let user_input = get_user_input()?;

        println!("You said: '{user_input}'");
    }
}

fn get_user_input() -> Result<String> {
    let mut user_input = String::new();

    stdin()
        .read_line(&mut user_input)
        .context("Reading user input")?;

    Ok(user_input.trim().to_owned())
}
