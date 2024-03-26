use crate::{get_all, get_by_id, get_generation, get_id_by_name, get_region, random};

#[test]
#[should_panic]
fn get_all_should_panic_when_non_supported_language_is_given() {
    get_all(Some("none-supported locale"));
}
#[test]
#[should_panic]
fn get_by_id_should_panic_when_non_supported_language_is_given() {
    get_by_id(1, Some("none-supported locale"));
}
#[test]
#[should_panic]
fn get_id_by_name_should_panic_when_non_supported_language_is_given() {
    get_id_by_name("Mew", Some("none-supported locale"));
}
#[test]
#[should_panic]
fn random_should_panic_when_non_supported_language_is_given() {
    random(Some("none-supported locale"));
}
#[test]
#[should_panic]
fn test_get_undefined_generation() {
    let undefined_gen = get_generation("Awesome Generation", Some("en"));
    assert_eq!(undefined_gen.is_empty(), false);
}
#[test]
#[should_panic]
fn test_really_high_id_for_generation_should_throw_panic() {
    get_region(100, None);
}
#[test]
#[should_panic]
fn test_invalid_id_for_generation_should_throw_panic() {
    get_region(0, None);
}
