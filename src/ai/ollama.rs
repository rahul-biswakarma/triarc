use anyhow::Result;
use base64::{Engine, engine::general_purpose};
use serde_json::json;
use std::{collections::HashMap, fs::File, io::Read};

use super::llm::LLM;

pub struct Ollama {
    model: String,
    api_path: String,
}

impl Default for Ollama {
    fn default() -> Self {
        Self {
            model: "gemma3:12b".to_string(),
            api_path: "http://localhost:11434/api/generate".to_string(),
        }
    }
}

impl LLM for Ollama {
    async fn generate(&self, prompt: &str, artifact: &[File]) -> Result<String, reqwest::Error> {
        let mut images: Vec<String> = Vec::new();
        for file in artifact {
            let mut buf = Vec::new();
            let mut f = file.try_clone().expect("Failed to clone file");
            f.read_to_end(&mut buf).expect("Failed to read file");

            let b64 = general_purpose::STANDARD.encode(buf);
            images.push(b64);
        }

        let mut payload = HashMap::new();
        payload.insert("model", json!(self.model));
        payload.insert("prompt", json!(prompt));

        if !images.is_empty() {
            payload.insert("images", json!(images));
        }

        // Send request
        let client = reqwest::Client::new();
        let res = client.post(&self.api_path).json(&payload).send().await?;

        // Return response text
        Ok(res.text().await?)
    }
}
