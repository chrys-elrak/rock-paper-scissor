mod enums;
mod helpers;
mod models;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::{env::var, process, vec};

use colored::Colorize;
use rand::{self, Rng};
use ucli::{item::Item as uItem, select::Select as uSelect, ucli::Main as uMain};

use crate::enums::choice::Choice;
use crate::helpers::{get_choice::choice, get_input::input, get_winner::message};
use crate::models::{element::Element, lang::Lang, stats::Stats};

fn main() {
    // Check if a pesistent file name is set
    let stats_file = var("ROCK_PAPER_SCISSORS_STATS").unwrap_or("rock_paper_scissors.db".into());
    let stats_path = Path::new(&stats_file);
    if !stats_path.exists() {
        let mut file = File::create(&stats_path).expect(format!("Failed to create {}", stats_file).as_str());
        file.write_all(b"0,0,0").expect("Unable to write to stats file!");
    };
    let persistent_stats = fs::read_to_string(&stats_path)
        .expect(format!("Failed to read from file {}", stats_file).as_str());
    let persistent_stats = persistent_stats.trim().replace("\n", "");
    // Format: Computer,Player,Draw
    let persistent_stats: Vec<usize> = persistent_stats.split(",").map(|v| v.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    // Get the language
    let lang = Lang::get_lang();
    // Print the welcome message
    // init stats
    let mut stats = Stats::new(lang.to_owned(), persistent_stats);
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
                .render()
                .get();
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
        stats.update_stats(result, stats_path);
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
