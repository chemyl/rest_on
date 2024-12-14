use crate::models::general::llm::{APIResponse, ChatCompletion, Message};

use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use std::env;
// backend_hive

// Call LLM GPT
pub async fn call_gpt(message: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
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
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send>{ Box::new(e) })?);

    // Create OpenAI org Header
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(&api_org.as_str())
            .map_err(|e| -> Box<dyn std::error::Error + Send>{ Box::new(e) })?);

    //create client
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send>{ Box::new(e) })?;

    // create chat completion
    let chat_completion: ChatCompletion = ChatCompletion {
        model: "gpt-3.5-turbo".to_string(),
        messages: message,
        temperature: 0.1,
    };

    // // Troubleshooting
    // let res_raw = client
    //     .post(url)
    //     .json(&chat_completion)
    //     .send()
    //     .await.unwrap();
    //
    // dbg!(&res_raw.text().await.unwrap());


    //GET API response
    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send>{ Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send>{ Box::new(e) })?;

    //send response
    Ok(res.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_to_call_gpt() {
        let message = Message {
            role: "user".to_string(),
            content: "hi this-is test. Give me a shot response".to_string(),
        };
        let messages: Vec<Message> = vec![message];
        let res: Result<String, Box<dyn std::error::Error + Send>> = call_gpt(messages).await;
        match res {
            Ok(response_str) => {
                dbg!(&response_str);
                assert!(true);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }
}