//! The record model: hex notation and the range/scaling parsers, including GM's en-dash minus.

use mode06kit::model::{DescriptionSource, Mode06Record};

fn sample() -> Mode06Record {
    Mode06Record {
        obdmid: 0x01,
        tid: 0x04,
        uasid: Some(0x0A),
        description: "Lean to Rich Sensor Switch Time".to_string(),
        description_source: DescriptionSource::Primary,
        unit_range: "0 to 65535 ms".to_string(),
        resolution: "1 ms / bit".to_string(),
        footnote: None,
        monitor: Some("Oxygen Sensor Monitor Bank 1 Sensor 1".to_string()),
        line: 5,
        continuations: Vec::new(),
    }
}

#[test]
fn identifiers_render_in_gm_dollar_hex() {
    let record = sample();
    assert_eq!(record.obdmid_hex(), "$01");
    assert_eq!(record.tid_hex(), "$04");
    assert_eq!(record.uasid_hex(), Some("$0A".to_string()));
}

#[test]
fn a_blank_uasid_renders_as_none() {
    let mut record = sample();
    record.uasid = None;
    assert_eq!(record.uasid_hex(), None);
}

#[test]
fn value_range_parses_min_max_and_unit() {
    let value = sample().value_range().expect("range parses");
    assert_eq!(value.minimum, 0.0);
    assert_eq!(value.maximum, 65535.0);
    assert_eq!(value.unit, "ms");
}

#[test]
fn scaling_parses_per_bit_and_unit() {
    let scaling = sample().scaling().expect("scaling parses");
    assert_eq!(scaling.per_bit, 1.0);
    assert_eq!(scaling.unit, "ms");
}

#[test]
fn an_en_dash_is_read_as_a_minus_sign() {
    // GM prints some negative limits with an en dash (U+2013) instead of a hyphen.
    let mut record = sample();
    record.unit_range = "\u{2013}32.768 to +32.767 kPa".to_string();
    let value = record.value_range().expect("range parses");
    assert_eq!(value.minimum, -32.768);
    assert_eq!(value.maximum, 32.767);
    assert_eq!(value.unit, "kPa");
}

#[test]
fn a_unit_with_a_slash_survives_the_scaling_parse() {
    let mut record = sample();
    record.resolution = "2 mV/s / bit".to_string();
    let scaling = record.scaling().expect("scaling parses");
    assert_eq!(scaling.per_bit, 2.0);
    assert_eq!(scaling.unit, "mV/s");
}

#[test]
fn a_wrapped_cell_returns_none_not_a_guess() {
    let mut record = sample();
    record.unit_range = String::new();
    record.resolution = String::new();
    assert_eq!(record.value_range(), None);
    assert_eq!(record.scaling(), None);
}
