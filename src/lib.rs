#![deny(
    unused_import_braces,
    unused_imports,
    unused_variables,
    unused_allocation,
    unused_crate_dependencies,
    unused_extern_crates
)]
#![allow(dead_code, non_upper_case_globals)]

mod constants;
mod data;
mod declarations;
mod functions;
mod generation;
mod list;

pub fn get_all(locale: Option<&str>) -> Vec<&str> {
    functions::get_all(locale)
}
pub fn get_by_id(id: usize, locale: Option<&str>) -> String {
    functions::get_by_id(id, locale)
}
pub fn get_id_by_name(name: &str, locale: Option<&str>) -> usize {
    functions::get_id_by_name(name, locale)
}
pub fn random(locale: Option<&str>) -> String {
    functions::random(locale)
}
pub fn get_generation<'a>(generation: &str, locale: Option<&'a str>) -> Vec<&'a str> {
    functions::get_complete_generation(generation, locale)
}
pub fn get_region(generation_number: usize) -> String {
    functions::get_region_by_generation(generation_number)
}

#[cfg(test)]
mod test;
