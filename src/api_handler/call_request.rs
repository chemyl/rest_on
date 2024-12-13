use crate::models::general::llm::Message;

use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use std::env;
// backend_hive

// Call LLM GPT
pub async fn call_gpt(message: Vec<Message>) {
    dotenv().ok();

    // Extract api keys
    let api_key: String = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found");
    let api_org: String = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found");

    // Confirm endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";

    // Create headers
    let mut headers = HeaderMap::new();

    // Create api key headers
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );

    // Create OpenAI org Header
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(&api_org.as_str()).unwrap(),
    );
}