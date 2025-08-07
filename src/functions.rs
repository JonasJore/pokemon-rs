use std::path::PathBuf;

use crate::data::element;
use crate::data::region;
use crate::data::{self};
use crate::declarations::generation::generation::GenerationExtension;
use crate::declarations::string::string::StringExtension;
use crate::declarations::vector::vector::VectorExtension;
use crate::generation::Generation;
use crate::list;
use crate::util::panic_by_reason::panic_handling::{get_panic_by_reason, PanicReason};

// TODO: can i make this myself to reduce external deps?
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
        // TODO: too much code repetition. fix!
        let list_alternate_locale = match name {
            name if data::cn::cn().contains(&name) => data::cn::cn(),
            name if data::de::de().contains(&name) => data::de::de(),
            name if data::en::en().contains(&name) => data::en::en(),
            name if data::fr::fr().contains(&name) => data::fr::fr(),
            name if data::jp::jp().contains(&name) => data::jp::jp(),
            name if data::ru::ru().contains(&name) => data::ru::ru(),
            _ => get_panic_by_reason(PanicReason::UnsupportedPokemon),
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
        _ => get_panic_by_reason(PanicReason::UnsupportedLanguage),
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
        "zh_cn" => element::zh_cn::zh(),
        _ => get_panic_by_reason(PanicReason::UnsupportedLanguage),
    };

    let types = (1..=types_by_locale.len())
        .map(|element_id| types_by_locale[&(element_id)].clone())
        .collect::<Vec<String>>();

    return types;
}

pub fn get_type_by_id(id: usize, locale: Option<&str>) -> String {
    let types = get_all_types(locale);
    return types.get(id - 1).unwrap().to_owned();
}

fn get_sprite_from_path(name: &str) -> Result<String, std::io::Error> {
    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // TODO: make this path more generic when supporting more generations
    let path = format!("src/data/sprites/pokemon/kanto/{}", name);
    file_path.push(path);
    let file_content = std::fs::read_to_string(&file_path);
    match file_content {
        Ok(raw_file_content) => {
            let file_content_parsed = raw_file_content.replace("\\e", "\x1b");
            Ok(file_content_parsed)
        }
        Err(_) => get_panic_by_reason(PanicReason::UnsupportedPokemonSprite),
    }
}

pub fn get_sprite_by_name(name: &str) -> Result<String, std::io::Error> {
    get_sprite_from_path(name)
}

pub fn get_sprite_by_id(id: usize) -> Result<String, std::io::Error> {
    let name_by_id = get_by_id(id, None);
    get_sprite_from_path(name_by_id.unwrap())
}
