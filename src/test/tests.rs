#[deny(unused_imports)]
#[cfg(test)]
pub mod test {
    use crate::{get_all, get_by_id, get_id_by_name};
    #[test]
    fn test_get_all() {
        let list = get_all(None);
        assert_eq!(list.len(), 251);
    }
    #[test]
    fn test_non_english_get_all_pokemon_get_bulbasaur() {
        let bulbasaur_jp = get_by_id(1, Some("jp"));
        assert_eq!(bulbasaur_jp, "フシギダネ");
    }
    #[test]
    fn chikorita_test_default_locale() {
        let chikorita = get_by_id(152, None);
        assert_eq!(chikorita, "Chikorita");
    }
    #[test]
    fn test_crossing_locale_and_given_pokemon_look_for_alternate_vector_for_finding_the_pokemon() {
        let id = get_id_by_name("フシギダネ", Some("en"));
        assert_eq!(id, 1);
    }
    #[test]
    fn test_crossing_locale_and_given_pokemon_en_in_russian_locale() {
        let id = get_id_by_name("Blastoise", Some("ru"));
        assert_eq!(id, 9);
    }
    #[cfg(test)]
    mod panic_tests {
        use crate::{get_all, get_by_id, get_id_by_name, random};

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
    }
}
