use std::collections::HashMap;

pub fn en() -> HashMap<i32, String> {
    return [
        (1, "Kanto"),
        (2, "Johto"),
        (3, "Hoenn"),
        (4, "Sinnoh"),
        (5, "Unova"),
        (6, "Kalos"),
        (7, "Alola"),
        (8, "Galar"),
        (9, "Paldea"),
    ]
    .iter()
    .cloned()
    .collect();
}
