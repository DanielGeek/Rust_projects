use std::time::Duration;

use reqwest::Client;

use crate::{
    model::{OllamaRequest, OllamaResponse},
    retry_util::retry_with_backoff,
};
use anyhow::{Result, anyhow};

const BASE_URL: &str = "http://localhost:11434/api";

#[derive(Default)]
pub struct OllamaClient {
    client: Client,
}

impl OllamaClient {
    pub async fn generate(&self, ollama_request: OllamaRequest) -> Result<OllamaResponse> {
        retry_with_backoff(
            || {
                let url = format!("{}/generate", BASE_URL);
                async {
                    let response = self.client.post(url).json(&ollama_request).send().await?;

                    if !response.status().is_success() {
                        let error_text = response.text().await?;
                        return Err(anyhow!("API request failed: {}", error_text));
                    }

                    let api_response = response.json().await?;

                    Ok(api_response)
                }
            },
            3,
            Duration::from_secs(2),
        )
        .await
    }
}
