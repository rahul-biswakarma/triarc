use anyhow::{Result, anyhow};
use regex::Regex;
use serde::de::DeserializeOwned;

pub fn get_llm_json_response<T>(response: String) -> Result<T>
where
    T: DeserializeOwned,
{
    let regex = Regex::new(r"(?s)```(?:json)?(.*?)```").unwrap();

    if let Some(caps) = regex.captures(&response) {
        if let Some(json_block) = caps.get(1) {
            let json_str = json_block.as_str().trim();
            let parsed: T = serde_json::from_str(json_str)?;
            return Ok(parsed);
        }
    }

    Err(anyhow!("No JSON code block found in response"))
}
