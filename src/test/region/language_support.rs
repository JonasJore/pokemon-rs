use crate::get_region;

#[test]
fn kanto_english_support() {
    let kanto = get_region(1, Some("en"));
    assert_eq!(kanto, "Kanto")
}
#[test]
fn johto_english_support() {
    let johto = get_region(2, Some("en"));
    assert_eq!(johto, "Johto")
}
#[test]
fn hoenn_english_support() {
    let hoenn = get_region(3, Some("en"));
    assert_eq!(hoenn, "Hoenn")
}
#[test]
fn sinnoh_english_support() {
    let sinnoh = get_region(4, Some("en"));
    assert_eq!(sinnoh, "Sinnoh")
}
#[test]
fn unova_english_support() {
    let unova = get_region(5, Some("en"));
    assert_eq!(unova, "Unova")
}
#[test]
fn kalos_english_support() {
    let kalos = get_region(6, Some("en"));
    assert_eq!(kalos, "Kalos")
}
#[test]
fn alola_english_support() {
    let alola = get_region(7, Some("en"));
    assert_eq!(alola, "Alola")
}
#[test]
fn galar_english_support() {
    let galar = get_region(8, Some("en"));
    assert_eq!(galar, "Galar")
}
#[test]
fn paldea_english_support() {
    let paldea = get_region(9, Some("en"));
    assert_eq!(paldea, "Paldea")
}
#[test]
fn kanto_german_support() {
    let kanto = get_region(1, Some("de"));
    assert_eq!(kanto, "Kanto")
}
#[test]
fn johto_german_support() {
    let johto = get_region(2, Some("de"));
    assert_eq!(johto, "Johto")
}
#[test]
fn hoenn_german_support() {
    let hoenn = get_region(3, Some("de"));
    assert_eq!(hoenn, "Hoenn")
}
#[test]
fn sinnoh_german_support() {
    let sinnoh = get_region(4, Some("de"));
    assert_eq!(sinnoh, "Sinnoh")
}
#[test]
fn einall_german_support() {
    let einall = get_region(5, Some("de"));
    assert_eq!(einall, "Einall")
}
#[test]
fn kalos_german_support() {
    let kalos = get_region(6, Some("de"));
    assert_eq!(kalos, "Kalos")
}
#[test]
fn alola_german_support() {
    let alola = get_region(7, Some("de"));
    assert_eq!(alola, "Alola")
}
#[test]
fn galar_german_support() {
    let galar = get_region(8, Some("de"));
    assert_eq!(galar, "Galar")
}
#[test]
fn paldea_german_support() {
    let paldea = get_region(9, Some("de"));
    assert_eq!(paldea, "Paldea")
}
#[test]
fn kanto_french_support() {
    let kanto = get_region(1, Some("fr"));
    assert_eq!(kanto, "Kanto")
}
#[test]
fn johto_french_support() {
    let johto = get_region(2, Some("fr"));
    assert_eq!(johto, "Johto")
}
#[test]
fn hoenn_french_support() {
    let hoenn = get_region(3, Some("fr"));
    assert_eq!(hoenn, "Hoenn")
}
#[test]
fn sinnoh_french_support() {
    let sinnoh = get_region(4, Some("fr"));
    assert_eq!(sinnoh, "Sinnoh")
}
#[test]
fn unys_french_support() {
    let unys = get_region(5, Some("fr"));
    assert_eq!(unys, "Unys")
}
#[test]
fn kalos_french_support() {
    let kalos = get_region(6, Some("fr"));
    assert_eq!(kalos, "Kalos")
}
#[test]
fn alola_french_support() {
    let alola = get_region(7, Some("fr"));
    assert_eq!(alola, "Alola")
}
#[test]
fn galar_french_support() {
    let galar = get_region(8, Some("fr"));
    assert_eq!(galar, "Galar")
}
#[test]
fn paldea_french_support() {
    let paldea = get_region(9, Some("fr"));
    assert_eq!(paldea, "Paldea")
}
#[test]
fn kanto_chinese_support() {
    let kanto = get_region(1, Some("ch"));
    assert_eq!(kanto, "关都")
}
#[test]
fn johto_chinese_support() {
    let johto = get_region(2, Some("ch"));
    assert_eq!(johto, "城都")
}
#[test]
fn hoenn_chinese_support() {
    let hoenn = get_region(3, Some("ch"));
    assert_eq!(hoenn, "丰缘")
}
#[test]
fn sinnoh_chinese_support() {
    let sinnoh = get_region(4, Some("ch"));
    assert_eq!(sinnoh, "神奥")
}
#[test]
fn unova_chinese_support() {
    let unova = get_region(5, Some("ch"));
    assert_eq!(unova, "合众")
}
#[test]
fn kalos_chinese_support() {
    let kalos = get_region(6, Some("ch"));
    assert_eq!(kalos, "卡洛斯")
}
#[test]
fn alola_chinese_support() {
    let alola = get_region(7, Some("ch"));
    assert_eq!(alola, "阿罗拉")
}
#[test]
fn galar_chinese_support() {
    let galar = get_region(8, Some("ch"));
    assert_eq!(galar, "伽勒尔")
}
#[test]
fn paldea_chinese_support() {
    let paldea = get_region(9, Some("ch"));
    assert_eq!(paldea, "帕尔迪亚")
}

#[test]
fn kanto_japanese_support() {
    let kanto = get_region(1, Some("jp"));
    assert_eq!(kanto, "カントー")
}

#[test]
fn johto_japanese_support() {
    let johto = get_region(2, Some("jp"));
    assert_eq!(johto, "ジョウト")
}

#[test]
fn hoenn_japanese_support() {
    let hoenn = get_region(3, Some("jp"));
    assert_eq!(hoenn, "ホウエン")
}

#[test]
fn sinnoh_japanese_support() {
    let sinnoh = get_region(4, Some("jp"));
    assert_eq!(sinnoh, "シンオウ")
}

#[test]
fn unova_japanese_support() {
    let unova = get_region(5, Some("jp"));
    assert_eq!(unova, "イッシュ")
}

#[test]
fn kalos_japanese_support() {
    let kalos = get_region(6, Some("jp"));
    assert_eq!(kalos, "カロス")
}

#[test]
fn alola_japanese_support() {
    let alola = get_region(7, Some("jp"));
    assert_eq!(alola, "アローラ")
}

#[test]
fn galar_japanese_support() {
    let galar = get_region(8, Some("jp"));
    assert_eq!(galar, "ガラル")
}

#[test]
fn paldea_japanese_support() {
    let paldea = get_region(9, Some("jp"));
    assert_eq!(paldea, "パルデア")
}

#[test]
fn kanto_russian_support() {
    let kanto = get_region(1, Some("ru"));
    assert_eq!(kanto, "Канто")
}

#[test]
fn johto_russian_support() {
    let johto = get_region(2, Some("ru"));
    assert_eq!(johto, "Джото")
}

#[test]
fn hoenn_russian_support() {
    let hoenn = get_region(3, Some("ru"));
    assert_eq!(hoenn, "Хоэнн")
}

#[test]
fn sinnoh_russian_support() {
    let sinnoh = get_region(4, Some("ru"));
    assert_eq!(sinnoh, "Синно")
}

#[test]
fn unova_russian_support() {
    let unova = get_region(5, Some("ru"));
    assert_eq!(unova, "Юнова")
}

#[test]
fn kalos_russian_support() {
    let kalos = get_region(6, Some("ru"));
    assert_eq!(kalos, "Калос")
}

#[test]
fn alola_russian_support() {
    let alola = get_region(7, Some("ru"));
    assert_eq!(alola, "Алола")
}

#[test]
fn galar_russian_support() {
    let galar = get_region(8, Some("ru"));
    assert_eq!(galar, "Галар")
}

#[test]
fn paldea_russian_support() {
    let paldea = get_region(9, Some("ru"));
    assert_eq!(paldea, "Палдеа")
}
