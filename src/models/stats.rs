use std::fmt::Display;
use colored::Colorize;
use super::lang::Lang;

#[derive(Clone)]
pub struct Stats {
    pub user: usize,
    pub computer: usize,
    pub draws: usize,
    pub lang: Lang,
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let t = format!(
            "{}: {} - {}: {} - {}: {}",
            self.lang.you, self.user, self.lang.computer, self.computer, self.lang.draw, self.draws
        )
        .bright_magenta();
        write!(f, "{}", t)
    }
}

impl Stats {
    pub fn new(lang: Lang) -> Self {
        Stats {
            computer: 0,
            user: 0,
            draws: 0,
            lang,
        }
    }
    pub fn update_stats(&mut self, result: Option<bool>) {
        match result {
            Some(true) => self.user += 1,
            Some(false) => self.computer += 1,
            None => self.draws += 1,
        }
    }
    pub fn show(self) {
        println!("\n{}\n", self);
    }
}
