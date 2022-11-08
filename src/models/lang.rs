use serde::Deserialize;
use std::fs;
use ucli::{item::Item, select::Select, ucli::Main};

#[derive(Clone)]
pub enum AvalaibleLang {
    EN,
    FR,
    MG,
    DE,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct Lang {
    #[serde(alias = "ROCK")]
    pub rock: String,
    #[serde(alias = "PAPER")]
    pub paper: String,
    #[serde(alias = "SCISSORS")]
    pub scissors: String,
    #[serde(alias = "SPOCK")]
    pub spock: String,
    #[serde(alias = "LIZARD")]
    pub lizard: String,
    #[serde(alias = "EXIT")]
    pub exit: String,
    #[serde(alias = "LOADING")]
    pub loading: String,
    #[serde(alias = "YOU")]
    pub you: String,
    #[serde(alias = "COMPUTER")]
    pub computer: String,
    #[serde(alias = "DRAW")]
    pub draw: String,
    #[serde(alias = "DRAW_MESSAGE")]
    pub draw_message: String,
    #[serde(alias = "WIN")]
    pub win: String,
    #[serde(alias = "LOSE")]
    pub lose: String,
    #[serde(alias = "WELCOME")]
    pub welcome: String,
    #[serde(alias = "I_CHOOSE")]
    pub i_choose: String,
    #[serde(alias = "YOU_CHOOSE")]
    pub you_choose: String,
    #[serde(alias = "INPUT_NOT_RECOGNIZED")]
    pub input_failed: String,
    #[serde(alias = "GIVE_INPUT")]
    pub input: String,
    #[serde(alias = "QUIT")]
    pub quit: String,
    #[serde(alias = "CONTINUE")]
    pub continue_: String,
    #[serde(alias = "YES")]
    pub yes: String,
    #[serde(alias = "NO")]
    pub no: String,
}

impl Lang {
    pub fn new(lang: AvalaibleLang) -> Self {
        let file = match lang {
            AvalaibleLang::EN => "src/lang/en.json",
            AvalaibleLang::FR => "src/lang/fr.json",
            AvalaibleLang::MG => "src/lang/mg.json",
            AvalaibleLang::DE => "src/lang/de.json",
        };
        let file = fs::read_to_string(file).expect("Something went wrong reading the file");
        let l: Lang = serde_json::from_str(&file).expect("Something went wrong parsing the file");
        l
    }

    pub fn get_lang() -> Self {
        let lang = Main::new(&Select::new(vec![
            Item::new("English".to_string(), Lang::new(AvalaibleLang::EN), false),
            Item::new("Français".to_string(), Lang::new(AvalaibleLang::FR), false),
            Item::new("Malagasy".to_string(), Lang::new(AvalaibleLang::MG), false),
            Item::new("Deutsch".to_string(), Lang::new(AvalaibleLang::DE), false),
        ]))
        .prompt("Choose your language".to_string())
        .render()
        .get();
        lang.unwrap()
    }
}

impl Default for Lang {
    fn default() -> Self {
        Lang::new(AvalaibleLang::EN)
    }
}
