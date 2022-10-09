mod enums;
mod helpers;

use colored::Colorize;
use rand::{self, Rng};

use crate::helpers::get_choice::choice;
use crate::helpers::get_input::input;
use crate::helpers::get_winner::{winner, message};
use crate::helpers::show_loader::loading;

fn main() {
    loop {
        let header: colored::ColoredString = "
    (1): Rock (default)
    (2): Paper
    (3): Scissor
    (4): Spock
    (5): Lizard
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
        message(winner(user_choice, computer_choice));
        let s = input("Do you want to play again ? (y/N)");
        if s.to_lowercase() != "y" {
            break;
        }
    }
}
