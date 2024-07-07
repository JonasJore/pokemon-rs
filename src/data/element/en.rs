use std::collections::HashMap;

use maplit::hashmap;

pub fn en() -> HashMap<usize, String> {
    return hashmap! {
        1 => "Normal".to_string(),
        2 => "Fire".to_string(),
        3 => "Water".to_string(),
        4 => "Electric".to_string(),
        5 => "Grass".to_string(),
        6 => "Ice".to_string(),
        7 => "Fighting".to_string(),
        8 => "Poison".to_string(),
        9 => "Ground".to_string(),
        10 => "Flying".to_string(),
        11 => "Psychic".to_string(),
        12 => "Bug".to_string(),
        13 => "Rock".to_string(),
        14 => "Ghost".to_string(),
        15 => "Dragon".to_string(),
        16 => "Dark".to_string(),
        17 => "Steel".to_string(),
        18 => "Fairy".to_string(),
    };
}
