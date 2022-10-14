mod enums;
mod helpers;
mod models;
use std::process;
use std::process::Command;

use colored::Colorize;
use rand::{self, Rng};

use crate::enums::choice::Choice;
use crate::helpers::{
    get_choice::choice, get_input::input, get_winner::message, show_loader::loading,
};
use crate::models::{element::Element, lang::Lang, stats::Stats};

fn main() {
    // Get the language
    let lang = Lang::get_lang();
    // Print the welcome message
    println!("\n{}\n", lang.welcome.yellow());
    // init stats
    let mut stats = Stats::new(lang.to_owned());
    // Init element [Rock, Paper, Scissors, Lizard, Spock]
    let e = Element::new(&lang);
    // Show the header
    let header: Vec<String> = e
        .items
        .iter()
        .map(|item| format!("({}) {}", item.id, item.name))
        .collect();
    let header = header.join('\n'.to_string().as_str());
    // MAIN LOOP
    loop {
        let user_choice;
        'GETINPUT: loop {
            println!("\n{}", header);
            let c = choice(&input(lang.input.as_str()));
            // Get the user input and check if it's a valid choice
            user_choice = match c {
                Some(choice) => choice,
                None => {
                    let err = format!("\n{}\n", lang.input_failed.red());
                    println!("{}", err);
                    exit(&lang, &stats);
                    continue 'GETINPUT;
                }
            };
            break 'GETINPUT;
        }
        let x = rand::thread_rng().gen_range(1..e.items.len());
        let computer_choice = choice(x.to_string().as_str()).unwrap();
        // loading(&lang.loading, 100);
        clear();
        println!(
            "\n{}: {} & {}: {}",
            lang.you_choose,
            Choice::get_string_from_id(&user_choice, &lang),
            lang.i_choose,
            Choice::get_string_from_id(&computer_choice, &lang),
        );
        let result = e.win(user_choice, computer_choice);
        stats.update_stats(result);
        message(result, &lang);
        exit(&lang, &stats);
    }
}

fn exit(lang: &Lang, stats: &Stats) {
    let x = input(lang.exit.as_str());
    if x.trim().to_lowercase() == "y" {
        clear();
        stats.clone().show();
        process::exit(0);
    }
}

fn clear() {
    let mut clear_command = "clear";
    if cfg!(windows) {
        clear_command = "cls";
    }
    Command::new(clear_command).status().unwrap();
}
