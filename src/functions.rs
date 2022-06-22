use crate::constants::REPO_LINK;
use crate::data;
use crate::declarations::generation::generation::GenerationExtension;
use crate::declarations::string::string::StringExtension;
use crate::declarations::vector::vector::VectorExtension;
use crate::generation::Generation;
use crate::list;
use rand::prelude::SliceRandom;

pub fn get_complete_generation<'get_gen_lifetime>(
    generation: &str,
    locale: Option<&'get_gen_lifetime str>,
) -> Vec<&'get_gen_lifetime str> {
    let pokemon_list: Vec<&str> = list::get_pokemon(locale).unwrap();
    let mapped_gen: &Generation = generation.map_str_to_generation();

    mapped_gen.generation_sublist(pokemon_list)
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
            _ => panic!("The pokémon given does not seem to have been added to the list yet, PRs welcome at {}", REPO_LINK)
        };

        return list_alternate_locale.get_id(name);
    }

    return pokemon_list.get_id(name);
}

pub fn random(locale: Option<&str>) -> String {
    let pokemon_list = list::get_pokemon(locale).unwrap().to_owned();
    pokemon_list
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_owned()
        .to_string()
}
