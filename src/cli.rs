//! The read-only command line over the parsed GM Mode $06 definitions.
//!
//! The logic lives here as [`run`], which takes the argument list and returns an exit code and the
//! text to print, so it can be exercised in tests without spawning a process. Nothing here opens a
//! bus, sends a frame, or writes to a vehicle.

use std::fmt::Write as _;
use std::fs;

use crate::catalog;
use crate::groups;
use crate::model::Mode06Record;
use crate::parser::{group_by_obdmid, lookup, parse_all, parse_footnotes, reconstructed};

/// The usage text, shown on a missing or unknown command.
fn usage() -> String {
    [
        "usage: mode06kit <command>",
        "  records  [--source FILE]        list every parsed Mode $06 record",
        "  lookup   OBDMID [TID] [--source FILE]   look up records by OBDMID (and optional TID), hex",
        "  monitors [--source FILE]        list the monitor headings found and their row counts",
        "  review   [--source FILE]        list the low-confidence (reconstructed) rows and their raw lines",
        "  footnotes [--source FILE]       the FOOTNOTES section bodies (marker 1..20)",
        "  groups                          the OBD-group document mapping and the Volt selection",
    ]
    .join("\n")
}

/// Pull an optional `--source FILE` out of the arguments, returning `(source, positionals)`.
fn take_source(args: &[String]) -> (Option<String>, Vec<String>) {
    let mut source = None;
    let mut positionals = Vec::new();
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg == "--source" {
            source = iter.next().cloned();
        } else {
            positionals.push(arg.clone());
        }
    }
    (source, positionals)
}

/// Read the source text: the named file, or the bundled sample when `source` is [`None`].
fn load_text(source: Option<&str>) -> Result<String, String> {
    match source {
        None => Ok(catalog::SAMPLE_TEXT.to_string()),
        Some(path) => fs::read_to_string(path).map_err(|err| format!("cannot read {path}: {err}")),
    }
}

/// Load records from the named source, or the bundled sample when `source` is [`None`].
fn load(source: Option<&str>) -> Result<Vec<Mode06Record>, String> {
    match source {
        None => Ok(catalog::sample_records()),
        Some(_) => Ok(parse_all(&load_text(source)?)),
    }
}

/// Render one record as a single display line.
fn format_record(record: &Mode06Record) -> String {
    let uasid = record.uasid_hex().unwrap_or_else(|| "$--".to_string());
    let mut line = format!(
        "OBDMID {} TID {} UASID {uasid}",
        record.obdmid_hex(),
        record.tid_hex()
    );
    if let Some(footnote) = record.footnote {
        let _ = write!(line, " (fn {footnote})");
    }
    for extra in [&record.description, &record.unit_range, &record.resolution] {
        if !extra.is_empty() {
            line.push_str("  ");
            line.push_str(extra);
        }
    }
    if record.is_reconstructed() {
        line.push_str("  ~reconstructed");
    }
    line
}

/// Parse a hex byte argument such as `"01"` or `"a2"`.
fn parse_hex_arg(arg: &str) -> Result<u8, String> {
    u8::from_str_radix(arg, 16).map_err(|_| format!("not a hex byte: {arg}"))
}

fn cmd_records(args: &[String]) -> (i32, String) {
    let (source, _) = take_source(args);
    match load(source.as_deref()) {
        Err(message) => (1, message),
        Ok(records) => {
            let mut lines: Vec<String> = records.iter().map(format_record).collect();
            lines.push("--".to_string());
            lines.push(format!(
                "{} records across {} OBDMID(s)",
                records.len(),
                group_by_obdmid(&records).len()
            ));
            (0, lines.join("\n"))
        }
    }
}

fn cmd_lookup(args: &[String]) -> (i32, String) {
    let (source, positionals) = take_source(args);
    let Some(obdmid_arg) = positionals.first() else {
        return (2, format!("lookup needs an OBDMID\n{}", usage()));
    };
    let obdmid = match parse_hex_arg(obdmid_arg) {
        Ok(value) => value,
        Err(message) => return (2, message),
    };
    let tid = match positionals.get(1) {
        Some(arg) => match parse_hex_arg(arg) {
            Ok(value) => Some(value),
            Err(message) => return (2, message),
        },
        None => None,
    };
    let records = match load(source.as_deref()) {
        Err(message) => return (1, message),
        Ok(records) => records,
    };
    let hits = lookup(&records, obdmid, tid);
    if hits.is_empty() {
        let scope = tid.map_or_else(
            || "any TID".to_string(),
            |value| format!("TID ${value:02X}"),
        );
        return (0, format!("no record for OBDMID ${obdmid:02X} {scope}"));
    }
    (
        0,
        hits.iter()
            .map(|record| format_record(record))
            .collect::<Vec<_>>()
            .join("\n"),
    )
}

fn cmd_monitors(args: &[String]) -> (i32, String) {
    let (source, _) = take_source(args);
    match load(source.as_deref()) {
        Err(message) => (1, message),
        Ok(records) => {
            let mut order: Vec<String> = Vec::new();
            let mut counts: std::collections::HashMap<String, usize> =
                std::collections::HashMap::new();
            for record in &records {
                let name = record
                    .monitor
                    .clone()
                    .unwrap_or_else(|| "(no heading)".to_string());
                if !counts.contains_key(&name) {
                    order.push(name.clone());
                }
                *counts.entry(name).or_insert(0) += 1;
            }
            let lines: Vec<String> = order
                .iter()
                .map(|name| format!("{:>4}  {name}", counts[name]))
                .collect();
            (0, lines.join("\n"))
        }
    }
}

fn cmd_review(args: &[String]) -> (i32, String) {
    let (source, _) = take_source(args);
    match load(source.as_deref()) {
        Err(message) => (1, message),
        Ok(records) => {
            let low = reconstructed(&records);
            let mut lines: Vec<String> = Vec::new();
            for record in &low {
                lines.push(format_record(record));
                for cont in &record.continuations {
                    lines.push(format!("      | {cont}"));
                }
            }
            lines.push("--".to_string());
            lines.push(format!(
                "{} of {} rows had a reconstructed (low-confidence) description",
                low.len(),
                records.len()
            ));
            (0, lines.join("\n"))
        }
    }
}

fn cmd_footnotes(args: &[String]) -> (i32, String) {
    let (source, _) = take_source(args);
    match load_text(source.as_deref()) {
        Err(message) => (1, message),
        Ok(text) => {
            let footnotes = parse_footnotes(&text);
            if footnotes.is_empty() {
                return (
                    0,
                    "no footnotes in this source (point --source at a full extraction)".to_string(),
                );
            }
            let mut lines: Vec<String> = Vec::new();
            for footnote in &footnotes {
                let body = footnote.body.replace('\n', "\n      ");
                lines.push(format!("({}) {body}", footnote.marker));
            }
            lines.push("--".to_string());
            lines.push(format!("{} footnotes", footnotes.len()));
            (0, lines.join("\n"))
        }
    }
}

fn cmd_groups() -> String {
    let mut lines: Vec<String> = Vec::new();
    for doc in groups::documents() {
        let tag = if doc.obd_groups.is_empty() {
            "generic".to_string()
        } else {
            doc.obd_groups.join(",")
        };
        lines.push(format!("[{tag}]  {}  -- {}", doc.filename, doc.what));
    }
    lines.push("--".to_string());
    let volt = groups::volt_files();
    lines.push(format!(
        "Chevrolet Volt is OBD Group {}: {} applicable files",
        groups::VOLT_OBD_GROUP,
        volt.len()
    ));
    for doc in volt {
        lines.push(format!("    {}", doc.filename));
    }
    lines.join("\n")
}

/// Run the command line: returns `(exit_code, text)`; the text prints to stdout on 0, else stderr.
#[must_use]
pub fn run(args: &[String]) -> (i32, String) {
    match args.split_first() {
        None => (2, usage()),
        Some((command, rest)) => match command.as_str() {
            "records" => cmd_records(rest),
            "lookup" => cmd_lookup(rest),
            "monitors" => cmd_monitors(rest),
            "review" => cmd_review(rest),
            "footnotes" => cmd_footnotes(rest),
            "groups" => (0, cmd_groups()),
            other => (2, format!("unknown command: {other}\n{}", usage())),
        },
    }
}
