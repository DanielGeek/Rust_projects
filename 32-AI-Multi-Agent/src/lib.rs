use std::io::stdin;
pub mod ai;
pub mod config;
pub mod tools;

use bb_ollama::models::{
    chat_request::Chat,
    message::Message,
    options::ChatRequestOptions,
    tool::{Property, Tool},
};
use eyre::{Context, OptionExt, Result};

use config::Config;

use ai::ask_ai_what_tool_to_use;

pub fn run(config: Config) -> Result<()> {
    let options = ChatRequestOptions::new().system("you are a todo application, gree the user when you don't have any history of todos. Just write as if you were talking to a real person who has walked up to the todo counter. Your todo app name is AI todo, and the user is user. Your features include creating, updating, retreiving, and deleting simple tasks.");
    let mut app = Chat::new(config.model, Some(options));
    let initital_message = "hello";

    app.add_message(Message::new_user(initital_message));

    let app_greeting = app.send().context("Sending initial message")?;

    println!("{app_greeting}");

    app.add_message(app_greeting);

    loop {
        let user_input = get_user_input().context("getting user input")?;

        println!("user input: {user_input}");

        let user_message = Message::new_user(user_input);

        app.add_message(user_message);

        app
            .add_tool(Tool::new().function_name("run_sql")
            .function_description("Run a SQL command for a postgres database that has a single table named tasks. The table has the following columns: id: int, name: text.")
            .add_function_property("sql", Property::new_string("The SQL to be run on the database"))
            .add_required_property("sql")
            .build()
            .ok_or_eyre("Adding tool")?);

        let tool_result = app.send().context("getting sql")?;

        println!("result of command: {:?}", tool_result.tool_calls);

        panic!();
    }

    Ok(())
}

fn get_user_input() -> Result<String> {
    let mut user_input = String::new();

    stdin()
        .read_line(&mut user_input)
        .context("Reading user input")?;

    Ok(user_input.trim().to_owned())
}
