use crate::data::element;
use crate::data::region;
use crate::data::{self};
use crate::declarations::generation::generation::GenerationExtension;
use crate::declarations::string::string::StringExtension;
use crate::declarations::vector::vector::VectorExtension;
use crate::generation::Generation;
use crate::github::constants::REPO_LINK;
use crate::list;

use rand::prelude::SliceRandom;

pub fn get_complete_generation<'a>(generation: &str, locale: Option<&'a str>) -> Vec<&'a str> {
    let pokemon_list: Vec<&'a str> = list::get_pokemon(locale).unwrap();
    let mapped_gen: &Generation = generation.map_str_to_generation();
    let valid_pokemon: Vec<&str> = pokemon_list.into_iter().collect();

    mapped_gen.generation_sublist(valid_pokemon)
}

// TODO: cover id being out of bounds
pub fn get_by_id(id: usize, locale: Option<&str>) -> Option<&str> {
    let pokemon_list = list::get_pokemon(locale).unwrap();
    return Some(pokemon_list[id - 1]);
}

pub fn get_id_by_name(name: &str, locale: Option<&str>) -> usize {
    let pokemon_list = list::get_pokemon(locale).unwrap();
    if !pokemon_list.contains(&name) {
        let list_alternate_locale = match name {
            name if data::cn::cn().contains(&name) => data::cn::cn(),
            name if data::de::de().contains(&name) => data::de::de(),
            name if data::en::en().contains(&name) => data::en::en(),
            name if data::fr::fr().contains(&name) => data::fr::fr(),
            name if data::jp::jp().contains(&name) => data::jp::jp(),
            name if data::ru::ru().contains(&name) => data::ru::ru(),
            _ => panic!("The pokémon given does not seem to have been added to the list yet, PRs welcome at {}", REPO_LINK)
        };

        return list_alternate_locale.get_id(name);
    }

    return pokemon_list.get_id(name);
}

pub fn random(locale: Option<&str>) -> Option<String> {
    let mut rng = rand::thread_rng();
    let pokemons: Vec<String> = match list::get_pokemon(locale) {
        Some(pokemon_list) => pokemon_list.into_iter().map(ToString::to_string).collect(),
        None => return None,
    };

    pokemons
        .choose(&mut rng)
        .map(|random_item| random_item.clone())
}

pub fn get_all(locale: Option<&str>) -> Vec<&str> {
    list::get_pokemon(locale)
        .unwrap_or_default()
        .iter()
        .cloned()
        .collect()
}

pub fn get_region_by_generation(id: usize, locale: Option<&str>) -> String {
    let region_by_locale = match locale.unwrap_or("en") {
        "en" => region::en::en(),
        "de" => region::de::de(),
        "fr" => region::fr::fr(),
        "cn" => region::cn::cn(),
        "jp" => region::jp::jp(),
        "ru" => region::ru::ru(),
        _ => panic!(
            "Invalid or unsupported locale. PRs welcome at {}",
            REPO_LINK
        ),
    };

    return region_by_locale.get(&id).unwrap().to_string();
}

pub fn get_all_regions(locale: Option<&str>) -> Vec<String> {
    return (1..=9)
        .map(|region_id| get_region_by_generation(region_id, locale))
        .collect::<Vec<String>>();
}

pub fn get_all_types(locale: Option<&str>) -> Vec<String> {
    let types_by_locale = match locale.unwrap_or("en") {
        "en" => element::en::en(),
        _ => panic!(
            "Invalid or unsupported locale. PRs welcome at {}",
            REPO_LINK
        ),
    };

    let types = (1..=types_by_locale.len())
        .map(|element_id| types_by_locale[&(element_id)].clone())
        .collect::<Vec<String>>();

    return types;
}
