//! The bundled GM CAN Mode $06 sample, embedded at build time and parsed on demand.
//!
//! The crate embeds one representative monitor section of GM's `GM CAN mode $06 data` reference
//! (`src/data/gm_can_mode06_sample.txt`, provenance in its header) so the CLI and tests parse
//! something real without a local copy of the full PDF. Point the CLI's `--source` at a full text
//! extraction to parse the whole document with the same parser.

use crate::model::Mode06Record;
use crate::parser::parse_section;

/// The embedded GM CAN Mode $06 sample text, including its provenance header comments.
pub const SAMPLE_TEXT: &str = include_str!("data/gm_can_mode06_sample.txt");

/// The embedded sample parsed into records.
#[must_use]
pub fn sample_records() -> Vec<Mode06Record> {
    parse_section(SAMPLE_TEXT)
}
