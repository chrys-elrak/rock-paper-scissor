use std::fmt::Display;

use super::lang::Lang;

#[derive(Default, Debug)]
pub struct Stats {
    pub user: usize,
    pub computer: usize,
    pub draws: usize,
    pub lang: Lang,
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} - {}: {} - {}: {}",
            self.lang.you, self.user, self.lang.computer, self.computer, self.lang.draw, self.draws
        )
    }
}

impl Stats {
    pub fn update_stats(&mut self, result: Option<bool>){
        match result {
            Some(true) => self.user += 1,
            Some(false) => self.computer += 1,
            None => self.draws += 1,
        }
    }
    
    pub fn show(self) {
        println!("{}", self);
    }
}