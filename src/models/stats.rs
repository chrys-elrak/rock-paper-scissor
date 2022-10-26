use super::lang::Lang;
use colored::Colorize;
use std::{
    fmt::Display,
    fs::OpenOptions,
    io::{Seek, SeekFrom, Write},
    path::Path,
};

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
    pub fn new(lang: Lang, stats: Vec<usize>) -> Self {
        Stats {
            computer: stats[0],
            user: stats[1],
            draws: stats[2],
            lang,
        }
    }
    pub fn update_stats(&mut self, result: Option<bool>, path: &Path) {
        let mut file = OpenOptions::new()
            .write(true)
            .open(path)
            .expect("Unable to write to database");

        match result {
            Some(true) => self.user += 1,
            Some(false) => self.computer += 1,
            None => self.draws += 1,
        }
        file.seek(SeekFrom::Start(0)).unwrap();
        file.write_all(format!("{},{},{}", self.computer, self.user, self.draws).as_bytes())
            .unwrap();
    }
    pub fn show(self) {
        println!("\n{}\n", self);
    }
}
