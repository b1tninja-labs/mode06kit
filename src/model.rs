//! The typed model for a GM Mode $06 test definition.
//!
//! A Mode $06 record is one row of GM's monitor/test-ID tables. The reliably structured fields are
//! the three identifiers GM prints in the leading columns -- the OBD Monitor ID (OBDMID), the Test
//! ID (TID) and the unit-and-scaling ID (UASID, "For Information ONLY" per ISO 15031-5) -- plus an
//! optional footnote marker. The description and the value range/resolution are carried as the text
//! GM published; [`Mode06Record::value_range`] and [`Mode06Record::scaling`] parse the clean cases
//! and return [`None`] when GM's layout is too wrapped to split safely.
//!
//! Nothing here is stated from memory: a record is only ever produced from text read out of GM's own
//! document (`docs/sources.md`). The ISO 15031-5 UASID -> unit table is not loaded in `primarysource`
//! (`docs/iso-map.md`), so the UASID is kept as its raw byte, not expanded.

/// The en dash (U+2013) GM prints for some minus signs, where a hyphen-minus is meant.
pub const EN_DASH: char = '\u{2013}';

/// The min and max of a test value, in the unit GM named for it.
#[derive(Debug, Clone, PartialEq)]
pub struct ValueRange {
    /// The lower limit of the range.
    pub minimum: f64,
    /// The upper limit of the range.
    pub maximum: f64,
    /// The unit GM printed for the range, e.g. `"Volts"` (may be empty).
    pub unit: String,
}

/// The per-bit resolution of a test value, in the unit GM named for it.
#[derive(Debug, Clone, PartialEq)]
pub struct Scaling {
    /// The value of one least-significant bit.
    pub per_bit: f64,
    /// The unit GM printed for the resolution, e.g. `"mV"` (may be empty).
    pub unit: String,
}

/// Where a record's reconstructed fields came from -- in effect, the confidence of the join.
///
/// GM's PDF wraps a long description across several output lines and vertically offsets the ID
/// column from the description column, so a fragment can land on a *different* row's text line once
/// the table is flattened (see `docs/extraction-limits.md`). The parser fills a row's empty cells
/// only from the continuation lines that follow its own ID line, and records how it got the
/// description so a caller can treat the low-confidence joins with suspicion or revisit the PDF.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DescriptionSource {
    /// The description was printed on the row's own ID line (highest confidence).
    Primary,
    /// The ID line carried no description; this one was adopted from a following continuation line
    /// (lower confidence -- the fragment could instead belong to a stacked second definition).
    Reconstructed,
    /// No description was found for the row, on its ID line or in any continuation.
    Missing,
}

/// One Mode $06 monitor/test-ID definition read from a GM document.
#[derive(Debug, Clone, PartialEq)]
pub struct Mode06Record {
    /// OBD Monitor ID, e.g. `0x01` for Oxygen Sensor Monitor Bank 1 Sensor 1.
    pub obdmid: u8,
    /// Test ID within the monitor, e.g. `0x01`.
    pub tid: u8,
    /// Unit-and-scaling ID (ISO 15031-5), or [`None`] when GM left the cell blank.
    pub uasid: Option<u8>,
    /// The test description GM printed (best effort; empty when the cell wrapped away).
    pub description: String,
    /// How [`Self::description`] was obtained -- its join confidence.
    pub description_source: DescriptionSource,
    /// The raw range cell, e.g. `"0.0000 to 7.9900 Volts"` (empty when none was on the row).
    pub unit_range: String,
    /// The raw resolution cell, e.g. `"0.122 mV / bit"` (empty when none was on the row).
    pub resolution: String,
    /// The footnote marker on the TID, e.g. `1`, or [`None`]. Footnotes are model-year caveats.
    pub footnote: Option<u32>,
    /// The monitor section heading this row fell under, or [`None`] when parsed without one.
    pub monitor: Option<String>,
    /// 1-based line number of the row within the parsed text, for citation back to the source.
    pub line: usize,
    /// The raw continuation lines that followed this row's ID line before the next row, verbatim
    /// and in document order. Kept losslessly: they may be wrapped description fragments, stacked
    /// second definitions, or range/resolution cells that wrapped off the ID line.
    pub continuations: Vec<String>,
}

impl Mode06Record {
    /// The OBDMID in GM's `$HH` notation.
    #[must_use]
    pub fn obdmid_hex(&self) -> String {
        format!("${:02X}", self.obdmid)
    }

    /// The TID in GM's `$HH` notation.
    #[must_use]
    pub fn tid_hex(&self) -> String {
        format!("${:02X}", self.tid)
    }

    /// The UASID in GM's `$HH` notation, or [`None`] when blank.
    #[must_use]
    pub fn uasid_hex(&self) -> Option<String> {
        self.uasid.map(|value| format!("${value:02X}"))
    }

    /// The parsed min/max/unit of [`Self::unit_range`], or [`None`] if it does not parse cleanly.
    #[must_use]
    pub fn value_range(&self) -> Option<ValueRange> {
        parse_range(&self.unit_range)
    }

    /// The parsed per-bit resolution of [`Self::resolution`], or [`None`] if it does not parse cleanly.
    #[must_use]
    pub fn scaling(&self) -> Option<Scaling> {
        parse_scaling(&self.resolution)
    }

    /// Whether the description was reconstructed from a continuation line (a low-confidence join).
    #[must_use]
    pub fn is_reconstructed(&self) -> bool {
        self.description_source == DescriptionSource::Reconstructed
    }
}

/// One entry from the document's FOOTNOTES section: a marker and its applicability text.
///
/// Footnotes are model-year / vehicle caveats referenced from a row's TID (see
/// [`Mode06Record::footnote`]). The body is the text GM printed for the marker with the page-break
/// furniture removed; it is kept as text, not parsed into structured vehicle or OBDMID/TID rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Footnote {
    /// The footnote marker, `1`..=`20`, matching [`Mode06Record::footnote`].
    pub marker: u32,
    /// The applicability text GM printed, furniture removed and lines joined by `\n`.
    pub body: String,
}

/// Parse a GM numeric token, treating the en dash GM uses for minus as a minus sign.
fn to_f64(token: &str) -> Option<f64> {
    token.trim().replace(EN_DASH, "-").parse::<f64>().ok()
}

/// Split a leading numeric prefix from a cell, returning `(value, remaining unit)`.
///
/// Handles `"0.122 mV"`, `"2 mV/s"`, `"0.01V"` (no space) and `"0.000305"` (no unit).
fn split_leading_number(text: &str) -> Option<(f64, String)> {
    let text = text.trim();
    let mut end = 0;
    for (index, ch) in text.char_indices() {
        let is_sign = ch == '+' || ch == '-' || ch == EN_DASH;
        if ch.is_ascii_digit() || ch == '.' || (is_sign && index == 0) {
            end = index + ch.len_utf8();
        } else {
            break;
        }
    }
    let value = to_f64(&text[..end])?;
    Some((value, text[end..].trim().to_string()))
}

/// Parse a GM range cell such as `"0.0000 to 7.9900 Volts"` or `"-32.768 to +32.767 kPa"`.
#[must_use]
pub fn parse_range(cell: &str) -> Option<ValueRange> {
    let (lo, rest) = cell.trim().split_once(" to ")?;
    let minimum = to_f64(lo)?;
    let (maximum, unit) = split_leading_number(rest)?;
    Some(ValueRange {
        minimum,
        maximum,
        unit,
    })
}

/// Parse a GM resolution cell such as `"0.122 mV / bit"`, `"2 mV/s / bit"` or `"0.000305 / bit"`.
#[must_use]
pub fn parse_scaling(cell: &str) -> Option<Scaling> {
    let cell = cell.trim();
    let body = cell.strip_suffix("bit")?.trim_end();
    let body = body.strip_suffix('/')?.trim_end();
    let (per_bit, unit) = split_leading_number(body)?;
    Some(Scaling { per_bit, unit })
}
