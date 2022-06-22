pub mod generation {
    use crate::generation::Generation;

    pub trait GenerationExtension {
        fn generation_sublist<'a>(&'a self, list: Vec<&'a str>) -> Vec<&str>;
    }

    impl GenerationExtension for Generation {
        fn generation_sublist<'a>(&'a self, list: Vec<&'a str>) -> Vec<&str> {
            match self {
                Generation::Kanto => list[0..=151].to_vec(),
                Generation::Johto => list[151..=250].to_vec(),
                Generation::Hoenn => list[251..=385].to_vec(),
                Generation::Sinnoh => list[386..=492].to_vec(),
                Generation::Unova => list[493..=648].to_vec(),
                Generation::Kalos => list[649..=720].to_vec(),
                Generation::Alola => list[721..=808].to_vec(),
                Generation::Galar => list[809..=897].to_vec()
            }
        }
    }
}
