use crate::models::general::llm::Message;


// Extend AI function for specific output
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_string = ai_func(func_input);

    // Extend the string to encourage only printing the output

    let msg: String = format!("FUNCTION {} INSTRUCTION: You are a function printer. \
    You ONLY prints the result of a function. \
    NOTHING else. \
    NO commentary. \
    Here is the input to the function: {}", ai_function_string, func_input);

    // return Message
    Message {
        role: "system".to_string(),
        content: msg,
    }
}


#[cfg(test)]
mod tests {
    use crate::ai_functions::ai_func_manager::convert_user_input_to_goal;
    use crate::helpers::general::extend_ai_function;

    #[test]
    fn test_extending_ai_function() {
        let extended_string = extend_ai_function(convert_user_input_to_goal, "dummy string");
        assert_eq!(extended_string.role, "system".to_string());
        dbg!(extended_string);
    }
}