use crate::constants;

fn generation_nine_early_days_warning() {
    println!(
        "This is early days for gen 9. Lists will change as more information becomes available"
    );
    println!("Some names for gen 9 Pokemon, may be incorrect due to inaccurate translation or incorrect name");
    println!(
        "Feel free to file an issue at {} if something happens to be incorrect",
        constants::REPO_ISSUES
    );
}

pub mod string {
    use super::generation_nine_early_days_warning;
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
                "Paldea" => {
                    generation_nine_early_days_warning();
                    return &Generation::Paldea;
                }
                _ => panic!(""),
            }
        }
    }
}
