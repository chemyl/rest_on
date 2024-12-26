use serde::{Deserialize, Serialize};

/// Represents a single message in a chat, including its role and content.
///
/// # Fields
/// - `role`: The role of the sender (e.g., "user", "assistant").
/// - `content`: The content of the message.

#[derive(Debug, Serialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}
/// Represents the payload for a chat completion request, including the model, messages, and temperature.
///
/// # Fields
/// - `model`: The identifier of the language model to be used (e.g., "gpt-4").
/// - `message`: The collection of messages in the conversation.
/// - `temperature`: The randomness level for the model's output (higher values lead to more varied responses).
#[derive(Debug, Serialize, Clone)]
pub struct ChatCompletion {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
}
/// Represents a single API message returned as part of a response.
///
/// # Fields
/// - `content`: The content of the message provided by the API.

#[derive(Debug, Deserialize)]
pub struct APIMessage {
    pub content: String,
}
/// Represents a single choice returned in the API response.
///
/// # Fields
/// - `message`: The message corresponding to the choice.
#[derive(Debug, Deserialize)]
pub struct APIChoice {
    pub message: APIMessage,
}
/// Represents the entire response returned by the API, containing a list of choices.
///
/// # Fields
/// - `choices`: The list of choices provided in the API response.
#[derive(Debug, Deserialize)]
pub struct APIResponse {
    pub choices: Vec<APIChoice>,
}
