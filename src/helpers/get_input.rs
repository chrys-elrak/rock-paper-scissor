use std::{ io::{self, Write} };

pub fn input(message: &'static str) -> String {
    print!("{}: ", message);
    io::stdout().flush().expect("");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or_default();
    input.trim().to_string()
}