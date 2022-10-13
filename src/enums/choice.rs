use std::fmt::{Formatter, Display};

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Choice {
    ROCK = 0,
    PAPER = 1,
    SCISSOR = 2,
    SPOCK = 3,
    LIZARD = 4,
}

impl  Default for Choice {
    fn default() -> Self {
        Choice::ROCK
    }
}

impl Display for Choice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Choice::ROCK => write!(f, "1"),
            Choice::PAPER => write!(f, "2"),
            Choice::SCISSOR => write!(f, "3"),
            Choice::SPOCK => write!(f, "4"),
            Choice::LIZARD => write!(f, "5"),
        }
    }    
}