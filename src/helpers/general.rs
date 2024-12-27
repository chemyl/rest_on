use crate::api_handler::call_request::call_gpt;
use crate::helpers::command_lines::PrintCommand;
use crate::models::general::llm::Message;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::path::PathBuf;
use std::{env, fs};

fn get_project_root() -> PathBuf {
    env::current_dir().expect("Failed to get current directory")
}
const EXEC_MAIN_PATH: &str = r#"src\main2.rs"#;
const API_SCHEMA_PATH: &str = r#"source\schemas\api_schema.json"#;
const CODE_TEMPLATE_PATH: &str = r#"source\web_server_code_template.rs"#;

pub fn get_web_server_project_path() -> String {
    // let project_root = get_project_root();
    get_project_root().to_str().unwrap().to_string()
}

pub fn get_exec_main_path() -> String {
    let project_root = get_project_root();
    project_root.join(EXEC_MAIN_PATH).to_str().unwrap().to_string()
}

pub fn get_api_schema_path() -> String {
    let project_root = get_project_root();
    project_root.join(API_SCHEMA_PATH).to_str().unwrap().to_string()
}

pub fn get_code_template_path() -> String {
    let project_root = get_project_root();
    project_root.join(CODE_TEMPLATE_PATH).to_str().unwrap().to_string()
}

/// Extends an AI function by formatting the input and creating a system message.
/// This function prepares a message in the format expected by GPT models.
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str: &str = ai_func(func_input);

    let msg: String = format!(
        "FUNCTION: {}
  INSTRUCTION: You are a function printer. You ONLY print the results of functions.
  Nothing else. No commentary. Here is the input to the function: {}.
  Print out what the function will return.",
        ai_function_str, func_input
    );

    Message {
        role: "system".to_string(),
        content: msg,
    }
}

/// Sends a request to the AI service with a provided message context and function.
/// It calls the `call_gpt` function to interact with the OpenAI API, printing relevant agent messages.
pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    let extended_msg: Message = extend_ai_function(function_pass, &msg_context);

    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    let llm_response_res: Result<String, Box<dyn std::error::Error + Send>> =
        call_gpt(vec![extended_msg.clone()]).await;

    match llm_response_res {
        Ok(llm_resp) => llm_resp,
        Err(_) => call_gpt(vec![extended_msg.clone()])
            .await
            .expect("Failed twice to call OpenAI"),
    }
}

/// Sends a request to the AI service and decodes the response into a specific type.
///
/// # Type Parameters
/// - `T`: The type that the response will be deserialized into. It must implement `DeserializeOwned`.
pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response: String =
        ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;
    let decoded_response: T = serde_json::from_str(llm_response.as_str())
        .expect("Failed to decode ai response from serde_json");
    return decoded_response;
}

/// Checks the HTTP status code of a URL using the provided client.
///
/// # Arguments
/// - `client`: The `reqwest::Client` used to make the request.
/// - `url`: The URL to send the GET request to.
///
/// # Returns
/// The HTTP status code of the response.
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response: reqwest::Response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

/// Reads the contents of the web server code template from the specified path.
///
/// # Returns
/// The contents of the code template as a `String`.
pub fn read_code_template_contents() -> String {
    let path: String = String::from(get_code_template_path());
    fs::read_to_string(path).expect("Failed to read code template")
}

/// Reads the contents of the main server file from the specified path.
///
/// # Returns
/// The contents of the `main2.rs` file as a `String`.
pub fn read_exec_main_contents() -> String {
    let path: String = String::from(get_exec_main_path());
    fs::read_to_string(path).expect("Failed to read code template")
}

/// Saves the backend code to the `main2.rs` file.
///
/// # Arguments
/// - `contents`: The code to write into the file.
pub fn save_backend_code(contents: &String) {
    let path: String = String::from(get_exec_main_path());
    fs::write(path, contents).expect("Failed to write main2.rs file");
}

/// Saves the API endpoints to a JSON file.
///
/// # Arguments
/// - `api_endpoints`: The API endpoints to save as a JSON string.
pub fn save_api_endpoints(api_endpoints: &String) {
    let path: String = String::from(get_api_schema_path());
    fs::write(path, api_endpoints).expect("Failed to write API Endpoints to file");
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn tests_paths() {
//         println!("{:?}", get_api_schema_path());
//         println!("{:?}", get_code_template_path());
//         println!("{:?}", get_exec_main_path());
//         println!("{:?}", get_web_server_project_path());
//     }
// }


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::ai_functions::ai_func_manager::convert_user_input_to_goal;
//
//     #[test]
//     fn tests_extending_ai_function() {
//         let extended_msg: Message =
//             extend_ai_function(convert_user_input_to_goal, "dummy variable");
//         dbg!(&extended_msg);
//         assert_eq!(extended_msg.role, "system".to_string());
//     }
//
//     #[tokio::test]
//     async fn tests_ai_task_request() {
//         let ai_func_param: String =
//             "Build me a webserver for making stock price api requests.".to_string();
//
//         let res: String = ai_task_request(
//             ai_func_param,
//             "Managing Agent",
//             "Defining user requirements",
//             convert_user_input_to_goal,
//         )
//         .await;
//
//         assert!(res.len() > 20);
//     }
// }
