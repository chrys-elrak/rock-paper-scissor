use colored::Colorize;

use crate::enums::choice::Choice::{*, self};
// The first argument is the user's choice, the second is the computer's choice
// TODO: Use the graph to determine the winner instead of this function
// Link: https://medium.com/@jp.mfichtl/rock-paper-scissors-lizard-spock-or-why-math-is-awesome-for-coding-405dabe30f4
pub fn winner(user_choice: Choice, computer_choice: Choice) -> (bool, bool) {
    match (user_choice, computer_choice) {
        (ROCK, SCISSOR)    => (true, false),
        (SCISSOR, ROCK)    => (false, true),
        (SCISSOR, PAPER)   => (true, false),
        (PAPER, SCISSOR)   => (false, true),
        (PAPER, ROCK)      => (true, false),
        (ROCK, PAPER)      => (false, true),
        (SCISSOR, SCISSOR) => (true, true),
        (ROCK, ROCK)       => (true, true),
        (PAPER, PAPER)     => (true, true),
        (LIZARD, SPOCK)    => (false, true),
        (SPOCK, LIZARD)    => (true, false),
        (LIZARD, PAPER)    => (true, false),
        (PAPER, LIZARD)    => (false, true),
        (LIZARD, ROCK)     => (false, true),
        (ROCK, LIZARD)     => (true, false),
        (SPOCK, SCISSOR)   => (false, true),
        (SCISSOR, SPOCK)   => (true, false),
        (SPOCK, ROCK)      => (false, true),
        (ROCK, SPOCK)      => (true, false),
        (SPOCK, PAPER)     => (true, false),
        (PAPER, SPOCK)     => (false, true),
        (SCISSOR, LIZARD)  => (true, false),
        (SPOCK, SPOCK)     => (true, true),
        (LIZARD, LIZARD)   => (true, true),
        (LIZARD, SCISSOR)  => (false, true),
    }
}

pub fn message(winner: (bool, bool)) {
    let (user, computer) = winner;
    if user == computer {
        println!("{}", "Draw !".yellow());
    } else if computer {
        println!("{}", "You lose !".red());
    } else {
        println!("{}", "You win !".green());
    }
}

#[cfg(test)]
#[test]
fn user_rock_sharpens_sissors() {
    assert_eq!(winner(ROCK, SCISSOR), (true, false));
}

#[test]
fn computer_rock_sharpens_sissors() {
    assert_eq!(winner(SCISSOR, ROCK), (false, true));
}

#[test]
fn user_scissors_cut_paper() {
    assert_eq!(winner(SCISSOR, PAPER), (true, false));
   
}

#[test]
fn computer_scissors_cut_paper() {
    assert_eq!(winner(PAPER, SCISSOR), (false, true));
}

#[test]
fn user_paper_wraps_rock() {
    assert_eq!(winner(PAPER, ROCK), (true, false));
}

#[test]
fn computer_paper_wraps_rock() {
    assert_eq!(winner(ROCK, PAPER), (false, true));
}

#[test]
fn same_choice_is_draw() {
    assert_eq!(winner(ROCK, ROCK), (true, true));
    assert_eq!(winner(PAPER, PAPER), (true, true));
    assert_eq!(winner(SCISSOR, SCISSOR), (true, true));
}
