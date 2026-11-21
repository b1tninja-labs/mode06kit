//! The OBD-group document mapping and the Chevrolet Volt (Group 1A) selection.

use mode06kit::groups;

#[test]
fn the_volt_is_group_1a() {
    assert_eq!(groups::VOLT_OBD_GROUP, "1A");
}

#[test]
fn files_for_a_group_include_the_group_specific_and_the_generic() {
    let names: Vec<&str> = groups::files_for_group("1A")
        .iter()
        .map(|doc| doc.filename)
        .collect();
    assert!(names.contains(&"17OBDG01 Hybrid Diagnostics.pdf"));
    assert!(names.contains(&"GM CAN mode $06 data final_dm.pdf"));
    // A different group's hybrid file is not included.
    assert!(!names.contains(&"17OBDG02 Hybrid Diagnostics.pdf"));
}

#[test]
fn volt_files_match_group_1a() {
    assert_eq!(groups::volt_files(), groups::files_for_group("1A"));
}

#[test]
fn the_generic_files_apply_to_every_group() {
    assert!(
        groups::documents()
            .iter()
            .any(mode06kit::groups::Document::is_generic)
    );
    let group3: Vec<&str> = groups::files_for_group("3")
        .iter()
        .map(|doc| doc.filename)
        .collect();
    assert!(group3.contains(&groups::CAN_MODE06_FILE));
}
