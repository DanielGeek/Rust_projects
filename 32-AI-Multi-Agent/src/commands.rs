use bb_ollama::models::message::Message;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    InsertTaskIntoDb,
    GetAllTasksFromDb,
    GetTaskByIdFromDb,
    UpdateTaskInDb,
    DeleteTaskInDb,
    EraseDb,
    Chat,
    Quit,
}

impl Command {
    pub fn from_message(message: &Message) -> (Self, HashMap<String, String>) {
        if message.content.is_empty() {
            let Some(tool_calls) = &message.tool_calls else {
                return (
                    Self::Chat,
                    HashMap::from([(
                        "error".to_string(),
                        "I'm sorry, but I didn't understand what you said, please try again".to_string(),
                    )]),
                );
            };
            let Some(tool_call) = tool_calls.first() else {
                return (
                    Self::Chat,
                    HashMap::from([(
                        "error".to_string(),
                        "I'm sorry, but I didn't understand what you said, please try again".to_string(),
                    )]),
                );
            };
            let function = &tool_call.function;
            let name = &function.name;
            let command = Self::from(name.as_str());
            let arguments = function.arguments.clone();

            (command, arguments)
        } else {
            (
                Self::Chat,
                HashMap::from([("message".to_string(), message.content.clone())]),
            )
        }
    }
}

impl Into<String> for Command {
    fn into(self) -> String {
        self.to_string()
    }
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "insert_task_into_db" => Self::InsertTaskIntoDb,
            "get_all_tasks_from_db" => Self::GetAllTasksFromDb,
            "get_task_by_id_from_db" => Self::GetTaskByIdFromDb,
            "update_task_in_db" => Self::UpdateTaskInDb,
            "delete_task_in_db" => Self::DeleteTaskInDb,
            "erase_db" => Self::EraseDb,
            "quit" => Self::Quit,
            _ => Self::Chat,
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let command = match self {
            Self::InsertTaskIntoDb => "insert_task_into_db",
            Self::Chat => "chat",
            Command::GetAllTasksFromDb => "get_all_tasks_from_db",
            Command::GetTaskByIdFromDb => "get_task_by_id_from_db",
            Command::UpdateTaskInDb => "update_task_in_db",
            Command::DeleteTaskInDb => "delete_task_in_db",
            Command::EraseDb => "erase_db",
            Command::Quit => "quit",
        };

        write!(f, "{command}")
    }
}
