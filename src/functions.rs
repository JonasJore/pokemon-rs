use crate::constants::repo_link;
use crate::data;
use crate::generation::Generation;
use crate::list;
use rand::prelude::SliceRandom;

/// TODO: denne sjekker ikke for alle generasjoner ennå...
fn map_str_to_generation(generation_name: &str) -> Generation {
    match generation_name {
        "Kanto" => Generation::Kanto,
        _ => panic!(""),
    }
}

pub fn get_complete_generation<'get_gen_lifetime>(
    generation: &str,
    locale: Option<&'get_gen_lifetime str>,
) -> Vec<&'get_gen_lifetime str> {
    let pokemon_list: Vec<&str> = list::get_pokemon(locale).unwrap();
    let mapped_gen: Generation = map_str_to_generation(generation);

    match mapped_gen {
        Generation::Kanto => pokemon_list[0..=151].to_vec(),
    }
}

pub fn get_by_id(id: usize, locale: Option<&str>) -> String {
    let pokemon_list = list::get_pokemon(locale).unwrap();
    pokemon_list[id - 1].to_string()
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
            _ => panic!("The pokémon given does not seem to have been added to the list yet, PRs welcome at {}", repo_link)
        };

        return list_alternate_locale
            .iter()
            .position(|pokemon| pokemon.to_owned() == name)
            .unwrap()
            + 1;
    }

    pokemon_list
        .iter()
        .position(|pokemon| pokemon.to_owned() == name)
        .unwrap()
        + 1
}

pub fn random(locale: Option<&str>) -> String {
    let pokemon_list = list::get_pokemon(locale).unwrap().to_owned();
    pokemon_list
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_owned()
        .to_string()
}
