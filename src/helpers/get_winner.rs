use colored::Colorize;

use crate::models::lang::Lang;

pub fn message(winner: Option<bool>, lang: &Lang) {
    match winner {
        Some(true) => println!("\n{}", lang.win.green()),
        Some(false) => println!("\n{}", lang.lose.red()),
        None =>  println!("\n{}", lang.draw_message.yellow()),
    }
}