use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{stdin, stdout};

/// Enum representing different print command types for agent messages and statuses.
#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
    Success,
}

impl PrintCommand {
    /// Prints a message from an agent with different formatting based on the command type.
    ///
    /// # Arguments
    ///
    /// * `agent_position` - A string representing the agent's position or role.
    /// * `agent_statement` - A string representing the message the agent wants to print.
    ///
    /// # Behavior
    ///
    /// The message will be printed to stdout with a specific color depending on the `PrintCommand` variant:
    /// - `AICall` prints in cyan.
    /// - `UnitTest` prints in magenta.
    /// - `Issue` prints in red.
    /// - `Success` prints in green.
    ///
    /// The method also prints the agent's position, followed by the agent's message.
    pub fn print_agent_message(&self, agent_position: &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = stdout();

        let statement_color: Color = match self {
            PrintCommand::AICall => Color::Cyan,
            PrintCommand::UnitTest => Color::Magenta,
            PrintCommand::Issue => Color::Red,
            PrintCommand::Success => Color::Green,
        };

        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {}: ", agent_position);

        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);

        stdout.execute(ResetColor).unwrap();
    }
}

/// Prompts the user with a question and retrieves their response.
///
/// # Arguments
///
/// * `question` - The question to ask the user.
///
/// # Returns
///
/// This function returns a `String` representing the user's input after trimming whitespace.
///
/// # Example
/// ```
/// let response = get_user_response("What is your name?");
/// ```

pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("{}", question);

    stdout.execute(ResetColor).unwrap();
    let mut user_response = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response");
    user_response.trim().to_string()
}

/// Prompts the user to confirm whether it is safe to run code written by AI.
///
/// # Returns
///
/// This function returns a `bool`:
/// - `true` if the user confirms to continue with the code execution.
/// - `false` if the user chooses to stop the project.
///
/// # Behavior
///
/// The user will be presented with a warning and two options:
/// - `[1]` to confirm it's all good.
/// - `[2]` to stop the project.
///
/// If the input is invalid, the prompt will ask again until a valid response is received.

pub fn confirm_safe_code() -> bool {
    let mut stdout: std::io::Stdout = stdout();
    loop {
        stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
        println!();
        println!("WARNING: You are about to run code written entirely by AI");
        println!("Please, REVIEW CODE and confirm you wish to continue");

        stdout.execute(ResetColor).unwrap();
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("[1] All Good");
        stdout.execute(SetForegroundColor(Color::Red)).unwrap();
        println!("[2] Stop project");
        stdout.execute(ResetColor).unwrap();

        let mut human_input = String::new();
        stdin()
            .read_line(&mut human_input)
            .expect("Failed to read user confirmation");

        let human_response = human_input.trim().to_lowercase();

        match human_response.as_str() {
            "1" | "ok" | "y" => return true,
            "2" | "no" | "n" => return false,
            _ => {
                println!("Invalid input. Please select one of the following");
            }
        };
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_print_agent_message() {
//         PrintCommand::AICall.print_agent_message("Managing Agent", "Testing, Processing something")
//     }
// }
