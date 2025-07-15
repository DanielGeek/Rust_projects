use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequestOptions {
    pub system: Option<String>,
    pub seed: Option<u32>,
}
