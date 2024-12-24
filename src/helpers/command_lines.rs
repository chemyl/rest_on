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
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_position: &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = stdout();

        let statement_color: Color = match self {
            PrintCommand::AICall => Color::Cyan,
            PrintCommand::UnitTest => Color::Magenta,
            PrintCommand::Issue => Color::Red,
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_print_agent_message() {
        PrintCommand::AICall.print_agent_message("Managing Agent", "Testing, Processing something")
    }
}
