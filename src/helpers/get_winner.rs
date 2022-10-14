use colored::Colorize;

use crate::models::lang::Lang;

pub fn message(winner: Option<bool>, lang: &Lang) {
    match winner {
        Some(true) => println!("{}", lang.win.green()),
        Some(false) => println!("{}", lang.lose.red()),
        None =>  println!("{}", lang.draw.yellow()),
    }
}