#[deny(unused_imports)]
#[cfg(test)]
pub mod test {
    use crate::{
        get_all, get_by_id, get_generation, get_id_by_name, get_region,
    };

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
    #[test]
    fn test_gen_2_support_english() {
        let lugia = get_by_id(249, None);
        assert_eq!(lugia, "Lugia");
    }
    #[test]
    fn test_gen_2_support_japanese() {
        let lugia = get_by_id(249, Some("jp"));
        assert_eq!(lugia, "ルギア");
    }
    #[test]
    fn test_gen_2_support_french() {
        let igglybuff = get_by_id(174, Some("fr"));
        assert_eq!(igglybuff, "Toudoudou");
    }
    #[test]
    fn test_gen_2_support_german() {
        let tyrannitar = get_by_id(248, Some("de"));
        assert_eq!(tyrannitar, "Despotar");
    }
    #[test]
    fn test_gen_2_support_chinese() {
        let raikou = get_by_id(243, Some("ch"));
        assert_eq!(raikou, "雷公");
    }
    #[test]
    fn test_gen_2_support_russian() {
        let typlosion = get_by_id(157, Some("ru"));
        assert_eq!(typlosion, "Тайфложн");
    }
    #[test]
    fn test_gen_3_support_english() {
        let deoxys = get_by_id(386, None);
        assert_eq!(deoxys, "Deoxys");
    }
    #[test]
    fn test_gen_3_support_chinese() {
        let rayquaza = get_by_id(384, Some("ch"));
        assert_eq!(rayquaza, "烈空坐");
    }
    #[test]
    fn test_gen_3_support_german() {
        let marshtomp = get_by_id(259, Some("de"));
        assert_eq!(marshtomp, "Moorabbel");
    }
    #[test]
    fn test_gen_3_support_french() {
        let jirachi = get_by_id(385, Some("fr"));
        assert_eq!(jirachi, "Jirachi");
    }
    #[test]
    fn test_gen_3_support_japanese() {
        let latias = get_by_id(380, Some("jp"));
        assert_eq!(latias, "ラティアス");
    }
    #[test]
    fn test_gen_3_support_russian() {
        let regigas = get_by_id(379, Some("ru"));
        assert_eq!(regigas, "Реджистил");
    }
    #[test]
    fn test_gen_4_support_english() {
        let arceus = get_by_id(493, None);
        assert_eq!(arceus, "Arceus");
    }
    #[test]
    fn test_gen_4_support_japanese() {
        let garchomp = get_by_id(445, Some("jp"));
        assert_eq!(garchomp, "ガブリアス");
    }
    #[test]
    fn test_gen_4_support_chinese() {
        let creselia = get_by_id(488, Some("ch"));
        assert_eq!(creselia, "克雷色利亞");
    }
    #[test]
    fn test_gen_4_support_german() {
        let shaymin = get_by_id(492, Some("de"));
        assert_eq!(shaymin, "Shaymin");
    }
    #[test]
    fn test_gen_4_support_french() {
        let rhinastoc = get_by_id(464, Some("fr"));
        assert_eq!(rhinastoc, "Rhinastoc");
        let rhinastock_id_by_name = get_id_by_name("Rhinastoc", Some("fr"));
        assert_eq!(464, rhinastock_id_by_name);
    }
    #[test]
    fn test_gen_5_support_english() {
        let trubbish = get_by_id(568, None);
        assert_eq!(trubbish, "Trubbish");
    }
    #[test]
    fn test_gen_5_support_japanese() {
        let victini = get_by_id(494, Some("jp"));
        assert_eq!(victini, "ビクティニ");
    }
    #[test]
    fn test_gen_5_support_chinese() {
        let victini = get_by_id(494, Some("ch"));
        assert_eq!(victini, "比克提尼");
    }
    #[test]
    fn test_gen_5_support_german() {
        let skallyk = get_by_id(629, Some("de"));
        assert_eq!(skallyk, "Skallyk");
        let skallyk_id_by_name = get_id_by_name("Skallyk", Some("de"));
        assert_eq!(629, skallyk_id_by_name);
    }
    #[test]
    fn test_gen_6_support_english() {
        let yveltal = get_by_id(717, None);
        assert_eq!(yveltal, "Yveltal");
    }
    #[test]
    fn test_gen_6_support_japanese() {
        let xerneas = get_by_id(716, Some("jp"));
        assert_eq!(xerneas, "ゼルネアス");
    }
    #[test]
    fn test_gen_6_support_chinese() {
        let chespin = get_by_id(650, Some("ch"));
        assert_eq!(chespin, "哈力栗");
    }
    #[test]
    fn test_gen_6_support_german() {
        let viscora = get_by_id(704, Some("de"));
        assert_eq!(viscora, "Viscora");
        let viscora_by_name = get_id_by_name("Viscora", Some("de"));
        assert_eq!(viscora_by_name, 704);
    }
    #[test]
    fn test_gen_7_support_english() {
        let melmetal = get_by_id(809, None);
        assert_eq!(melmetal, "Melmetal");
    }
    #[test]
    fn test_gen_7_support_japanese() {
        let torracat = get_by_id(726, Some("jp"));
        assert_eq!(torracat, "ニャヒート");
    }
    #[test]
    fn test_gen_7_support_chinese() {
        let solgaleo = get_by_id(791, Some("ch"));
        assert_eq!(solgaleo, "索爾迦雷歐");
    }
    #[test]
    fn test_gen_7_support_german() {
        let agoyon = get_by_id(804, Some("de"));
        assert_eq!(agoyon, "Agoyon");
        let agoyon_id_by_name = get_id_by_name("Agoyon", Some("de"));
        assert_eq!(804, agoyon_id_by_name);
    }
    #[test]
    fn test_gen_8_support_english() {
        let calyrex = get_by_id(898, None);
        assert_eq!(calyrex, "Calyrex");
    }
    #[test]
    fn test_gen_8_support_japanese() {
        let sandaconda = get_by_id(844, Some("jp"));
        assert_eq!(sandaconda, "サダイジャ");
    }
    #[test]
    fn test_gen_8_support_chinese() {
        let regieleki = get_by_id(894, Some("ch"));
        assert_eq!(regieleki, "雷吉艾勒奇");
    }
    #[test]
    fn test_gen_8_support_german() {
        let chimstix = get_by_id(811, Some("de"));
        assert_eq!(chimstix, "Chimstix");
        let chimstix_id_by_name = get_id_by_name("Chimstix", Some("de"));
        assert_eq!(811, chimstix_id_by_name);
    }
    #[test]
    fn test_gen_9_support_english() {
        let sprigatito = get_by_id(906, None);
        assert_eq!(sprigatito, "Sprigatito");
        let sprigatito_id_by_name = get_id_by_name("Sprigatito", None);
        assert_eq!(906, sprigatito_id_by_name);
    }
    #[test]
    fn test_gen_9_support_german() {
        let eisenkrieger = get_by_id(1006, Some("de"));
        assert_eq!("Eisenkrieger", eisenkrieger);
        let eisenkrieger_id_by_name = get_id_by_name("Eisenkrieger", Some("de"));
        assert_eq!(1006, eisenkrieger_id_by_name);
    }
    #[test]
    fn test_gen_2_should_get_Johto_as_string() {
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
    #[cfg(test)]
    mod panic_test_generations {
        use crate::get_generation;
        #[test]
        #[should_panic]
        fn test_get_undefined_generation() {
            let undefined_gen = get_generation("Awesome Generation", Some("en"));
            assert_eq!(undefined_gen.is_empty(), false);
        }
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
    #[should_panic]
    fn test_asking_for_non_valid_pokemon_will_panic() {
        get_id_by_name("N/A", Some("en"));
    }
    #[test]
    #[should_panic]
    fn test_invalid_id_for_generation_should_throw_panic() {
        get_region(0);
    }
    #[test]
    #[should_panic]
    fn test_really_high_id_for_generation_should_throw_panic() {
        get_region(100);
    }
}
