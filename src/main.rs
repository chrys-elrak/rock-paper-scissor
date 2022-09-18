mod e_choice;
mod prompt;
mod get_choice;

use std::io::Write;

use prompt::input;
use rand::{self, Rng};
use  get_choice::{get_choice};

fn main() {
    let user_choice;
    loop {
        println!(
            "
        (1): Rock
        (2): Paper
        (3): Scissor
    "
        );
        let s = input("Please choose");
        user_choice = get_choice(&s);
        break;
    }
    let x = rand::thread_rng().gen_range(1..4);
    let computer_choice = get_choice(x.to_string().as_str());
    for v in "Loading...".as_bytes().iter() {
        std::thread::sleep(std::time::Duration::from_millis(100));
        print!("{}", *v as char);
        std::io::stdout().flush().unwrap();
    }
    println!("\nYou choose: {:?}", { user_choice.unwrap() });
    println!("I choose: {:?}", { computer_choice.unwrap() });
}

