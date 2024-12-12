use crossterm::{style::{Color, ResetColor, SetForegroundColor}, ExecutableCommand};
use std::io::stdin;

// get user request
pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = std::io::stdout();

    // print the questions in a specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);

    // reset color
    stdout.execute(ResetColor).unwrap();
    let mut user_response = String::new();
    stdin().read_line(&mut user_response).expect("Failed to read response");
    return user_response.trim().to_string();
}