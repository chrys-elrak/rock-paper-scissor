mod enums;
mod helpers;
mod models;
use std::process::Command;
use std::{process, vec};

use colored::Colorize;
use rand::{self, Rng};
use ucli::{item::Item as uItem, select::Select as uSelect, ucli::Main as uMain};

use crate::enums::choice::Choice;
use crate::helpers::{get_choice::choice, get_input::input, get_winner::message};
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
    let mut v = vec![];
    for i in 0..e.items.len() {
        let item = &e.items[i];
        v.push(uItem::new(item.name.clone(), item, false));
    }
    println!("{:?}", v);
    let options = uSelect::new(v);
    // MAIN LOOP
    loop {
        let user_choice;
        'GETINPUT: loop {
            let o = uMain::new(&options)
            .prompt(lang.input.clone())
            .render().get();
            // Get the user input and check if it's a valid choice
            user_choice = match o {
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
            Choice::get_string_from_id(&user_choice.id, &lang),
            lang.i_choose,
            Choice::get_string_from_id(&computer_choice, &lang),
        );
        let result = e.win(
            Choice::get_choice_from_id(user_choice.id.to_string().as_str()).unwrap(),
            computer_choice,
        );
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
