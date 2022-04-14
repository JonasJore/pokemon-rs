#![deny(unused_import_braces)]
#![allow(dead_code, non_upper_case_globals)]

extern crate rand;

use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::error::Error;

mod data;
use data::ch::ch;
use data::de::de;
use data::en::en;
use data::fr::fr;
use data::jp::jp;
use data::ru::ru;

const repo_issues: &'static str = "https://github.com/JonasJore/pokemon-rs/issues";

fn supported_languages() -> HashSet<&'static str> {
    return HashSet::from(["en", "jp", "fr", "de", "ru", "ch"]);
}
fn get_translated_list(locale: Option<&str>) -> Result<Vec<&str>, Box<dyn Error>> {
    let language: &str = locale.unwrap_or("en");

    if language == "en" {
        let english_file = en();
        return Ok(english_file);
    }

    if !supported_languages().contains(language) {
        panic!("Translated list for language code {} does not exist. Feel free to post a issue here: {}", language, repo_issues);
    }

    let translated_pokemon_list: Vec<&'static str> = match locale {
        Some("ch") => ch(),
        Some("de") => de(),
        Some("en") => en(),
        Some("fr") => fr(),
        Some("jp") => jp(),
        Some("ru") => ru(),
        //None => todo!(),
        _ => panic!("error"),
    };

    Ok(translated_pokemon_list)
}
fn get_pokemon(locale: Option<&str>) -> Result<Vec<&str>, Box<dyn Error>> {
    get_translated_list(locale)
}
pub fn get_all(locale: Option<&str>) -> Vec<&str> {
    let pokemon_list = get_pokemon(locale);
    pokemon_list.unwrap()
}
pub fn get_by_id(id: usize, locale: Option<&str>) -> String {
    let pokemon_list = get_pokemon(locale).unwrap();
    pokemon_list[id - 1].to_string()
}
pub fn get_id_by_name(name: &str, locale: Option<&str>) -> usize {
    let pokemon_list = get_pokemon(locale).unwrap();
    pokemon_list
        .iter()
        .position(|pokemon| pokemon.to_owned() == name)
        .unwrap()
        + 1
}
pub fn random(locale: Option<&str>) -> String {
    let pokemon_list = get_pokemon(locale).unwrap().to_owned();
    pokemon_list
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_owned()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_get_all() {
        let list = get_all(None);
        assert_eq!(list.len(), 151);
        println!("{:?}", list);
    }
    #[test]
    fn test_non_english_get_all_pokemon_get_bulbasaur() {
        let bulbasaur_jp = get_by_id(1, Some("jp"));
        assert_eq!(bulbasaur_jp, "フシギダネ");
    }
}
