mod enums;
mod helpers;

use colored::Colorize;
use rand::{self, Rng};

use crate::helpers::get_choice::choice;
use crate::helpers::get_input::input;
use crate::helpers::get_winner::{message, winner};
use crate::helpers::show_loader::loading;

#[derive(Default, Debug)]
struct Stats {
    user: usize,
    computer: usize,
    draws: usize,
}

fn main() {
    let mut stats = Stats::default();
    loop {
        let header: colored::ColoredString = "
    (1): Rock
    (2): Paper
    (3): Scissor
    (4): Spock
    (5): Lizard
    Any other key to end the game
    "
        .yellow();
        let user_choice;
        println!("{}", header);
        let s = input("Please choose");
        user_choice = match choice(&s) {
            Some(choice) => choice,
            None => {
                println!("Input not recognized, ending game.");
                break
            }
        };
        let x = rand::thread_rng().gen_range(1..6);
        let computer_choice = choice(x.to_string().as_str()).unwrap();
        loading("Loading...", 100);
        let s1 = "\nYou choose:".cyan();
        let s2 = "I choose:".blue();
        println!("{} {:?}", &s1, &user_choice);
        println!("{} {:?}", &s2, &computer_choice);
        match winner(&user_choice, &computer_choice) {
            Some(true) => stats.user += 1,
            Some(false) => stats.computer += 1,
            None => stats.draws += 1,
        }
        message(winner(&user_choice, &computer_choice));
    }
    println!("{:?}", stats);

}
