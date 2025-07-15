use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tool {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: Function,
}

impl Tool {
    pub fn new() -> ToolBuilder {
        ToolBuilder::new()
    }
}

#[derive(Default)]
pub struct ToolBuilder {
    pub tool_type: String,
}

impl ToolBuilder {
    pub fn new() -> Self {
        let tool_type = "function".to_owned();

        Self { tool_type }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub description: String,
    pub parameters: Parameter,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    #[serde(rename = "type")]
    pub parameter_type: String,
    pub properties: HashMap<String, Property>,
    pub required: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
    #[serde(rename = "type")]
    pub property_type: String,
    pub description: String,
}
