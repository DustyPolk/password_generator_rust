mod password_generator;
mod utils;

use colored::Colorize;
use std::io::{self, Write};

fn main() {
    println!("{}", "Welcome to the Password Generator!".bold().green());

    let password_length = prompt_for_usize(&"Enter password length: ".bold().green().to_string());

    let include_special_chars = prompt_for_yes_no(&"Include special characters? (yes/no): ".bold().green().to_string());

    let include_uppercase = prompt_for_yes_no(&"Include uppercase letters? (yes/no): ".bold().green().to_string());

    match password_generator::generate_password(password_length, include_special_chars, include_uppercase) {
        Ok(password) => println!("{} {}", "Generated password:".bold().green(), password.bold().cyan()),
        Err(e) => println!("{} {}", "Error generating password:".bold().red(), e),
    }
}

fn prompt_for_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Flush the output buffer
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn prompt_for_usize(prompt: &str) -> usize {
    loop {
        let input = prompt_for_input(prompt);
        match input.parse::<usize>() {
            Ok(n) if n > 0 => return n,
            _ => println!("Invalid input. It should be a positive integer."),
        }
    }
}

fn prompt_for_yes_no(prompt: &str) -> bool {
    loop {
        let input = prompt_for_input(prompt);
        match input.to_lowercase().as_str() {
            "yes" => return true,
            "no" => return false,
            _ => println!("Invalid input. It should be either 'yes' or 'no'."),
        }
    }
}