use std::{ io::{self, Write} };
use colored::Colorize;

pub fn input(message: &str) -> String {
    print!("\n{}\n\n>>> ", message.on_black());
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or_default();
    input.trim().to_string()
}
