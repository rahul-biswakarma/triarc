use std::fs::File;

pub trait LLM {
    async fn generate(&self, prompt: &str, artifacts: &[File]) -> Result<String, reqwest::Error>;
}
