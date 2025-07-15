// use serde::{Deserialize, Serialize};

// #[derive(Debug)]
// pub struct Chat {}

// #[derive(Debug, Default, Serialize, Deserialize)]
// pub struct ChatBuilder<ARGUMENTS> {
//     model: Option<String>,
//     messages: Vec<Message<ARGUMENTS>>,
//     tools: Vec<Tool>,
// }

// impl ChatBuilder {
//     pub fn new() -> Self {
//         Self::default()
//     }

//     pub fn set_model(mut self, name: &str) -> Self {
//         self.model = Some(name.to_owned());

//         self
//     }

//     pub fn add_message<ARGUMENTS>(mut self, message: Message<ARGUMENTS>) {
//         self.messages.push(message);
//     }
// }

// #[derive(Debug, Default, Serialize, Deserialize)]
// pub struct Message<ARGUMENTS> {
//     pub role: Role,
//     pub content: String,
//     pub tool_calls: Option<Vec<ToolCall<ARGUMENTS>>>,
// }

// #[derive(Debug, Default, Serialize, Deserialize)]
// pub enum Role {
//     System,
//     #[default]
//     User,
//     Assistant,
//     Tool,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Tool {
//     #[serde(rename = "type")]
//     pub tool_type: String,
//     pub function: ToolFunction,
// }

// impl Default for Tool {
//     fn default() -> Self {
//         Self {
//             tool_type: "function".to_owned(),
//         }
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ToolFunction {
//     pub name: String,
//     pub description: String,
//     pub parameters: ToolFunctionParameter,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ToolFunctionParameter<PROPERTIES> {
//     #[serde(rename = "type")]
//     pub parameter_type: String,
//     pub properties: PROPERTIES,
// }

// impl<PROPERTIES> Default for ToolFunctionParameter<PROPERTIES> {
//     fn default() -> Self {
//         Self {
//             parameter_type: "object".to_owned(),
//             properties: PROPERTIES::default(),
//         }
//     }
// }

// #[derive(Debug, Serialize, Deserialize, Default)]
// struct DefaultProperty {
//     pub a: String,
// }

// #[derive(Debug, Default, Serialize, Deserialize)]
// pub struct ToolCall<ARGUMENTS> {
//     pub function: ToolCallFunction<ARGUMENTS>,
// }

// #[derive(Debug, Default, Serialize, Deserialize)]
// pub struct ToolCallFunction<ARGUMENTS> {
//     pub name: String,
//     pub arguments: ARGUMENTS,
// }

// #[derive(Debug, Default, Serialize, Deserialize)]
// pub struct ToolFunctionArguments {}

// mod tests {
//     use super::*;

//     #[test]
//     fn can_create_chat_using_builder() {
//         let chat = ChatBuilder::new().set_model("llama3.2:1b-instruct-fp16");
//     }
// }
