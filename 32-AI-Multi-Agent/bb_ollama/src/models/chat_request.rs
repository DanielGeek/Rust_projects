use eyre::{OptionExt, Result};
use serde::{Deserialize, Serialize};

use crate::{api, models::tool::Property};

use super::{message::Message, options::ChatRequestOptions, tool::Tool};

#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: Option<bool>,
    pub raw: Option<bool>,
    pub tools: Vec<Tool>,
    pub options: Option<ChatRequestOptions>,
}

impl Chat {
    pub fn new(model: impl Into<String>, options: Option<ChatRequestOptions>) -> Self {
        let messages = vec![];
        let stream = Some(false);
        let raw = Some(false);
        let tools = vec![];

        Self {
            model: model.into(),
            messages,
            stream,
            raw,
            tools,
            options,
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn add_tool(&mut self, tool: Tool) {
        self.tools.push(tool);
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
            tool_calls: None,
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
    let mut chat = Chat::new(model, Some(options));
    let weather_tool_name = "check_weather";
    let weather_tool = Tool::new()
        .function_name(weather_tool_name)
        .function_description("Get the weather in farhenheit degrees for any given location")
        .add_function_property(
            "location",
            Property::new_string(
                "location for where you want to check the weather. Use the format 'city, state' if the location is in the United States. Otherwise use 'city, country'",
            ),
        )
        .add_required_property("location")
        .build()
        .ok_or_eyre("Couldn't build the tool")?;

    chat.add_tool(weather_tool);
    chat.add_message(message);
    let received_message = chat.send()?;

    assert!(received_message.tool_calls.is_some());
    assert_eq!(received_message.content, "");

    let tool_call = received_message.tool_calls.unwrap();

    assert_eq!(tool_call.len(), 1);

    let tool_call = &tool_call[0];

    assert_eq!(tool_call.function.name, weather_tool_name);

    let tool_call_location = tool_call.function.arguments.get("location");

    assert!(tool_call_location.is_some());

    assert_eq!(tool_call_location.unwrap(), "New York, NY");

    Ok(())
}
