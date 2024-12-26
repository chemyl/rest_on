use crate::models::general::llm::{APIResponse, ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::env;

/// Calls the OpenAI GPT API to get a response based on the provided chat messages.
///
/// # Arguments
///
/// * `messages` - A vector of `Message` structs representing the conversation history.
///
/// # Returns
///
/// This function returns a `Result` which contains:
/// - A `String` with the response content if the request is successful.
/// - A boxed `dyn std::error::Error` if an error occurs during the request or response parsing.
///
/// # Errors
///
/// This function can fail in several ways:
/// - If the environment variables `OPEN_AI_KEY` or `OPEN_AI_ORG` are not found.
/// - If the HTTP request fails.
/// - If the JSON response parsing fails.
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();
    let api_key: String = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found in .env");
    let api_org: String = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found in .env");
    let url: &str = "https://api.openai.com/v1/chat/completions";
    let mut headers: HeaderMap = HeaderMap::new();

    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str())
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    let client: Client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    let chat_completion: ChatCompletion = ChatCompletion {
        model: "gpt-4".to_string(),
        // model: "gpt-3.5-turbo".to_string(),
        messages,
        temperature: 0.1,
    };

    let llm_raw_response: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    Ok(llm_raw_response.choices[0].message.content.clone())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[tokio::test]
//     async fn tests_call_to_openai() {
//         let message: Message = Message {
//             role: "user".to_string(),
//             content: "Hi there, this is a test. Give me a short reponse.".to_string(),
//         };
//         let messages: Vec<Message> = vec![message];
//         let res: Result<String, Box<dyn std::error::Error + Send>> = call_gpt(messages).await;
//         match res {
//             Ok(res_str) => {
//                 dbg!(res_str);
//                 assert!(true);
//             }
//             Err(_) => {
//                 assert!(false);
//             }
//         }
//     }
// }
