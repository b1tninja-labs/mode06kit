//! The read-only command line over the parsed definitions, exercised through `cli::run`.

use std::io::Write;

use mode06kit::cli::run;

fn args(parts: &[&str]) -> Vec<String> {
    parts.iter().map(|s| (*s).to_string()).collect()
}

#[test]
fn records_lists_and_summarises() {
    let (code, out) = run(&args(&["records"]));
    assert_eq!(code, 0);
    assert!(out.contains("OBDMID $01 TID $04 UASID $0A"));
    assert!(out.contains("records across 1 OBDMID(s)"));
}

#[test]
fn lookup_by_obdmid_and_tid() {
    let (code, out) = run(&args(&["lookup", "01", "04"]));
    assert_eq!(code, 0);
    assert!(out.contains("Lean to Rich Sensor Switch Time"));
}

#[test]
fn lookup_reports_a_miss_without_failing() {
    let (code, out) = run(&args(&["lookup", "05"]));
    assert_eq!(code, 0);
    assert!(out.contains("no record for OBDMID $05"));
}

#[test]
fn a_footnote_marker_shows_in_the_listing() {
    let (code, out) = run(&args(&["lookup", "01", "10"]));
    assert_eq!(code, 0);
    assert!(out.contains("(fn 1)"));
}

#[test]
fn monitors_lists_the_heading() {
    let (code, out) = run(&args(&["monitors"]));
    assert_eq!(code, 0);
    assert!(out.contains("Oxygen Sensor Monitor Bank 1 Sensor 1"));
}

#[test]
fn groups_shows_the_volt_selection() {
    let (code, out) = run(&args(&["groups"]));
    assert_eq!(code, 0);
    assert!(out.contains("Chevrolet Volt is OBD Group 1A"));
    assert!(out.contains("17OBDG01 Hybrid Diagnostics.pdf"));
}

#[test]
fn source_reads_a_local_file() {
    let mut path = std::env::temp_dir();
    path.push(format!("mode06kit_cli_{}.txt", std::process::id()));
    let mut file = std::fs::File::create(&path).expect("create temp");
    file.write_all(b"My Monitor\n02   07   24   Foo Test   0 to 9 counts   1 count / bit\n")
        .expect("write temp");
    let source = path.to_string_lossy().to_string();
    let (code, out) = run(&args(&["records", "--source", &source]));
    let _ = std::fs::remove_file(&path);
    assert_eq!(code, 0);
    assert!(out.contains("OBDMID $02 TID $07 UASID $24"));
}

#[test]
fn an_unknown_command_is_an_error() {
    let (code, _) = run(&args(&["frobnicate"]));
    assert_eq!(code, 2);
}
