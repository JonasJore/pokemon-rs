use maplit::hashmap;
use std::collections::HashMap;

pub fn ch() -> HashMap<usize, String> {
    return hashmap! {
        1 => "关都".to_string(),
        2 => "城都".to_string(),
        3 => "丰缘".to_string(),
        4 => "神奥".to_string(),
        5 => "合众".to_string(),
        6 => "卡洛斯".to_string(),
        7 => "阿罗拉".to_string(),
        8 => "伽勒尔".to_string(),
        9 => "帕尔迪亚".to_string(),
    };
}
