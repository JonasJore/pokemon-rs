use crate::data;
use crate::util::panic_by_reason::panic_handling::{get_panic_by_reason, PanicReason};
use std::error::Error;

pub fn get_translated_list(locale: Option<&str>) -> Result<Vec<&str>, Box<dyn Error>> {
    let translated_pokemon_list: Vec<&'static str> = match locale.unwrap_or("en") {
        "cn" => data::cn::cn(),
        "de" => data::de::de(),
        "en" => data::en::en(),
        "fr" => data::fr::fr(),
        "jp" => data::jp::jp(),
        "ru" => data::ru::ru(),
        _ => get_panic_by_reason(PanicReason::UnsupportedLanguageContribute),
    };

    Ok(translated_pokemon_list)
}

pub fn get_pokemon(locale: Option<&str>) -> Option<Vec<&str>> {
    let result = get_translated_list(locale);
    match result {
        Ok(vec) => Some(vec),
        Err(_) => get_panic_by_reason(PanicReason::TranslatedList),
    }
}
