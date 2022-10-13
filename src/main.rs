mod enums;
mod helpers;
mod models;
use colored::Colorize;
use rand::{self, Rng};

use crate::helpers::get_choice::choice;
use crate::helpers::get_input::input;
use crate::models::item::Item;
use crate::helpers::get_winner::{message, winner};
use crate::helpers::show_loader::loading;
use crate::models::lang::{Lang, AvalaibleLang};

#[derive(Default, Debug)]
struct Stats {
    user: usize,
    computer: usize,
    draws: usize,
}

fn main() {
    let mut stats = Stats::default();
    let lang = match input("Choose your language: [fr/mg/EN]").as_str() {
        "en" => Lang::new(AvalaibleLang::EN),
        "fr" => Lang::new(AvalaibleLang::FR),
        "mg" => Lang::new(AvalaibleLang::MG),
        _ => Lang::new(AvalaibleLang::EN),
    };
    let items = Item::new(lang.clone());
    let mut header = String::new();
    items
    .iter()
    .for_each(|item| {
        let t = format!("({}) {}\n", item.id, item.name );
        header.push_str(t.as_str());
    });
    header.push_str(lang.exit.as_str());
    let header = header.yellow();
    loop {
        let user_choice;
        println!("{}", header);
        let s = input("🤔 Please choose");
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
