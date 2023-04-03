use crate::constants::REPO_ISSUES;
use crate::constants::REPO_LINK;
use crate::data;
use std::error::Error;

pub fn get_translated_list(locale: Option<&str>) -> Result<Vec<&str>, Box<dyn Error>> {
    let language: &str = locale.unwrap_or("en");

    if language == "en" {
        let english_file = data::en::en();
        return Ok(english_file);
    }

    let translated_pokemon_list: Vec<&'static str> = match locale {
        Some("ch") => data::ch::ch(),
        Some("de") => data::de::de(),
        Some("en") => data::en::en(),
        Some("fr") => data::fr::fr(),
        Some("jp") => data::jp::jp(),
        Some("ru") => data::ru::ru(),
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
