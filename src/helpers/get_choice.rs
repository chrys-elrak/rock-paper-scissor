use crate::enums::choice::Choice;

pub fn choice(c: &str) -> Option<Choice> {
    match c {
        "1" => Some(Choice::ROCK),
        "2" => Some(Choice::PAPER),
        "3" => Some(Choice::SCISSOR),
        _ => None
    }
}
