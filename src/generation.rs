pub enum Generation {
    Kanto,
    Johto,
    Hoenn,
    Sinnoh,
    Unova,
    Kalos,
    Alola,
    Galar,
    Paldea,
}

impl std::fmt::Display for Generation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let gen_str = match self {
            Generation::Kanto => "Kanto",
            Generation::Johto => "Johto",
            Generation::Hoenn => "Hoenn",
            Generation::Sinnoh => "Sinnoh",
            Generation::Unova => "Unova",
            Generation::Kalos => "Kalos",
            Generation::Alola => "Alola",
            Generation::Galar => "Galar",
            Generation::Paldea => "Paldea",
        };
        write!(f, "{}", gen_str)
    }
}
