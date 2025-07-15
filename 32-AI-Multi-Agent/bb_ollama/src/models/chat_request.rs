use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::api;

use super::{message::Message, options::ChatRequestOptions};

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
    pub fn new(model: impl Into<String>) -> Self {
        let messages = vec![];
        let stream = Some(false);
        let raw = Some(false);
        let options = None;

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

    pub fn send(&self) -> Result<()> {
        api::send_to_ollama(&self)?;

        Ok(())
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
        let mut chat = Chat::new(model);

        chat.add_message(message);

        chat.send()?;

        Ok(())
    }
}
