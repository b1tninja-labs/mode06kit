# Obligations and their evidence

Every obligation `mode06kit` has taken on, with an honest status against the code, in the Rust
`Status | Obligation | Anchor | Test` matrix read by `projecthub sync`.

**The honesty rule:** a row reads `[ok]` only when the named test exists and would fail if the
behaviour regressed. `[open]` is unimplemented; `[unread]` would mean the test passes but the anchor
could not be read cleanly from the standard.

The source is GM's own `GM CAN mode $06 data` reference, read through the `primarysource` `gm_*`
tools (document `mode6-can`) and recorded in `docs/sources.md`. ISO 15031-5 itself is **not** loaded
in `primarysource`, so the UASID is kept as a raw byte rather than expanded to a unit
(`docs/iso-map.md`).

## Parsing the monitor/test-ID rows

| Status | Obligation | Anchor | Test |
|---|---|---|---|
| `[ok]` | `parser::parse_records` extracts the OBDMID, TID and UASID from a data row | GM CAN mode $06 data rows | `tests/parser.rs::a_clean_row_yields_the_three_leading_ids` |
| `[ok]` | `parser::parse_records` captures the `(n)` footnote marker on the TID | `gm:mode6-can§footnotes` | `tests/parser.rs::a_tid_footnote_marker_is_captured` |
| `[ok]` | `parser::parse_records` leaves a wrapped continuation line out of the records | GM CAN mode $06 data layout | `tests/parser.rs::a_wrapped_continuation_line_is_not_a_record` |
| `[ok]` | `parser::parse_records` does not misread a description beginning with hex letters as a row | GM CAN mode $06 data layout | `tests/parser.rs::description_starting_with_hex_letters_is_not_misread_as_a_row` |
| `[ok]` | `parser::parse_records` skips page furniture and provenance comments | GM CAN mode $06 data layout | `tests/parser.rs::boilerplate_and_comments_are_skipped` |
| `[ok]` | `parser::parse_section` names the monitor from the section heading | `gm:mode6-can` section headings | `tests/parser.rs::parse_section_names_the_monitor_from_the_heading` |
| `[ok]` | `parser::parse_document` splits the whole file into monitor sections, tagging every row | `gm:mode6-can` monitor headings | `tests/parser.rs::parse_document_splits_the_document_into_monitor_sections` |
| `[ok]` | `parser::parse_document` carries the monitor across page-break furniture instead of starting a section | GM CAN mode $06 data page breaks | `tests/parser.rs::a_page_break_header_does_not_start_a_new_section` |
| `[ok]` | `parser::parse_document` keeps only the monitor name when a heading line trails a legend | `gm:mode6-can` heading lines | `tests/parser.rs::a_heading_that_trails_a_legend_keeps_only_the_monitor_name` |
| `[ok]` | `parser::group_by_obdmid` groups records by OBD Monitor ID | GM CAN mode $06 data | `tests/parser.rs::the_sample_is_all_obdmid_01` |
| `[ok]` | `parser::lookup` filters records by OBDMID and optional TID | GM CAN mode $06 data | `tests/parser.rs::lookup_filters_by_obdmid_and_tid` |
| `[ok]` | the cell splitter keeps a slash-unit resolution apart from the range | GM CAN mode $06 data | `tests/parser.rs::a_slash_unit_resolution_is_split_from_the_range` |

## The record model

| Status | Obligation | Anchor | Test |
|---|---|---|---|
| `[ok]` | `model::Mode06Record::value_range` parses the min, max and unit of a clean range cell | ISO 15031-5 value semantics (GM's printed cells) | `tests/model.rs::value_range_parses_min_max_and_unit` |
| `[ok]` | `model::Mode06Record::scaling` parses the per-bit resolution and unit | ISO 15031-5 value semantics (GM's printed cells) | `tests/model.rs::scaling_parses_per_bit_and_unit` |
| `[ok]` | `model::Mode06Record::value_range` reads GM's en dash as a minus sign | `gm:mode6-can` range cells | `tests/model.rs::an_en_dash_is_read_as_a_minus_sign` |
| `[ok]` | `model::Mode06Record::scaling` keeps a unit that contains a slash (mV/s) | `gm:mode6-can` resolution cells | `tests/model.rs::a_unit_with_a_slash_survives_the_scaling_parse` |
| `[ok]` | `model::Mode06Record::uasid_hex` renders a blank UASID as `None` | GM CAN mode $06 data | `tests/model.rs::a_blank_uasid_renders_as_none` |
| `[ok]` | `model::Mode06Record::value_range` returns `None` for a cell too wrapped to split | GM CAN mode $06 data | `tests/model.rs::a_wrapped_cell_returns_none_not_a_guess` |
| `[ok]` | every continuation line after a row is kept verbatim on `Mode06Record::continuations` (lossless) | GM CAN mode $06 data layout | `tests/parser.rs::a_primary_description_is_kept_and_the_second_definition_is_preserved_raw` |
| `[ok]` | a row whose ID line has no description adopts one from its own continuation, marked `Reconstructed` | GM CAN mode $06 data lines 20-21 | `tests/parser.rs::a_bare_id_row_adopts_its_continuation_as_a_reconstructed_description` |
| `[ok]` | a wrapped range/resolution is filled from a continuation without altering the description | GM CAN mode $06 data lines 36-37 | `tests/parser.rs::a_wrapped_range_is_filled_from_a_continuation_without_touching_the_description` |
| `[ok]` | a row with no description on its ID line or any continuation is marked `Missing`, not guessed | GM CAN mode $06 data | `tests/parser.rs::a_row_with_no_description_anywhere_is_marked_missing` |
| `[ok]` | `parser::reconstructed` lists the low-confidence (reconstructed) rows for review | docs/extraction-limits.md | `tests/parser.rs::reconstructed_lists_only_the_low_confidence_rows` |

## OBD groups and the embedded sample

| Status | Obligation | Anchor | Test |
|---|---|---|---|
| `[ok]` | `groups::files_for_group` returns a group's own files plus the generic ones | docs/sources.md | `tests/groups.rs::files_for_a_group_include_the_group_specific_and_the_generic` |
| `[ok]` | `groups::volt_files` selects the Group 1A files for the Chevrolet Volt | docs/sources.md | `tests/groups.rs::volt_files_match_group_1a` |
| `[ok]` | `catalog::sample_records` parses the embedded GM CAN sample into records | docs/sources.md | `tests/catalog.rs::the_sample_parses_to_records` |

## The read-only command line

| Status | Obligation | Anchor | Test |
|---|---|---|---|
| `[ok]` | `cli::run` looks a record up by OBDMID and TID | this crate | `tests/cli.rs::lookup_by_obdmid_and_tid` |
| `[ok]` | `cli::run` prints the OBD-group mapping and the Volt selection | this crate | `tests/cli.rs::groups_shows_the_volt_selection` |
| `[ok]` | `cli::run` parses a local extraction named with `--source` | this crate | `tests/cli.rs::source_reads_a_local_file` |

## Deeper enumeration (not built)

| Status | Obligation | Anchor | Test |
|---|---|---|---|
| `[open]` | Stitch a description fragment stranded on a neighbouring row's ID line (cross-row): the flattened text offsets the ID and description columns, so this cannot be resolved without the source PDF. Blocked on a page/PDF back-reference from `primarysource` (see `docs/extraction-limits.md`). | GM CAN mode $06 data lines 36-38 | none |
| `[open]` | Parse the FOOTNOTES section bodies (model-year / vehicle applicability per marker 1..20) | `gm:mode6-can§footnotes` | none |
| `[open]` | Parse the legacy `GM Class2 mode $06 data` document (different layout) | GM Class2 mode $06 data | none |
| `[open]` | Expand each UASID to its ISO 15031-5 unit-and-scaling definition | ISO 15031-5 (not loaded in primarysource) | none |

**Non-goal (not a row).** Decoding a live Mode $06 response PDU against these definitions is out of
scope: this crate parses GM's documents and opens no bus. The end-to-end live Rust diagnostic tool
consumes this crate as its Mode $06 parser and data source; the UDS/GMLAN codec is the sibling
`udsdiag`, and this crate does not depend on it.
