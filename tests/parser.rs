//! The parser: leading-ID extraction, footnote markers, cell splitting, grouping and lookup.

use mode06kit::catalog::sample_records;
use mode06kit::parser::{
    group_by_obdmid, lookup, parse_all, parse_document, parse_records, parse_section,
};

/// A multi-monitor excerpt of GM's CAN Mode $06 document: two Oxygen Sensor sections, a page break
/// (its boilerplate and column-header furniture), and the `Catalyst Monitor Bank 1` heading whose
/// line trails a legend. Every line is real GM text read through the `gm_*` tools.
const MULTI_MONITOR: &str = "\
                   Oxygen Sensor Monitor Bank 1 Sensor 1

01        03       0A                           Rich to Lean Sensor Switch Time                    0 to 65535 ms           1 ms / bit
01        04       0A                           Lean to Rich Sensor Switch Time                    0 to 65535 ms           1 ms / bit
                      Oxygen Sensor Monitor Bank 1 Sensor 3

03         80        16         Particulate Matter Sensor Temperature Not Plausible                  -40 to 6513.5 C     0.1 C / bit
03         81        24         Particulate Matter Sensor Temperature Performance                  0 to 655.35 uAmps       0.01 uAmps / bit
Footnotes are explained on the last pages of this document.
[page 14]
           GM mode $06 data definitions for GM vehicles using GMLAN diagnostic data link
            OBD     Test     Units and                                Description                                Range                  Resolution
 Monitor             Scaling                                                                   For Information ONLY.          For Information
                   Catalyst Monitor Bank 1                                                                    EWMA = Exponentially Weighted
21         84 (8)   05                       Catalyst Test Bank 1 ( normalized ) - EWMA                        0 to 1.999              0.0000305 / bit
";

#[test]
fn a_clean_row_yields_the_three_leading_ids() {
    let records = parse_records(
        "01        03       0A     Rich to Lean Sensor Switch Time    0 to 65535 ms   1 ms / bit",
        None,
    );
    assert_eq!(records.len(), 1);
    let record = &records[0];
    assert_eq!(
        (record.obdmid, record.tid, record.uasid),
        (0x01, 0x03, Some(0x0A))
    );
    assert_eq!(record.description, "Rich to Lean Sensor Switch Time");
    assert_eq!(record.unit_range, "0 to 65535 ms");
    assert_eq!(record.resolution, "1 ms / bit");
    assert_eq!(record.footnote, None);
}

#[test]
fn a_tid_footnote_marker_is_captured() {
    let records = parse_records(
        "01        10 (1)   06     Dynamic Response Performance      0 to 1.999    0.122 mV / bit",
        None,
    );
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].tid, 0x10);
    assert_eq!(records[0].footnote, Some(1));
    assert_eq!(records[0].uasid, Some(0x06));
}

#[test]
fn a_wrapped_continuation_line_is_not_a_record() {
    let text = "01   01   0A   Rich to Lean Sensor Threshold Voltage   0.0000 to 7.9900 Volts   0.122 mV / bit\n\
                          Lean to Rich Sensor Threshold Voltage   0.0000 to 7.9900 Volts   0.122 mV / bit";
    assert_eq!(parse_records(text, None).len(), 1);
}

#[test]
fn description_starting_with_hex_letters_is_not_misread_as_a_row() {
    // "B1S1 WRAF Slow Response" leads with B1 but is a wrapped description, not OBDMID B1.
    let records = parse_records(
        "          B1S1 WRAF Slow Response   -40 to 6513.5 C   0.1 C / bit",
        None,
    );
    assert_eq!(records.len(), 0);
}

#[test]
fn boilerplate_and_comments_are_skipped() {
    let text = "# provenance\n[page 30]\nFOOTNOTES\nGMLAN rev3\n01 01 0A Foo 0 to 5 V 1 V / bit";
    let records = parse_records(text, None);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].obdmid, 0x01);
}

#[test]
fn a_slash_unit_resolution_is_split_from_the_range() {
    let text = "Oxygen Sensor Monitor Bank 1 Sensor 1\n\
                01        84 (16)  04                                                                        -65536 to +65534 mV/s        2 mV/s / bit";
    let records = parse_section(text);
    let hit = records.iter().find(|r| r.tid == 0x84).expect("row parses");
    assert_eq!(hit.footnote, Some(16));
    assert_eq!(hit.unit_range, "-65536 to +65534 mV/s");
    assert_eq!(hit.resolution, "2 mV/s / bit");
}

#[test]
fn parse_section_names_the_monitor_from_the_heading() {
    let records = sample_records();
    assert!(records.iter().any(|r| r.tid == 0x01));
    assert!(
        records
            .iter()
            .all(|r| r.monitor.as_deref() == Some("Oxygen Sensor Monitor Bank 1 Sensor 1"))
    );
}

#[test]
fn the_sample_is_all_obdmid_01() {
    let records = sample_records();
    let grouped = group_by_obdmid(&records);
    assert_eq!(grouped.keys().copied().collect::<Vec<_>>(), vec![0x01]);
}

#[test]
fn lookup_filters_by_obdmid_and_tid() {
    let records = sample_records();
    let hits = lookup(&records, 0x01, Some(0x04));
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].description, "Lean to Rich Sensor Switch Time");
    assert_eq!(lookup(&records, 0x05, None).len(), 0);
}

#[test]
fn parse_document_splits_the_document_into_monitor_sections() {
    let sections = parse_document(MULTI_MONITOR);
    let names: Vec<Option<&str>> = sections.iter().map(|s| s.monitor.as_deref()).collect();
    assert_eq!(
        names,
        vec![
            Some("Oxygen Sensor Monitor Bank 1 Sensor 1"),
            Some("Oxygen Sensor Monitor Bank 1 Sensor 3"),
            Some("Catalyst Monitor Bank 1"),
        ]
    );
    let counts: Vec<usize> = sections.iter().map(|s| s.records.len()).collect();
    assert_eq!(counts, vec![2, 2, 1]);
}

#[test]
fn a_heading_that_trails_a_legend_keeps_only_the_monitor_name() {
    // GM's "Catalyst Monitor Bank 1" heading shares its line with an "EWMA = ..." legend; only the
    // monitor name (the first column cell) is kept.
    let section = parse_document(MULTI_MONITOR)
        .into_iter()
        .find(|s| s.records.iter().any(|r| r.obdmid == 0x21))
        .expect("the catalyst section parses");
    assert_eq!(section.monitor.as_deref(), Some("Catalyst Monitor Bank 1"));
}

#[test]
fn a_page_break_header_does_not_start_a_new_section() {
    // The column-header furniture ("Monitor  Scaling ...") sits inside the Bank 1 Sensor 3 section;
    // the row after it stays tagged with that monitor rather than a spurious "Monitor" section.
    let sections = parse_document(MULTI_MONITOR);
    assert!(
        sections
            .iter()
            .all(|s| s.monitor.as_deref() != Some("Monitor")),
        "the bare-word 'Monitor' page header must not become a section"
    );
}

#[test]
fn parse_all_tags_each_row_with_its_own_monitor() {
    let records = parse_all(MULTI_MONITOR);
    let catalyst = records
        .iter()
        .find(|r| r.obdmid == 0x21)
        .expect("the catalyst row parses");
    assert_eq!(catalyst.monitor.as_deref(), Some("Catalyst Monitor Bank 1"));
    let oxygen = records
        .iter()
        .find(|r| r.obdmid == 0x03)
        .expect("the bank 1 sensor 3 row parses");
    assert_eq!(
        oxygen.monitor.as_deref(),
        Some("Oxygen Sensor Monitor Bank 1 Sensor 3")
    );
}

#[test]
fn records_before_the_first_heading_are_a_monitorless_section() {
    let sections = parse_document("02   07   24   Foo Test   0 to 9 counts   1 count / bit");
    assert_eq!(sections.len(), 1);
    assert_eq!(sections[0].monitor, None);
    assert_eq!(sections[0].records.len(), 1);
}
