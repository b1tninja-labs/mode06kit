//! The embedded GM CAN Mode $06 sample loads, carries its provenance, and parses to records.

use mode06kit::catalog;

#[test]
fn the_sample_text_carries_a_provenance_header() {
    assert!(catalog::SAMPLE_TEXT.contains("GM CAN mode $06 data"));
    assert!(catalog::SAMPLE_TEXT.trim_start().starts_with('#'));
}

#[test]
fn the_sample_parses_to_records() {
    let records = catalog::sample_records();
    assert!(records.len() >= 10);
    assert!(records.iter().all(|r| r.obdmid == 0x01));
}

#[test]
fn the_provenance_comments_are_not_parsed_as_records() {
    let lines: Vec<&str> = catalog::SAMPLE_TEXT.lines().collect();
    for record in catalog::sample_records() {
        assert!(!lines[record.line - 1].starts_with('#'));
    }
}
