use crate::models::general::llm::{APIResponse, ChatCompletion, Message};

use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use std::env;

/*
    Этот код отвечает за взаимодействие с API OpenAI для вызова функции "Chat Completions",
        которая используется для отправки сообщений и получения ответов от модели GPT
*/

/*
        Функция call_gpt выполняет следующие задачи:
            Загружает API-ключи и другие параметры из окружения.
            Создаёт HTTP-запрос для вызова API OpenAI.
            Отправляет сообщения модели GPT и получает ответ.
            Обрабатывает ошибки на каждом этапе.
            Возвращает текст ответа от модели.
*/

//  message: Vec<Message>: Вектор сообщений (Message), которые отправляются в модель GPT. Это, например, история чата или конкретные запросы.
pub async fn call_gpt(message: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    // Загружает переменные окружения из файла .env в текущую среду.
    dotenv().ok();

    // Extract api keys from .env
    let api_key: String = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found");
    let api_org: String = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found");

    // Это конечная точка OpenAI для вызова функции "Chat Completions"
    let url: &str = "https://api.openai.com/v1/chat/completions";

    // Создаётся объект HeaderMap, который хранит HTTP-заголовки.
    let mut headers = HeaderMap::new();
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(&api_org.as_str())
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Создаётся клиент с предварительно заданными заголовками (default_headers).
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // create chat completion
    /*
            model: Указание используемой модели (gpt-3.5-turbo).
            messages: Сообщения, которые будут отправлены модели.
            temperature: Параметр, определяющий степень случайности в ответах. Значение 0.1 указывает на минимальную случайность (ответы будут более детерминированными).
    */
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

    //crete request to url and get response as APIResponse
    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    //parse 'content' from massage from choice from api call response
    // Первый элемент из массива choices (модель может вернуть несколько вариантов ответа).
    // Поле message.content, которое содержит текст ответа.
    Ok(res.choices[0].message.content.clone())
}

/*

    Unit Tests

*/

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
