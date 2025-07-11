use std::io::stdin;
pub mod ai;
pub mod config;
pub mod tools;
use eyre::{Context, Result};

use config::Config;

use ai::ask_ai_what_tool_to_use;

pub fn run(config: Config) -> Result<()> {
    println!("Welcome to AI Todo agent!");

    loop {
        println!("What is your command?");

        let user_input = get_user_input()?;
        let what_to_do = ask_ai_what_tool_to_use(&user_input, &config)?;

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
