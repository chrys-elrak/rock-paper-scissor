use crate::enums::choice::Choice;
use super::{item::Item, lang::Lang};

pub struct Element {
    pub items: Vec<Item>,
}

impl Element {
    pub fn new(lang: &Lang) -> Self {
        Element {
            items: Item::new(lang.to_owned()),
        }
    }

    pub fn get(&self, id: Choice) -> Option<&Item> {
        self.items.iter().find(|item| item.id == id)
    }
    // Check if the user wins
    pub fn win(&self, user: Choice, computer: Choice) -> Option<bool> {
        if user == computer {
            return None
        }
        let user = self.get(user).unwrap();
        let computer = self.get(computer).unwrap();
        Some(user.beats.contains(&computer.id))
    }

}