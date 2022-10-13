use crate::enums::choice::Choice;
use crate::enums::choice::Choice::{*};

use super::lang::{Lang};

#[derive(Debug)]
pub struct Item {
    pub id: Choice,
    pub name: String,
    pub beats: Vec<Choice>,
}

impl Item {
    pub fn new(lang: Lang) -> Vec<Item> {
         vec![
            Item {
                id: ROCK,
                name: lang.rock,
                beats: vec![SCISSOR, LIZARD],
            },
            Item {
                id: PAPER,
                name: lang.paper,
                beats: vec![ROCK, SPOCK],
            },
            Item {
                id: SCISSOR,
                name: lang.scissors,
                beats: vec![PAPER, LIZARD],
            },
            Item {
                id: SPOCK,
                name: lang.spock,
                beats: vec![ROCK, SCISSOR],
            },
            Item {
                id: LIZARD,
                name: lang.lizard,
                beats: vec![PAPER, SPOCK],
            },
        ]
        
    }
}