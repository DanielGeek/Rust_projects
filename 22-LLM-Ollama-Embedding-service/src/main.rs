use std::time::Duration;

use crate::model::OllamaRequest;
use crate::ollama_client::OllamaClient;
use log::{error, info};
use tokio::time::sleep;
mod circuit_breaker;
mod model;
mod ollama_client;
mod retry_util;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let mut client = OllamaClient::new();

    loop {
        let request = OllamaRequest {
            model: String::from("llama3.2:1b"),
            prompt: String::from("Why is the sky blue?"),
            stream: Some(false),
        };

        match client.generate_with_circuit_breaker(request).await {
            Ok(response) => {
                info!("Got Response: {:?}", response);
            }
            Err(ex) => {
                error!("Error: {:?}", ex);
            }
        }
        sleep(Duration::from_secs(1)).await;
    }
}
