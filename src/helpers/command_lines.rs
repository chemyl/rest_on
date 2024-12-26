use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand, QueueableCommand,
};
use std::io::{stdin, stdout};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
    Success,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_position: &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = stdout();

        let statement_color: Color = match self {
            PrintCommand::AICall => Color::Cyan,
            PrintCommand::UnitTest => Color::Magenta,
            PrintCommand::Issue => Color::Red,
            PrintCommand::Success => Color::Green,
        };

        // Print the agent statement in a specific color
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {}: ", agent_position);

        //Make selected color
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);

        stdout.execute(ResetColor).unwrap();
    }
}

// get user request
pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    // print the questions in a specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("{}", question);

    // reset color
    stdout.execute(ResetColor).unwrap();
    let mut user_response = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response");
    return user_response.trim().to_string();
}

// get user confirmation of code safety
pub fn confirm_safe_code() -> bool {
    let mut stdout: std::io::Stdout = stdout();
    loop {
        // print questions
        stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
        println!();
        println!("WARNING: You are about to run code written entirely by AI");
        println!("Please, REVIEW CODE and confirm you wish to continue");
        stdout.execute(ResetColor).unwrap();

        // options with different colors
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("[1] All Good");
        stdout.execute(SetForegroundColor(Color::Red)).unwrap();
        println!("[2] Stop project");
        stdout.execute(ResetColor).unwrap();


        //read use input
        let mut human_input = String::new();
        stdin().read_line(&mut human_input).expect("Failed to read user confirmation");

        // trim and make lowercase
        let human_response = human_input.trim().to_lowercase();

        //match human response
        match human_response.as_str() {
            "1" | "ok" | "y" => return true,
            "2" | "no" | "n" => return false,
            _ => {
              println!("Invalid input. Please select one of the following");
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_print_agent_message() {
        PrintCommand::AICall.print_agent_message("Managing Agent", "Testing, Processing something")
    }
}
