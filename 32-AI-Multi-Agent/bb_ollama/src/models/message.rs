use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum Role {
    System,
    #[default]
    User,
    Assistant,
    Tool,
}
