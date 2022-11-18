fn generation_nine_early_days_warning() {
    println!("This is early days for gen 9. Lists will change as more information becomes available");
    println!("Some names for generation 9 Pokemon, may be incorrect due to inaccurate translation or incorrect name");
}

pub mod string {
    use crate::generation::Generation;
    use super::generation_nine_early_days_warning;

    pub trait StringExtension {
        fn map_str_to_generation<'l>(self) -> &'l Generation;
    }

    impl StringExtension for &str {
        fn map_str_to_generation<'l>(self) -> &'l Generation {
            match self {
                "Kanto" => &Generation::Kanto,
                "Johto" => &Generation::Johto,
                "Hoenn" => &Generation::Hoenn,
                "Sinnoh" => &Generation::Sinnoh,
                "Unova" => &Generation::Unova,
                "Kalos" => &Generation::Kalos,
                "Alola" => &Generation::Alola,
                "Galar" => &Generation::Galar,
                "Paldea" => {
                    generation_nine_early_days_warning();
                    return &Generation::Paldea;
                },
                _ => panic!(""),
            }
        }
    }
}
