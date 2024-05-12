use crate::get_region;

#[test]
pub fn kanto_english_support() {
    let kanto = get_region(1, Some("en"));
    assert_eq!(kanto, "Kanto")
}
#[test]
pub fn johto_english_support() {
    let johto = get_region(2, Some("en"));
    assert_eq!(johto, "Johto")
}
#[test]
pub fn hoenn_english_support() {
    let hoenn = get_region(3, Some("en"));
    assert_eq!(hoenn, "Hoenn")
}
#[test]
pub fn sinnoh_english_support() {
    let sinnoh = get_region(4, Some("en"));
    assert_eq!(sinnoh, "Sinnoh")
}
#[test]
pub fn unova_english_support() {
    let unova = get_region(5, Some("en"));
    assert_eq!(unova, "Unova")
}
#[test]
pub fn kalos_english_support() {
    let kalos = get_region(6, Some("en"));
    assert_eq!(kalos, "Kalos")
}
#[test]
pub fn alola_english_support() {
    let alola = get_region(7, Some("en"));
    assert_eq!(alola, "Alola")
}
#[test]
pub fn galar_english_support() {
    let galar = get_region(8, Some("en"));
    assert_eq!(galar, "Galar")
}
#[test]
pub fn paldea_english_support() {
    let paldea = get_region(9, Some("en"));
    assert_eq!(paldea, "Paldea")
}
#[test]
pub fn kanto_german_support() {
    let kanto = get_region(1, Some("de"));
    assert_eq!(kanto, "Kanto")
}
#[test]
pub fn johto_german_support() {
    let johto = get_region(2, Some("de"));
    assert_eq!(johto, "Johto")
}
#[test]
pub fn hoenn_german_support() {
    let hoenn = get_region(3, Some("de"));
    assert_eq!(hoenn, "Hoenn")
}
#[test]
pub fn sinnoh_german_support() {
    let sinnoh = get_region(4, Some("de"));
    assert_eq!(sinnoh, "Sinnoh")
}
#[test]
pub fn einall_german_support() {
    let einall = get_region(5, Some("de"));
    assert_eq!(einall, "Einall")
}
#[test]
pub fn kalos_german_support() {
    let kalos = get_region(6, Some("de"));
    assert_eq!(kalos, "Kalos")
}
#[test]
pub fn alola_german_support() {
    let alola = get_region(7, Some("de"));
    assert_eq!(alola, "Alola")
}
#[test]
pub fn galar_german_support() {
    let galar = get_region(8, Some("de"));
    assert_eq!(galar, "Galar")
}
#[test]
pub fn paldea_german_support() {
    let paldea = get_region(9, Some("de"));
    assert_eq!(paldea, "Paldea")
}
#[test]
pub fn kanto_french_support() {
    let kanto = get_region(1, Some("fr"));
    assert_eq!(kanto, "Kanto")
}
#[test]
pub fn johto_french_support() {
    let johto = get_region(2, Some("fr"));
    assert_eq!(johto, "Johto")
}
#[test]
pub fn hoenn_french_support() {
    let hoenn = get_region(3, Some("fr"));
    assert_eq!(hoenn, "Hoenn")
}
#[test]
pub fn sinnoh_french_support() {
    let sinnoh = get_region(4, Some("fr"));
    assert_eq!(sinnoh, "Sinnoh")
}
#[test]
pub fn unys_french_support() {
    let unys = get_region(5, Some("fr"));
    assert_eq!(unys, "Unys")
}
#[test]
pub fn kalos_french_support() {
    let kalos = get_region(6, Some("fr"));
    assert_eq!(kalos, "Kalos")
}
#[test]
pub fn alola_french_support() {
    let alola = get_region(7, Some("fr"));
    assert_eq!(alola, "Alola")
}
#[test]
pub fn galar_french_support() {
    let galar = get_region(8, Some("fr"));
    assert_eq!(galar, "Galar")
}
#[test]
pub fn paldea_french_support() {
    let paldea = get_region(9, Some("fr"));
    assert_eq!(paldea, "Paldea")
}
#[test]
pub fn kanto_chinese_support() {
    let kanto = get_region(1, Some("ch"));
    assert_eq!(kanto, "关都")
}
#[test]
pub fn johto_chinese_support() {
    let johto = get_region(2, Some("ch"));
    assert_eq!(johto, "城都")
}
#[test]
pub fn hoenn_chinese_support() {
    let hoenn = get_region(3, Some("ch"));
    assert_eq!(hoenn, "丰缘")
}
#[test]
pub fn sinnoh_chinese_support() {
    let sinnoh = get_region(4, Some("ch"));
    assert_eq!(sinnoh, "神奥")
}
#[test]
pub fn unova_chinese_support() {
    let unova = get_region(5, Some("ch"));
    assert_eq!(unova, "合众")
}
#[test]
pub fn kalos_chinese_support() {
    let kalos = get_region(6, Some("ch"));
    assert_eq!(kalos, "卡洛斯")
}
#[test]
pub fn alola_chinese_support() {
    let alola = get_region(7, Some("ch"));
    assert_eq!(alola, "阿罗拉")
}
#[test]
pub fn galar_chinese_support() {
    let galar = get_region(8, Some("ch"));
    assert_eq!(galar, "伽勒尔")
}
#[test]
pub fn paldea_chinese_support() {
    let paldea = get_region(9, Some("ch"));
    assert_eq!(paldea, "帕尔迪亚")
}
