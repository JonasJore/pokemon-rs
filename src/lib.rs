#![deny(
    unused_import_braces,
    unused_imports,
    unused_variables,
    unused_allocation,
    unused_crate_dependencies,
    unused_extern_crates
)]
#![allow(dead_code, non_upper_case_globals)]

mod github {
    pub mod constants;
}
mod data;
mod declarations;
mod functions;
mod generation;
mod list;

/// Returns a list of all Pokémons from all generations
///
/// # Arguments
/// `locale` - `Option<&str>` that holds the language  
/// you would like the Pokémon names should be translated in
///
/// # Example
/// ```rs
/// // default language is english so providing None is the same as providing `Some("en")`
/// pokemon_rs::get_all(None);
/// pokemon_rs::get_all(Some("jp"));
/// ```
pub fn get_all(locale: Option<&str>) -> Vec<&str> {
    functions::get_all(locale)
}
/// Returns the Pokémon as a &str that corresponds given id
///
/// # Arguments
/// `id` - `usize` that represents the official Pokémon id, used find the correct Pokémon
/// `locale` - `Option<&str>` that holds the language
///
/// # Example
/// ```rs
/// // default language is english so providing None is the same as providing `Some("en")`
/// pokemon_rs::get_by_id(1, Some("jp"));
/// pokemon_rs::get_by_id(24, None);
/// ```
pub fn get_by_id(id: usize, locale: Option<&str>) -> &str {
    functions::get_by_id(id, locale)
}
/// Returns the Pokémon's id that corresponds to a given id
///
/// # Arguments
///
/// `name` - `&str` that represents the Pokémon you'd like to lookup id for.  
/// `locale` - `Option<&str>` that holds the language you are searching for Pokémon id in.
///
/// # Example
/// ```rs
/// // default language is english so providing None is the same as providing `Some("en")`
/// pokemon_rs::get_id_by_name("Pikachu", None);
/// pokemon_rs::get_id_by_name("フシギダネ", Some("jp"));
/// ```
///
pub fn get_id_by_name(name: &str, locale: Option<&str>) -> usize {
    functions::get_id_by_name(name, locale)
}
/// Returns a random Pokémon out of all Pokémon released to this day
///
/// # Arguments
///
/// `locale` - `Option<&str>` that holds the language
///
/// # Example
/// ```rs
/// // default language is english so providing None is the same as providing `Some("en")`
/// pokemon_rs::random(None);
/// pokemon_rs::random(Some("jp"));
/// ```
pub fn random(locale: Option<&str>) -> String {
    functions::random(locale).unwrap()
}
/// Returns a whole generation as a sorted `Vector<&str>` based on its given region name
///
/// # Arguments
///
/// `generation` - `&str` that represents the generation (or region) name you would like to get.  
/// `locale` - `Option<&str>` that represents which language you would like the Pokémon names in returned generation in.
///
/// # Example
/// ```rs
/// // default language is english so providing None is the same as providing `Some("en")`
/// pokemon_rs::get_generation("Kanto", None);
/// pokemon_rs::get_generation("Kanto", Some("jp"));
/// ```
pub fn get_generation<'a>(generation: &str, locale: Option<&'a str>) -> Vec<&'a str> {
    functions::get_complete_generation(generation, locale)
}
/// Returns region name from generation number in released order.  
/// Kanto = 1, Johto = 2, Hoenn = 3 ...
///
/// # Arguments
///
/// `generation_number` - `usize`
///
/// # Example
/// ```rs
/// pokemon_rs::get_region(1)
/// let paldea: String = pokemon_rs::get_region(9)
/// ```
pub fn get_region(generation_number: usize) -> String {
    functions::get_region_by_generation(generation_number)
}
/// Returns all region names
///
/// # Example
/// ```rs
/// let all_regions: Vec<String> = pokemon_rs::get_all_regions();
/// ```
pub fn get_all_regions() -> Vec<String> {
    functions::get_all_regions()
}

#[cfg(test)]
mod test;
