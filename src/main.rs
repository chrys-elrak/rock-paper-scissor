mod enums;
mod helpers;

use colored::Colorize;
use rand::{self, Rng};

use crate::helpers::get_choice::choice;
use crate::helpers::get_input::input;
use crate::helpers::get_winner::winner;
use crate::helpers::show_loader::loading;

fn main() {
    let header: colored::ColoredString = "
    (1): Rock (default)
    (2): Paper
    (3): Scissor
    "
    .black();
    let user_choice;
    println!("{}", header);
    let s = input("Please choose");
    user_choice = choice(&s).unwrap_or_default();
    let x = rand::thread_rng().gen_range(1..4);
    let computer_choice = choice(x.to_string().as_str()).unwrap();
    loading("Loading...", 100);
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

