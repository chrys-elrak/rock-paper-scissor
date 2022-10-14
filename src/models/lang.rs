use std::fs;
use serde::{Deserialize};

pub enum AvalaibleLang {
    EN,
    FR,
    MG
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct Lang {
    pub rock: String,
    pub paper: String,
    pub scissors: String,
    pub spock: String,
    pub lizard: String,
    pub exit: String,
}

impl Lang {
    pub fn new(lang: AvalaibleLang) -> Self {
        let file = match lang {
            AvalaibleLang::EN => "src/lang/en.json",
            AvalaibleLang::FR => "src/lang/fr.json",
            AvalaibleLang::MG => "src/lang/mg.json",
        };
        let file = fs::read_to_string(file).expect("Something went wrong reading the file");
        let l: Lang = serde_json::from_str(&file).expect("Something went wrong parsing the file");
        l
    }
}