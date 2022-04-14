#![deny(unused_import_braces)]
#![allow(dead_code, non_upper_case_globals)]

extern crate rand;
extern crate serde_json;

pub mod pokemon {
    use rand::seq::SliceRandom;
    use std::collections::HashSet;
    use std::error::Error;
    use std::fs::File;

    const english: &'static str = "src/data/en.json";
    const repo_issues: &'static str = "https://github.com/JonasJore/pokemon-rs/issues";

    fn supported_languages() -> HashSet<&'static str> {
        return HashSet::from(["en", "jp", "fr", "de", "ru", "ch"]);
    }
    fn get_translated_list(locale: Option<&str>) -> Result<Vec<String>, Box<dyn Error>> {
        let language: &str = locale.unwrap_or("en");

        if language == "en" {
            let english_file: File = File::open(english)?;
            return Ok(serde_json::from_reader(&english_file)?);
        }

        if !supported_languages().contains(language) {
            panic!("Translated list for language code {} does not exit. Feel free to post a issue here: {}", language, repo_issues);
        }

        let file: File = File::open(format!("src/data/{}.json", language))?;
        let translated_pokemon_list: Vec<String> = serde_json::from_reader(&file)?;

        Ok(translated_pokemon_list)
    }
    fn get_pokemon(locale: Option<&str>) -> Result<Vec<String>, Box<dyn Error>> {
        get_translated_list(locale)
    }
    pub fn get_all(locale: Option<&str>) -> Vec<String> {
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
            .position(|pokemon| pokemon == name)
            .unwrap()
            + 1
    }
    pub fn random(locale: Option<&str>) -> String {
        let pokemon_list = get_pokemon(locale).unwrap();
        pokemon_list
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_owned()
    }
}
