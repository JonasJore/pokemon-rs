use maplit::hashmap;
use std::collections::HashMap;

pub fn zh() -> HashMap<usize, String> {
    return hashmap! {
        1 => "普通".to_string(),
        2 => "火".to_string(),
        3 => "水".to_string(),
        4 => "电".to_string(),
        5 => "草".to_string(),
        6 => "冰".to_string(),
        7 => "格斗".to_string(),
        8 => "毒".to_string(),
        9 => "地面".to_string(),
        10 => "飞行".to_string(),
        11 => "超能力".to_string(),
        12 => "虫".to_string(),
        13 => "岩石".to_string(),
        14 => "幽灵".to_string(),
        15 => "龙".to_string(),
        16 => "恶".to_string(),
        17 => "钢".to_string(),
        18 => "妖精".to_string(),
    };
}
