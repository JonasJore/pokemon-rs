use crate::data;
use crate::data::region;
use crate::declarations::generation::generation::GenerationExtension;
use crate::declarations::string::string::StringExtension;
use crate::declarations::vector::vector::VectorExtension;
use crate::generation::Generation;
use crate::github::constants::REPO_LINK;
use crate::list;

use rand::prelude::SliceRandom;

pub fn get_complete_generation<'a>(generation: &str, locale: Option<&'a str>) -> Vec<&'a str> {
    let pokemon_list: Vec<&'a str> = list::get_pokemon(locale).unwrap_or_default();
    let mapped_gen: &Generation = generation.map_str_to_generation();
    let valid_pokemon: Vec<&str> = pokemon_list.into_iter().collect();

    mapped_gen.generation_sublist(valid_pokemon)
}

// TODO: cover id being out of bounds, and introduce Option as return, also update tests accordingly
pub fn get_by_id(id: usize, locale: Option<&str>) -> &str {
    let pokemon_list = list::get_pokemon(locale).unwrap_or_default();
    pokemon_list[id - 1]
}

pub fn get_id_by_name(name: &str, locale: Option<&str>) -> usize {
    let pokemon_list = list::get_pokemon(locale).unwrap();
    if !pokemon_list.contains(&name) {
        let list_alternate_locale = match name {
            name if data::ch::ch().contains(&name) => data::ch::ch(),
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
    list::get_pokemon(locale).ok().and_then(|pokemon_list| {
        pokemon_list
            .choose(&mut rand::thread_rng())
            .map(|&chosen_pokemon| chosen_pokemon.to_string())
    })
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
        _ => panic!(
            "Invalid or unsupported locale. PRs welcome at {}",
            REPO_LINK
        ),
    };

    return region_by_locale.get(&id).unwrap().to_string();
}

pub fn get_all_regions() -> Vec<String> {
    (1..=9)
        .map(|region_id| get_region_by_generation(region_id, None))
        .collect::<Vec<String>>()
}
