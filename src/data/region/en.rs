use std::collections::HashMap;

use maplit::hashmap;

pub fn en() -> HashMap<usize, String> {
    return hashmap! {
        1 => "Kanto".to_string(),
        2 => "Johto".to_string(),
        3 => "Hoenn".to_string(),
        4 => "Sinnoh".to_string(),
        5 => "Unova".to_string(),
        6 => "Kalos".to_string(),
        7 => "Alola".to_string(),
        8 => "Galar".to_string(),
        9 => "Paldea".to_string(),
    };
}
