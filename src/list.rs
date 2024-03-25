use crate::data;
use crate::github::constants::{REPO_ISSUES, REPO_LINK};
use std::error::Error;

pub fn get_translated_list(locale: Option<&str>) -> Result<Vec<&str>, Box<dyn Error>> {
    let translated_pokemon_list: Vec<&'static str> = match locale.unwrap_or("en") {
        "ch" => data::ch::ch(),
        "de" => data::de::de(),
        "en" => data::en::en(),
        "fr" => data::fr::fr(),
        "jp" => data::jp::jp(),
        "ru" => data::ru::ru(),
        _ => panic!(
            "Language currently not supported. Want support for your language? Pull requests welcome at {}. Or you can just post a feature request as an issue here: {}",
            REPO_LINK,
            REPO_ISSUES
        )
    };

    Ok(translated_pokemon_list)
}

pub fn get_pokemon(locale: Option<&str>) -> Result<Vec<&str>, Box<dyn Error>> {
    get_translated_list(locale)
}
