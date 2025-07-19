use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolCall {
    pub function: ToolCallFunction,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: HashMap<String, String>,
}

impl Clone for ToolCall {
    fn clone(&self) -> Self {
        Self {
            function: self.function.clone(),
        }
    }
}

impl Clone for ToolCallFunction {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            arguments: self.arguments.clone(),
        }
    }
}
