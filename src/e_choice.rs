// use std::fmt;

#[derive(Debug)]
pub enum Choice {
    ROCK = 0,
    PAPER = 1,
    SCISSOR = 2,
}


// impl fmt::Display for Choice {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             Choice::ROCK => write!(f, "Rock"),
//             Choice::PAPER => write!(f, "Paper"),
//             Choice::SCISSOR => write!(f, "Scissor"),
//         }
//     }
// }