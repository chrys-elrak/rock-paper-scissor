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
