use anyhow::Result;
use reqwest::Response;
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashMap;

use crate::ai::llm;

pub struct Ollama {
    model: String,
    api_url: String,
}

impl Default for Ollama {
    fn default() -> Self {
        Self {
            model: "gemma3:12b".to_string(),
            api_url: "http://localhost:11434/api/generate".to_string(),
        }
    }
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

impl llm::LLM for Ollama {
    async fn generate(&self, prompt: &str, images: &[&str]) -> Result<Response> {
        let client = reqwest::Client::new();

        let mut payload: HashMap<&str, Value> = HashMap::new();
        payload.insert("model", json!(&self.model));
        payload.insert("prompt", json!(prompt));
        payload.insert("stream", json!(false));
        payload.insert("foramt", json!("json"));

        if !images.is_empty() {
            payload.insert("images", json!(images));
        }

        let response = client.post(&self.api_url).json(&payload).send().await?;

        Ok(response)
    }
    async fn get_content(&self, response: Response) -> Result<String> {
        let response_json: OllamaResponse = response.json().await?;

        Ok(response_json.response)
    }
}
