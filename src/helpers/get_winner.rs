use colored::Colorize;

use crate::enums::choice::Choice::{*, self};
// The first argument is the user's choice, the second is the computer's choice
// TODO: Use the graph to determine the winner instead of this function
// Link: https://medium.com/@jp.mfichtl/rock-paper-scissors-lizard-spock-or-why-math-is-awesome-for-coding-405dabe30f4
pub fn winner(user_choice: &Choice, computer_choice: &Choice) -> Option<bool> {
    match (user_choice, computer_choice) {
        (&ROCK, &SCISSOR)    => Some(true),
        (&ROCK, &LIZARD)     => Some(true),
        (&ROCK, &PAPER)      => Some(false),
        (&ROCK, &SPOCK)      => Some(false),
        (&ROCK, &ROCK)       => None,

        (&SCISSOR, &PAPER)   => Some(true),
        (&SCISSOR, &LIZARD)  => Some(true),
        (&SCISSOR, &ROCK)    => Some(false),
        (&SCISSOR, &SPOCK)   => Some(false),
        (&SCISSOR, &SCISSOR) => None,
        
        (&PAPER, &ROCK)      => Some(true),
        (&PAPER, &SPOCK)     => Some(true),
        (&PAPER, &SCISSOR)   => Some(false),
        (&PAPER, &LIZARD)    => Some(false),
        (&PAPER, &PAPER)     => None,

        (&LIZARD, &PAPER)    => Some(true), 
        (&LIZARD, &SPOCK)    => Some(true),
        (&LIZARD, &ROCK)     => Some(false),
        (&LIZARD, &SCISSOR)  => Some(false), 
        (&LIZARD, &LIZARD)   => None,
        
        (&SPOCK, &SCISSOR)   => Some(true),
        (&SPOCK, &ROCK)      => Some(true),
        (&SPOCK, &LIZARD)    => Some(false),
        (&SPOCK, &PAPER)     => Some(false),
        (&SPOCK, &SPOCK)     => None,
    }
}

pub fn message(winner: Option<bool>) {
    match winner {
        Some(true) => println!("{}", "You win !".green()),
        Some(false) => println!("{}", "You loose !".red()),
        None =>  println!("{}", "Draw !".yellow()),
    }
}

// Rock sharpens Scissors
// Rock crushes Lizard

// Scissors cuts Paper
// Scissors decapitates Lizard

// Paper wraps Rock
// Paper disproves Spock

// Lizard poisons Spock
// Lizard eats Paper

// Spock smashes Scissors
// Spock vaporizes Rock


#[cfg(test)]
#[test]
fn user_rock_sharpens_sissors() {
    assert_eq!(winner(&ROCK, &SCISSOR), Some(true));
}

#[test]
fn user_rock_crushes_lizard() {
    assert_eq!(winner(&ROCK, &LIZARD), Some(true));
}

#[test]
fn computer_rock_sharpens_sissors() {
    assert_eq!(winner(&SCISSOR, &ROCK), Some(false));
}

#[test]
fn computer_rock_crushes_lizard() {
    assert_eq!(winner(&LIZARD, &ROCK), Some(false));
}

#[test]
fn user_scissors_cut_paper() {
    assert_eq!(winner(&SCISSOR, &PAPER), Some(true));
}

#[test]
fn user_scissors_decapites_lizard() {
    assert_eq!(winner(&SCISSOR, &LIZARD), Some(true));
}

#[test]
fn computer_scissors_cut_paper() {
    assert_eq!(winner(&PAPER, &SCISSOR), Some(false));
}

#[test]
fn computer_scissors_decapitates_lizard() {
    assert_eq!(winner(&LIZARD, &SCISSOR), Some(false));
}

#[test]
fn user_paper_wraps_rock() {
    assert_eq!(winner(&PAPER, &ROCK), Some(true));
}

#[test]
fn user_paper_disproves_spock() {
    assert_eq!(winner(&PAPER, &SPOCK), Some(true));
}

#[test]
fn computer_paper_wraps_rock() {
    assert_eq!(winner(&ROCK, &PAPER), Some(false));
}

#[test]
fn same_choice_is_draw() {
    assert_eq!(winner(&ROCK, &ROCK), None);
    assert_eq!(winner(&PAPER, &PAPER), None);
    assert_eq!(winner(&SCISSOR, &SCISSOR), None);
}
