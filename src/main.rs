mod enums;
mod helpers;
mod models;
use std::process;

use colored::Colorize;
use models::stats;
use rand::{self, Rng};

use crate::helpers::{
    get_choice::choice,
    get_input::input,
    get_winner::message,
    show_loader::loading,
};
use crate::models::{
    element::Element,
    lang::{AvalaibleLang, Lang},
    stats::Stats,
};

fn main() {

    // Get the language
    let lang = match input("Choose your language: [fr/mg/EN]").as_str() {
        "en" => Lang::new(AvalaibleLang::EN),
        "fr" => Lang::new(AvalaibleLang::FR),
        "mg" => Lang::new(AvalaibleLang::MG),
        _ => Lang::new(AvalaibleLang::EN),
    };
    let mut stats = Stats{
        computer: 0,
        user: 0,
        draws: 0,
        lang: lang.to_owned(),
    };
    let e = Element::new(&lang);
    // Show the header
    let header: Vec<String> = e
        .items
        .iter()
        .map(|item| format!("({}) {}", item.id, item.name))
        .collect();
    let header = header.join('\n'.to_string().as_str());
    loop {
        let user_choice;
        'GETINPUT: loop {
            println!("{}", header);
            let c = choice(&input("🤔 Please make you choice"));
            // Get the user input and check if it's a valid choice
            user_choice = match c {
                Some(choice) => choice,
                None => {
                    let err = "\n❌ Input not recognized!\n".red();
                    println!("{}", err);
                    exit(&lang, &stats);
                    continue 'GETINPUT;
                }
            };
            break 'GETINPUT;
        }
        let x = rand::thread_rng().gen_range(1..6);
        let computer_choice = choice(x.to_string().as_str()).unwrap();
        loading("Loading...", 100);
        println!(
            "\nYou choose: {} and I choose: {}",
            &user_choice, &computer_choice
        );
        let result = e.win(user_choice, computer_choice);
        stats.update_stats(result);
        message(result);
        exit(&lang, &stats);
    }
}

fn exit(lang: &Lang, stats: &Stats) {
    let x = input(lang.exit.as_str());
    if x.trim().to_lowercase() == "y" {
        stats.clone().show();
        process::exit(0);
    }
}