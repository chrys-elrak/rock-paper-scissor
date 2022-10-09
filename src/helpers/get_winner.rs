use crate::enums::choice::Choice::{*, self};
/*
* First return is for user, the last for computer
*/
pub fn winner(user_choice: Choice, computer_choice: Choice) -> (bool, bool) {
    let res = if user_choice == ROCK && computer_choice == PAPER
        || user_choice == SCISSOR && computer_choice == ROCK
    {
        (false, true)
    } else if user_choice == computer_choice {
        (true, true)
    } else {
        (true, false)
    };
    return res;
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
