use eyre::{Context, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::models::{chat_request::Chat, message::Message};

const OLLAMA_CHAT_URL: &str = "http://localhost:11434/api/chat";

pub fn send_to_ollama(chat: &Chat) -> Result<Message> {
    let client = Client::new();
    let request = client
        .post(OLLAMA_CHAT_URL)
        .json(chat)
        .send()
        .context("Sending chat to Ollama")?;

    let chat_response = request
        .json::<ChatResponse>()
        .context("Converting response from Ollama to a Chat")?;

    Ok(chat_response.message)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    pub model: String,
    pub message: Message,
}
