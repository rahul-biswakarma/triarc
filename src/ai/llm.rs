use anyhow::Result;
use reqwest::Response;

pub trait LLM {
    async fn generate(&self, prompt: &str, images: &[&str]) -> Result<Response>;
    async fn get_content(&self, response: Response) -> Result<String>;
}
