use crate::api_handler::call_request::call_gpt;
use crate::helpers::command_lines::PrintCommand;
use crate::models::general::llm::Message;
use serde::de::DeserializeOwned;
use serde::Deserialize;


// Extend AI function for specific output
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    // Вызывает функцию ai_func, передавая ей func_input, и получает результат в виде строки (ai_function_string).
    let ai_function_string = ai_func(func_input);

    // Extend the string to encourage only printing the output

    let msg: String = format!("FUNCTION {} INSTRUCTION: You are a function printer. \
    You ONLY prints the result of a function. \
    NOTHING else. \
    NO commentary. \
    Here is the input to the function: {}", ai_function_string, func_input);

    // Возвращает Message, который используется для отправки запроса в модель GPT.
    Message {
        role: "system".to_string(),
        content: msg,
    }
}

// performs call to LLm GPT

pub async fn ai_task_request(
    /*
        msg_context: Строка контекста для функции (входные данные пользователя).
        agent_position: Позиция агента (например, роль или текущая задача).
        agent_operation: Операция, выполняемая агентом (например, "обработка запроса").
        function_pass: Ссылка на функцию, которая обрабатывает входной контекст (fn(&str) -> &'static str).
    */
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str, ) -> String
{
    // Расширяет функцию с помощью extend_ai_function, чтобы создать сообщение для LLM.
    let extend_message: Message = extend_ai_function(function_pass, &msg_context);

    // Печатает текущую информацию о вызове агента
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    // Вызывает LLM через call_gpt с созданным сообщением extend_message
    let llm_response_res = call_gpt(vec![extend_message.clone()]).await;

    // возвращает ответ модели. or try again
    match llm_response_res {
        Ok(llm_response) => llm_response,
        Err(_) => call_gpt(vec![extend_message.clone()])
            .await.expect("Could not call GPT function"),
    }
}


/*
        Эта функция ai_task_request_decoded предназначена для вызова GPT через асинхронную функцию ai_task_request,
            получения строки-ответа, а затем преобразования (десериализации)
            этой строки в объект нужного типа T с использованием библиотеки serde.
*/
pub async fn ai_task_request_decoded<T: DeserializeOwned>(          // тип T должен поддерживать десериализацию из JSON.
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {

    let llm_response: String = ai_task_request(
        msg_context,
        agent_position,
        agent_operation,
        function_pass
    ).await;
//  Десериализует строку JSON в объект типа T
    let decoded_response: T = serde_json::from_str(&llm_response.as_str()).expect("Could not deserialize GPT response from serde_json");
    decoded_response
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::ai_func_manager::convert_user_input_to_goal;
    use crate::helpers::general::extend_ai_function;

    #[test]
    fn test_extending_ai_function() {
        let extended_string = extend_ai_function(convert_user_input_to_goal, "dummy string");
        assert_eq!(extended_string.role, "system".to_string());
        dbg!(extended_string);
    }

    #[tokio::test]
    async fn test_ai_task_request() {
        let ai_func_param = "Build me a webserver for making stock api requests".to_string();
        let res = ai_task_request(
            ai_func_param,
            "managing Agent",
            "Defining user requirements",
            convert_user_input_to_goal).await;

        dbg!(res);
    }
}