use crate::models::general::llm::{Message};

use dotenv::dotenv;
use std::env;

use reqwest::Client;


//call LLM GPT
pub async fn call_gpt(message: Vec<Message>) {
    dotenv().ok();

    //extract api keys
    let api_keys: String = env::var("GPT_API_KEY").expect("GPT_API_KEY");

}