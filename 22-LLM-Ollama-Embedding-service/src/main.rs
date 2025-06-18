use crate::model::OllamaRequest;
use crate::ollama_client::OllamaClient;
use log::{error, info};
mod circuit_breaker;
mod model;
mod ollama_client;
mod retry_util;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let client = OllamaClient::default();
    let request = OllamaRequest {
        model: String::from("llama3.2:1b"),
        prompt: String::from("Why is the sky blue?"),
        stream: Some(false),
    };

    match client.generate(request).await {
        Ok(response) => {
            info!("Got Response: {:?}", response);
        }
        Err(ex) => {
            error!("Error: {:?}", ex);
        }
    }
}
