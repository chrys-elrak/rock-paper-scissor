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