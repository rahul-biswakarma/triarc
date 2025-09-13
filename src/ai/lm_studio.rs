use anyhow::Result;
use reqwest::Response;
use serde::Deserialize;
use serde_json::{Value, json};
use std::{collections::HashMap, time::Duration};

use crate::ai::llm;

pub struct LMStudio {
    model: String,
    api_url: String,
}

impl Default for LMStudio {
    fn default() -> Self {
        Self {
            model: "google/gemma-3-27b".to_string(),
            api_url: "http://127.0.0.1:1234/v1/chat/completions".to_string(),
        }
    }
}

#[derive(Deserialize, Debug)]
struct LMStudioResponseMessages {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct LMStudioResponseChoices {
    index: u32,
    finish_reason: String,
    message: LMStudioResponseMessages,
}

#[derive(Deserialize, Debug)]
struct LMStudioResponse {
    id: String,
    object: String,
    created: u32,
    model: String,
    choices: Vec<LMStudioResponseChoices>,
}

impl llm::LLM for LMStudio {
    async fn generate(&self, prompt: &str, images: &[&str]) -> Result<Response> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap();

        let mut user_message = HashMap::new();
        user_message.insert("role", "user");
        user_message.insert("content", prompt);

        let messages = vec![user_message];

        let mut payload: HashMap<&str, Value> = HashMap::new();
        payload.insert("model", json!(&self.model));
        payload.insert("messages", json!(messages));
        payload.insert("stream", json!(false));
        payload.insert("foramt", json!("json"));

        if !images.is_empty() {
            payload.insert("images", json!(images));
        }

        let response = client.post(&self.api_url).json(&payload).send().await?;

        Ok(response)
    }
    async fn get_content(&self, response: Response) -> Result<String> {
        let response_json: LMStudioResponse = response.json().await?;
        let content: String = response_json.choices[0].message.content.clone();
        Ok(content)
    }
}
