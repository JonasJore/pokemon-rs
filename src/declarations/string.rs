pub mod string {
    use crate::generation::Generation;

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
                _ => panic!(""),
            }
        }
    }
}
