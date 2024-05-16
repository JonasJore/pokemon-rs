use maplit::hashmap;
use std::collections::HashMap;

pub fn jp() -> HashMap<usize, String> {
    return hashmap! {
        1 => "カントー".to_string(),
        2 => "ジョウト".to_string(),
        3 => "ホウエン".to_string(),
        4 => "シンオウ".to_string(),
        5 => "イッシュ".to_string(),
        6 => "カロス".to_string(),
        7 => "アローラ".to_string(),
        8 => "ガラル".to_string(),
        9 => "パルデア".to_string(),
    };
}
