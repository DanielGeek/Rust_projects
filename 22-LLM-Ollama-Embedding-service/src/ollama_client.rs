use std::time::Duration;

use reqwest::Client;

use crate::{
    circuit_breaker::CircuitBreaker,
    model::{OllamaRequest, OllamaResponse},
    retry_util::retry_with_backoff,
};
use anyhow::{anyhow, Error, Result};

const BASE_URL: &str = "http://localhost:11434/api";

pub struct OllamaClient {
    client: Client,
    circuit_breaker: CircuitBreaker,
}

impl OllamaClient {
    pub fn new() -> Self {
        OllamaClient {
            client: Client::new(),
            circuit_breaker: CircuitBreaker::new(3, 3, Duration::from_secs(5)),
        }
    }

    pub async fn generate_with_circuit_breaker(
        &mut self,
        ollama_request: OllamaRequest,
    ) -> Result<OllamaResponse, Error> {
        self.circuit_breaker
            .call(|| {
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
            })
            .await
    }

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
