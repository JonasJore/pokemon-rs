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
