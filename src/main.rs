mod enums;
mod helpers;

use colored::Colorize;
use rand::{self, Rng};
use std::io::Write;

use crate::helpers::get_choice::choice;
use crate::helpers::get_input::input;
use crate::helpers::get_winner::winner;

fn main() {
    let header: colored::ColoredString = "
    (1): Rock
    (2): Paper
    (3): Scissor
    "
    .black();
    let user_choice;
    loop {
        println!("{}", header);
        let s = input("Please choose");
        user_choice = choice(&s).unwrap();
        break;
    }
    let x = rand::thread_rng().gen_range(1..4);
    let computer_choice = choice(x.to_string().as_str()).unwrap();
    for v in "Loading...".as_bytes().iter() {
        std::thread::sleep(std::time::Duration::from_millis(50));
        print!("{}", *v as char);
        std::io::stdout().flush().unwrap();
    }
    let s1 = "\nYou choose:".cyan();
    let s2 = "I choose:".blue();
    println!("{} {:?}", s1, user_choice.clone());
    println!("{} {:?}", s2, computer_choice.clone());
    let (user_win, computer_win) = winner(user_choice, computer_choice);
    if user_win == computer_win {
        println!("{}", "Draw !".yellow());
    } else if computer_win {
        println!("{}", "You lose !".red());
    } else {
        println!("{}", "You win !".green());
    }
}

