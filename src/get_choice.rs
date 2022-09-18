use std::process;

use crate::e_choice::Choice;

pub fn get_choice(c: &str) -> Result<Choice, &'static str> {
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
