use std::process;
use crate::enums::choice::Choice;

pub fn choice(c: &str) -> Result<Choice, &'static str> {
    match c {
        "1" => Ok(Choice::ROCK),
        "2" => Ok(Choice::PAPER),
        "3" => Ok(Choice::SCISSOR),
        &_ => {
            println!("Choice not recognized!");
            process::exit(1)
        }
    }
}
