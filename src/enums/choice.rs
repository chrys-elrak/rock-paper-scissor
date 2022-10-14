use std::fmt::{Formatter, Display};
use crate::models::lang::Lang;

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

impl Choice {
    pub fn get_string_from_id(choice: &Self, lang: &Lang) -> String {
        let lang = lang.to_owned();
        match choice {
            Self::ROCK => lang.rock,
            Self::PAPER => lang.paper,
            Self::SCISSOR => lang.scissors,
            Self::SPOCK => lang.spock,
            Self::LIZARD => lang.lizard,
        }
    }
}