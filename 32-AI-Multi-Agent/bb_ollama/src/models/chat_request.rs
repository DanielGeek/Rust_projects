use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::api;

use super::{message::Message, options::ChatRequestOptions, tool::Tool};

#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: Option<bool>,
    pub raw: Option<bool>,
    // pub tools: Option<Vec>,
    pub options: Option<ChatRequestOptions>,
}

impl Chat {
    pub fn new(model: impl Into<String>, options: Option<ChatRequestOptions>) -> Self {
        let messages = vec![];
        let stream = Some(false);
        let raw = Some(false);

        Self {
            model: model.into(),
            messages,
            stream,
            raw,
            options,
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn send(&self) -> Result<Message> {
        api::send_to_ollama(&self)
    }
}

mod tests {
    use super::*;

    #[test]
    fn can_send_and_receive_chat() -> Result<()> {
        let message = Message {
            role: crate::models::message::Role::User,
            content: "Hello".to_owned(),
        };

        let model = "llama3.1:8b-instruct-fp16";
        let options = ChatRequestOptions {
            system: None,
            seed: Some(123),
        };
        let mut chat = Chat::new(model, Some(options));

        chat.add_message(message);

        let message_response = chat.send()?;

        assert_eq!(message_response.content, "How can I assist you today?");

        Ok(())
    }
}

#[test]
fn can_get_a_tool_call_from_ollama() -> Result<()> {
    let message = Message::new_user("What is the weather like in New York?");
    let model = "llama3.1:8b-instruct-fp16";
    let options = ChatRequestOptions::new().seed(123);
    let chat = Chat::new(model, Some(options));
    let tool = Tool {
        tool_type: "function".to_owned(),
    }
    Ok(())
}
