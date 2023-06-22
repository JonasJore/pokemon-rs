use crate::{get_all, get_generation, get_region};

fn type_to_string<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

const TOTAL_NUMBER_OF_POKEMON: usize = 1008;

#[test]
fn test_get_all() {
    let list = get_all(None);
    assert_eq!(list.len(), TOTAL_NUMBER_OF_POKEMON);
}
#[test]
fn test_get_all_first_gen_pokemon_as_vector() {
    let kanto: Vec<&str> = get_generation("Kanto", Some("en"));
    assert_eq!(kanto.len(), 151);
    assert_eq!(type_to_string(&kanto), "alloc::vec::Vec<&str>");
}
#[test]
fn test_get_all_second_gen_pokemon_as_vector() {
    let johto: Vec<&str> = get_generation("Johto", Some("en"));
    assert_eq!(johto.len(), 100);
    assert_eq!(type_to_string(&johto), "alloc::vec::Vec<&str>");
}
#[test]
fn test_get_all_third_gen_pokemon_as_vector() {
    let hoenn: Vec<&str> = get_generation("Hoenn", Some("en"));
    assert_eq!(hoenn.len(), 135);
    assert_eq!(type_to_string(&hoenn), "alloc::vec::Vec<&str>");
}
#[test]
fn test_get_all_fourth_gen_pokemon_as_vector() {
    let sinnoh: Vec<&str> = get_generation("Sinnoh", Some("en"));
    assert_eq!(sinnoh.len(), 107);
    assert_eq!(type_to_string(&sinnoh), "alloc::vec::Vec<&str>");
}
#[test]
fn test_get_all_fifth_gen_pokemon_as_vector() {
    let unova: Vec<&str> = get_generation("Unova", Some("en"));
    println!("{:?}", unova);
    assert_eq!(unova.len(), 156);
    assert_eq!(type_to_string(&unova), "alloc::vec::Vec<&str>");
}
#[test]
fn test_get_all_sixth_gen_pokemon_as_vector() {
    let kalos: Vec<&str> = get_generation("Kalos", Some("en"));
    assert_eq!(kalos.len(), 72);
    assert_eq!(type_to_string(&kalos), "alloc::vec::Vec<&str>");
}
#[test]
fn test_get_all_seventh_gen_pokemon_as_vector() {
    let alola: Vec<&str> = get_generation("Alola", Some("en"));
    assert_eq!(alola.len(), 88);
    assert_eq!(type_to_string(&alola), "alloc::vec::Vec<&str>");
}
#[test]
fn test_get_all_eight_gen_pokemon_as_vector() {
    let galar: Vec<&str> = get_generation("Galar", Some("en"));
    assert_eq!(galar.len(), 96);
    assert_eq!(type_to_string(&galar), "alloc::vec::Vec<&str>");
}
#[test]
fn test_get_all_ninth_gen_pokemon_as_vector() {
    let paldea: Vec<&str> = get_generation("Paldea", Some("en"));
    assert_eq!(paldea.len(), 103);
    assert_eq!(type_to_string(&paldea), "alloc::vec::Vec<&str>");
}
#[test]
fn test_gen_2_should_get_johto_as_string() {
    let johto = get_region(2);
    assert_eq!("Johto", johto);
}
#[test]
fn test_verify_all_nine_regions_works() {
    let mut vec: Vec<String> = vec![];
    let range = 1..10;
    let start = range.start as usize;
    let end = range.end as usize;
    let all_regions = (start..end).map(|f| vec.push(get_region(f)));
    assert_eq!(9, all_regions.len());
}
