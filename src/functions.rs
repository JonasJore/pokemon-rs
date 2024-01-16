use crate::constants::REPO_LINK;
use crate::data;
use crate::declarations::generation::generation::GenerationExtension;
use crate::declarations::string::string::StringExtension;
use crate::declarations::vector::vector::VectorExtension;
use crate::generation::Generation;
use crate::list;

use rand::prelude::SliceRandom;

pub fn get_complete_generation<'a>(generation: &str, locale: Option<&'a str>) -> Vec<&'a str> {
    let pokemon_list: Vec<&str> = list::get_pokemon(locale).unwrap();
    let mapped_gen: &Generation = generation.map_str_to_generation();
    let valid_pokemon: Vec<&str> = pokemon_list
        .iter()
        .filter(|&&pokemon| pokemon != "N/A")
        .cloned()
        .collect();

    return mapped_gen.generation_sublist(valid_pokemon);
}

pub fn get_by_id(id: usize, locale: Option<&str>) -> String {
    let pokemon_list = list::get_pokemon(locale).unwrap();
    let pokemon_by_id = pokemon_list[id - 1].to_string();
    if pokemon_by_id == "N/A".to_string() {
        panic!("Not a valid pokemon");
    }

    return pokemon_by_id;
}

pub fn get_id_by_name(name: &str, locale: Option<&str>) -> usize {
    if name == "N/A" {
        panic!("not a valid pokemon");
    }
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
    let pokemon_list = list::get_pokemon(locale)
        .unwrap()
        .iter()
        .filter(|&&pokemon| pokemon != "N/A")
        .cloned()
        .collect();

    return pokemon_list;
}

pub fn get_region_by_generation(id: usize) -> String {
    let gen: Generation = match id {
        1 => Generation::Kanto,
        2 => Generation::Johto,
        3 => Generation::Hoenn,
        4 => Generation::Sinnoh,
        5 => Generation::Unova,
        6 => Generation::Kalos,
        7 => Generation::Alola,
        8 => Generation::Galar,
        9 => Generation::Paldea,
        _ => panic!("Invalid input"),
    };

    return gen.to_string();
}

// TODO: need support for generations in different locales
pub fn get_all_regions() -> Vec<String> {
    let all_regions = (1..=9)
        .map(|x| get_region_by_generation(x))
        .collect::<Vec<String>>();

    return all_regions;
}
