use crate::get_all_types;

#[test]
fn support_for_all_types() {
    let all_types = get_all_types(Some("en"));
    assert_eq!(all_types.len(), 18)
}
#[test]
fn type_is_correct_format() {
    let all_types = get_all_types(Some("en"));
    let normal_type = all_types.first().unwrap().to_owned();

    assert_eq!(normal_type, String::from("Normal"));
}
