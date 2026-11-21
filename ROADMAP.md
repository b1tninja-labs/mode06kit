# Obligations and their evidence

Every obligation `mode06kit` has taken on, with an honest status against the code.

**The honesty rule:** a row reads `[ok]` only when the test named in its Evidence cell exists and would fail if
the behaviour regressed. `tests/test_roadmap_rows.py` resolves every `[ok]` row's test and every callable the
matrix names.

Legend: `[ok]` implemented and pinned by a named test — `[part]` partial, what is missing stated — `[ ]` not
built — `[no]` deliberate non-goal, with the reason.

The source is GM's own `GM CAN mode $06 data` reference, read through the `primarysource` `gm_*` tools and
recorded in `docs/sources.md`. The standardized Mode $06 semantics (OBDMID/TID structure, the UASID is
"For Information ONLY") are GM's text; ISO 15031-5 itself is **not** loaded in `primarysource`, so the UASID is
kept as a raw byte rather than expanded to a unit (`docs/iso-map.md`).

## 1. Matrix

### Parsing the monitor/test-ID rows

| Id | Obligation | Spec | Status | Evidence |
|---|---|---|---|---|
| `m6.parser.ids` | `parser.parse_records()` extracts the OBDMID, TID and UASID from a data row | GM CAN mode $06 data | `[ok]` | `tests/test_parser.py::test_a_clean_row_yields_the_three_leading_ids` |
| `m6.parser.footnote` | `parser.parse_records()` captures the `(n)` footnote marker on the TID | GM CAN mode $06 data | `[ok]` | `tests/test_parser.py::test_a_tid_footnote_marker_is_captured` |
| `m6.parser.wrap` | `parser.parse_records()` leaves a wrapped continuation line out of the records | GM CAN mode $06 data | `[ok]` | `tests/test_parser.py::test_a_wrapped_continuation_line_is_not_a_record` |
| `m6.parser.hexguard` | `parser.parse_records()` does not misread a description beginning with hex letters as a row | GM CAN mode $06 data | `[ok]` | `tests/test_parser.py::test_description_starting_with_hex_letters_is_not_misread_as_a_row` |
| `m6.parser.boiler` | `parser.parse_records()` skips page furniture and provenance comments | GM CAN mode $06 data | `[ok]` | `tests/test_parser.py::test_boilerplate_and_comments_are_skipped` |
| `m6.parser.section` | `parser.parse_section()` names the monitor from the section heading | GM CAN mode $06 data | `[ok]` | `tests/test_parser.py::test_parse_section_names_the_monitor_from_the_heading` |
| `m6.parser.group` | `parser.by_obdmid()` groups records by OBD Monitor ID | GM CAN mode $06 data | `[ok]` | `tests/test_parser.py::test_the_sample_is_all_obdmid_01` |
| `m6.parser.lookup` | `parser.lookup()` filters records by OBDMID and optional TID | GM CAN mode $06 data | `[ok]` | `tests/test_parser.py::test_lookup_filters_by_obdmid_and_tid` |

### The record model

| Id | Obligation | Spec | Status | Evidence |
|---|---|---|---|---|
| `m6.model.range` | `model.Mode06Record.value_range()` parses the min, max and unit of a clean range cell | ISO 15031-5 value semantics (GM's text) | `[ok]` | `tests/test_model.py::test_value_range_parses_min_max_and_unit` |
| `m6.model.scaling` | `model.Mode06Record.scaling()` parses the per-bit resolution and unit | ISO 15031-5 value semantics (GM's text) | `[ok]` | `tests/test_model.py::test_scaling_parses_per_bit_and_unit` |
| `m6.model.endash` | `model.Mode06Record.value_range()` reads GM's en-dash as a minus sign | GM CAN mode $06 data | `[ok]` | `tests/test_model.py::test_an_en_dash_is_read_as_a_minus_sign` |
| `m6.model.slashunit` | `model.Mode06Record.scaling()` keeps a unit that contains a slash (mV/s) | GM CAN mode $06 data | `[ok]` | `tests/test_model.py::test_a_unit_with_a_slash_survives_the_scaling_parse` |
| `m6.model.blank` | `model.Mode06Record.uasid_hex()` renders a blank UASID as `None`, not `$00` | GM CAN mode $06 data | `[ok]` | `tests/test_model.py::test_a_blank_uasid_renders_as_none` |
| `m6.model.noguess` | `model.Mode06Record.value_range()` returns `None` for a cell too wrapped to split | GM CAN mode $06 data | `[ok]` | `tests/test_model.py::test_a_wrapped_range_cell_returns_none_not_a_guess` |

### OBD groups and the bundled sample

| Id | Obligation | Spec | Status | Evidence |
|---|---|---|---|---|
| `m6.groups.files` | `groups.files_for_group()` returns a group's own files plus the generic ones | docs/sources.md | `[ok]` | `tests/test_groups.py::test_files_for_a_group_include_the_group_specific_and_the_generic` |
| `m6.groups.volt` | `groups.volt_files()` selects the Group 1A files for the Chevrolet Volt | docs/sources.md | `[ok]` | `tests/test_groups.py::test_volt_files_match_group_1a` |
| `m6.catalog.sample` | `catalog.sample_records()` parses the bundled GM CAN sample into records | docs/sources.md | `[ok]` | `tests/test_catalog.py::test_the_sample_parses_to_records` |

### The read-only command line

| Id | Obligation | Spec | Status | Evidence |
|---|---|---|---|---|
| `m6.cli.lookup` | `cli.main()` looks a record up by OBDMID and TID | this package | `[ok]` | `tests/test_cli.py::test_lookup_by_obdmid_and_tid` |
| `m6.cli.groups` | `cli.main()` prints the OBD-group mapping and the Volt selection | this package | `[ok]` | `tests/test_cli.py::test_groups_shows_the_volt_selection` |
| `m6.cli.source` | `cli.main()` parses a local extraction named with `--source` | this package | `[ok]` | `tests/test_cli.py::test_source_reads_a_local_file` |

## 2. Deliberately not built yet

These are the deeper enumeration tasks. They are `[ ]` (not built) or `[no]` (non-goal); none names a callable,
so the matrix does not claim them.

| Id | Obligation | Spec | Status | Evidence |
|---|---|---|---|---|
| `m6.next.segment` | Whole-document monitor segmentation: split the full CAN file into every monitor section, not one at a time | GM CAN mode $06 data | `[ ]` | deferred |
| `m6.next.desc` | Reconstruct wrapped multi-line descriptions and the second definition stacked in a row | GM CAN mode $06 data | `[ ]` | deferred |
| `m6.next.footbody` | Parse the FOOTNOTES section bodies (model-year / vehicle applicability per marker 1..20) | GM CAN mode $06 data | `[ ]` | deferred |
| `m6.next.class2` | Parse the legacy `GM Class2 mode $06 data` document (different layout) | GM Class2 mode $06 data | `[ ]` | deferred |
| `m6.next.uasid` | Expand each UASID to its ISO 15031-5 unit-and-scaling definition | ISO 15031-5 | `[ ]` | blocked: ISO 15031-5 is not loaded in primarysource (docs/iso-map.md) |
| `m6.next.bus` | Decode a live Mode $06 response PDU against these definitions | — | `[no]` | non-goal: this package reads GM's documents; it opens no bus and the codec belongs to the sibling udsdiag |
