#![allow(unused_attributes)]
pub mod ai;
pub mod commands;
pub mod config;
pub mod logger;
pub mod state;
pub mod tool_property;
pub mod tools;
use std::collections::HashMap;

use ai::create_assistant_chat;
use bb_ollama::models::{chat_request::Chat, message::Message};
use commands::Command;
use db::{connect, insert, Client};
use eyre::{Context, Result};
use logger::{loggit, LogLevel};
use tool_property::ToolProperty;

pub fn run() -> Result<Message> {
    // setup
    let mut personal_assistant = create_assistant_chat();
    let mut db_client = connect().context("connecting to the database")?;

    personal_assistant.add_message(Message::new_system(
        "You are an AI Todo Application. You can CRUD (Create, Read, Update, and Delete) tasks in the database. You are super professional while replying to the user.",
    ));
    personal_assistant.add_message(Message::new_user("User has logged into the system, feel free to ask what their name is then introduce them to yourself and your features."));

    // update
    loop {
        let response = match personal_assistant.send() {
            Ok(message) => message,
            Err(error) => {
                loggit(
                    format!(
                        "There was an error sending a message to the personal assistant: {error:?}"
                    ),
                    LogLevel::Error,
                );
                personal_assistant.add_message(Message::new_user(format!("SYSTEM: Apparently there was an error sending a message to you, let's try whatever you were doing / going to say again")));
                continue;
            }
        };
        let (command, arguments) = Command::from_message(&response);

        match command {
            Command::Chat => {
                handle_chat(arguments);
                get_user_input(&mut personal_assistant);
            }
            Command::InsertTaskIntoDb => {
                handle_insert_task(&mut personal_assistant, arguments, &mut db_client)
                    .context("inserting task into db")?;
            }
            Command::GetAllTasksFromDb => todo!(),
            Command::GetTaskByIdFromDb => todo!(),
            Command::UpdateTaskInDb => todo!(),
            Command::DeleteTaskInDb => todo!(),
            Command::EraseDb => todo!(),
            Command::Quit => {
                personal_assistant.add_message(Message::new_tool(
                    "Quitting app, you can leave a final message for the user now.",
                ));
                break;
            }
        }
    }
    // teardown

    personal_assistant.send().context("Sending last message")
}

fn handle_insert_task(
    personal_assistant: &mut Chat,
    arguments: HashMap<String, String>,
    db_client: &mut Client,
) -> Result<()> {
    loggit("AI running insert task into db", logger::LogLevel::Info);

    let Some(value) = arguments.get(ToolProperty::Name.to_string().as_str()) else {
        loggit(
            "could not find task name in arguments",
            logger::LogLevel::Error,
        );
        personal_assistant.add_message(Message::new_tool("Error, name was not passed into the tool correctly, please try again, but this time pass in the new task name"));
        return Ok(());
    };

    let new_task = insert(db_client, &value).context("inserting the task into the database")?;

    loggit(
        format!("task inserted into the database :{new_task}"),
        LogLevel::Debug,
    );

    let message = format!("The task was created in the database successfully! Here is the full task that was created: {new_task}");

    personal_assistant.add_message(Message::new_tool(message));

    Ok(())
}

fn handle_chat(arguments: HashMap<String, String>) {
    loggit("AI chatting", LogLevel::Info);

    for value in arguments.values() {
        loggit(value, LogLevel::Normal);
    }
}

fn get_user_input(personal_assistant: &mut Chat) {
    let mut user_input = String::new();
    if let Err(error) = std::io::stdin()
        .read_line(&mut user_input)
        .context("getting user input")
    {
        loggit(format!("{error:?}"), LogLevel::Error);
        personal_assistant.add_message(Message::new_tool(format!(
            "There was an error getting input from your user: {error:?}"
        )));
        return;
    }

    personal_assistant.add_message(Message::new_tool(format!("The user said: {user_input}. To answer the question use one of the tools to find to appropriate information before responding.")));
}
