use colored::Colorize;

pub fn message(winner: Option<bool>) {
    match winner {
        Some(true) => println!("{}", "You win !".green()),
        Some(false) => println!("{}", "You loose !".red()),
        None =>  println!("{}", "Draw !".yellow()),
    }
}