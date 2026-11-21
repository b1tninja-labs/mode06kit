//! Parse the text of GM's `GM CAN mode $06 data` reference into [`Mode06Record`] values.
//!
//! GM's document is a column layout flattened to text: each data row leads with the OBDMID, the TID
//! (optionally carrying a `(n)` footnote marker) and the UASID, then the description, value range
//! and resolution cells, which the PDF separates by runs of whitespace. Description cells sometimes
//! wrap onto the next line; those wrapped continuations have no leading IDs and are left out of the
//! structured record (the reliable core is the three IDs and the footnote -- see `ROADMAP.md` for
//! the deeper description-reconstruction work that is deliberately not built yet).
//!
//! The parser only ever runs on text the caller read out of GM's own document; it invents nothing.

use std::collections::BTreeMap;

use crate::model::{Mode06Record, parse_range, parse_scaling};

/// A parsed data row before it is wrapped in a [`Mode06Record`].
struct RawRow {
    obdmid: u8,
    tid: u8,
    footnote: Option<u32>,
    uasid: u8,
    rest: String,
}

/// Split the first whitespace-delimited token off `text`, returning `(token, remainder)`.
///
/// The remainder keeps its internal spacing (only the gap before the token is consumed), so a row's
/// trailing text can still be split into cells by its column gaps.
fn next_token(text: &str) -> Option<(&str, &str)> {
    let text = text.trim_start();
    if text.is_empty() {
        return None;
    }
    match text.find(char::is_whitespace) {
        Some(index) => Some((&text[..index], &text[index..])),
        None => Some((text, "")),
    }
}

/// Parse a two-hex-digit token into a byte, or [`None`] if it is not exactly two hex digits.
fn parse_hex2(token: &str) -> Option<u8> {
    if token.len() == 2 && token.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        u8::from_str_radix(token, 16).ok()
    } else {
        None
    }
}

/// Parse a `(n)` footnote token into its number, or [`None`] if it is not that shape.
fn parse_footnote(token: &str) -> Option<u32> {
    let inner = token.strip_prefix('(')?.strip_suffix(')')?;
    if inner.is_empty() || !inner.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    inner.parse::<u32>().ok()
}

/// Page furniture and our own provenance comments: never rows, never monitor headings.
fn is_boilerplate(line: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.starts_with('#') || trimmed.starts_with("[page") {
        return true;
    }
    for marker in [
        "GM mode $06 data definitions",
        "Some items have footnotes",
        "FOOTNOTES",
        "Footnotes are explained",
    ] {
        if trimmed.starts_with(marker) {
            return true;
        }
    }
    let collapsed = trimmed.trim_end();
    collapsed.starts_with("GMLAN rev")
        && collapsed["GMLAN rev".len()..]
            .bytes()
            .all(|b| b.is_ascii_digit())
}

/// Split a row's trailing text into `(description, unit_range, resolution)` by its column gaps.
fn split_rest(rest: &str) -> (String, String, String) {
    let mut description: Vec<&str> = Vec::new();
    let mut unit_range = String::new();
    let mut resolution = String::new();
    for cell in rest.trim().split("  ") {
        let cell = cell.trim();
        if cell.is_empty() {
            continue;
        }
        if resolution.is_empty() && parse_scaling(cell).is_some() {
            resolution = cell.to_string();
        } else if unit_range.is_empty() && parse_range(cell).is_some() {
            unit_range = cell.to_string();
        } else {
            description.push(cell);
        }
    }
    (description.join(" "), unit_range, resolution)
}

/// Parse one line into a [`RawRow`], or [`None`] if it is not a data row.
fn parse_row(line: &str) -> Option<RawRow> {
    let (t0, r0) = next_token(line)?;
    let obdmid = parse_hex2(t0)?;
    let (t1, r1) = next_token(r0)?;
    let tid = parse_hex2(t1)?;
    let (footnote, after_footnote) = match next_token(r1) {
        Some((token, remainder)) if parse_footnote(token).is_some() => {
            (parse_footnote(token), remainder)
        }
        _ => (None, r1),
    };
    let (t3, r3) = next_token(after_footnote)?;
    let uasid = parse_hex2(t3)?;
    Some(RawRow {
        obdmid,
        tid,
        footnote,
        uasid,
        rest: r3.trim_start().to_string(),
    })
}

/// Build a [`Mode06Record`] from a raw row, its monitor and its 1-based line number.
fn record(row: &RawRow, monitor: Option<&str>, line: usize) -> Mode06Record {
    let (description, unit_range, resolution) = split_rest(&row.rest);
    Mode06Record {
        obdmid: row.obdmid,
        tid: row.tid,
        uasid: Some(row.uasid),
        description,
        unit_range,
        resolution,
        footnote: row.footnote,
        monitor: monitor.map(str::to_string),
        line,
    }
}

/// Parse every data row in `text` into records, all tagged with `monitor`.
#[must_use]
pub fn parse_records(text: &str, monitor: Option<&str>) -> Vec<Mode06Record> {
    let mut records = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if is_boilerplate(line) {
            continue;
        }
        if let Some(row) = parse_row(line) {
            records.push(record(&row, monitor, index + 1));
        }
    }
    records
}

/// Parse one monitor section: the first non-row, non-boilerplate line names the monitor.
///
/// This matches how a single section reads out of the GM document (the monitor heading, then its
/// rows). Whole-document segmentation is a `ROADMAP.md` open item.
#[must_use]
pub fn parse_section(text: &str) -> Vec<Mode06Record> {
    let mut monitor: Option<String> = None;
    let mut records = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if line.trim().is_empty() || is_boilerplate(line) {
            continue;
        }
        match parse_row(line) {
            Some(row) => records.push(record(&row, monitor.as_deref(), index + 1)),
            None => {
                if monitor.is_none() {
                    monitor = Some(line.trim().to_string());
                }
            }
        }
    }
    records
}

/// Group records by OBDMID, preserving the order they were parsed in within each group.
#[must_use]
pub fn group_by_obdmid(records: &[Mode06Record]) -> BTreeMap<u8, Vec<&Mode06Record>> {
    let mut grouped: BTreeMap<u8, Vec<&Mode06Record>> = BTreeMap::new();
    for record in records {
        grouped.entry(record.obdmid).or_default().push(record);
    }
    grouped
}

/// Return the records matching `obdmid` (and `tid` when given), in parse order.
#[must_use]
pub fn lookup(records: &[Mode06Record], obdmid: u8, tid: Option<u8>) -> Vec<&Mode06Record> {
    records
        .iter()
        .filter(|record| record.obdmid == obdmid && tid.is_none_or(|wanted| record.tid == wanted))
        .collect()
}
