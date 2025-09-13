use anyhow::Result;
use std::collections::HashMap;

use crate::{ai::llm::LLM, utils::get_llm_json_response};

mod ai;
mod utils;

#[tokio::main]
async fn main() {
    let llm = crate::ai::lm_studio::LMStudio::default();
    let images: Vec<&str> = Vec::new();
    let result = llm.generate(r#"
    Generate a compact JSON map definition for a 2D side-scrolling platformer level (inspired by Hollow Knight), designed for terminal rendering.
	•	The JSON keys must define rectangular coordinate ranges in the format:
	•	"x_start..x_end, y_start..y_end"
	•	Single coordinates can be "x, y"
	•	Mixed ranges like "x_start..x_end, y" or "x, y_start..y_end" are allowed.
	•	The JSON values must be single-character symbols representing tiles, e.g.:
	•	" " → empty space
	•	"-" → ground/floor/platform
	•	"|" → wall
	•	"=" → moving platform
	•	"~" → spikes
	•	"@" → player spawn
	•	"o" → collectible item
	•	"\#" → destructible/interactive block
	•	Ranges should be merged wherever possible to keep the JSON compact. Overlapping ranges are allowed, later ones override earlier ones.
	•	The output should only be valid JSON — no explanations, no extra text.
	•	Example:

    {
      "0..200,0..200": " ",
      "0..200,198..200": "-",
      "95..120,150": "-",
      "50,100..105": "|",
      "75,120": "@"
    }
    Now generate a medium-sized level (about 1000x500) with a main floor, some floating platforms, walls, spikes, and a spawn point.
        "#, &images).await;
    match result {
        Ok(response) => {
            let res = llm.get_content(response).await;
            match res {
                Ok(res) => {
                    let map: Result<HashMap<String, String>> = get_llm_json_response(res);
                    println!("{:?}", map);
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
