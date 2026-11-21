# mode06kit — plan

## What this is

A parser for GM's own published **Mode $06** (On-Board Monitoring Test Results, SAE J1979 /
ISO 15031-5) documents. It turns the monitor/test-ID tables in GM's `GM CAN mode $06 data`
reference into a typed model (`Mode06Record`: OBDMID, TID, UASID, description, range, resolution,
footnote marker, monitor), and offers a read-only CLI over the result.

It is the *parser* counterpart to the sibling `udsdiag`, whose `mode06.py` only *catalogs* these
documents (filename → OBD group). `mode06kit` does not depend on `udsdiag`.

## Why these sources

The Chevrolet Volt is **OBD Group 1A**, so the applicable GM files are `17OBDG01 Hybrid
Diagnostics` (group-specific) and the generic `GM CAN mode $06 data` reference (all CAN/GMLAN
vehicles). This package parses the generic CAN reference, which is the one that carries the
monitor/test-ID / unit-and-scaling definitions. The OBD-group → file mapping and the Volt selection
are in `groups.py`. See `docs/sources.md`.

## In this slice (the tested core)

- `model.py` — `Mode06Record` and the `ValueRange` / `Scaling` parsers for the clean cells, keeping
  GM's raw text and returning `None` rather than guessing when a cell is too wrapped to split.
- `parser.py` — `parse_records` / `parse_section`: robust extraction of the leading OBDMID / TID /
  UASID and the footnote marker, best-effort split of the trailing description / range / resolution
  cells, and `by_obdmid` / `lookup` helpers.
- `groups.py` — the OBD-group document catalog and the Volt (Group 1A) selection.
- `catalog.py` + `data/gm_can_mode06_sample.txt` — a provenance-headed single-monitor excerpt so
  the CLI and tests parse real GM text out of the box.
- `cli.py` — read-only `records` / `lookup` / `monitors` / `groups` sub-commands.

## Deliberately not in this slice

Tracked as `[ ]` / `[no]` rows in `ROADMAP.md`:

- Whole-document monitor segmentation (every section at once), not one section at a time.
- Reconstructing GM's wrapped multi-line descriptions and the second definition stacked in a row.
- Parsing the FOOTNOTES bodies (year/vehicle applicability per marker) — only the marker number is
  captured now.
- Parsing the legacy `GM Class2 mode $06 data` document (different layout).
- Expanding each UASID to its ISO 15031-5 unit-and-scaling definition — **blocked**: ISO 15031-5 is
  not loaded in `primarysource` (`docs/iso-map.md`).
- Decoding a live Mode $06 response PDU — **non-goal**: this package reads documents; the codec is
  the sibling `udsdiag`.

## Invariants

1. **The matrix and the code agree.** A `ROADMAP.md` row is `[ok]` only when it names a callable and
   a test that both exist. `tests/test_roadmap_rows.py` checks it.
2. **Nothing from memory.** Every value comes from text read through the `primarysource` `gm_*`
   tools; no Mode $06 / PID / scaling rule is stated from memory (a `D:\code\AGENTS.md` rule).
3. **No vehicle contact.** The package parses documents. It opens no bus, decodes no live PDU, and
   writes nothing to a vehicle.
4. **No dependency on `udsdiag`.** The OBD-group catalog is re-encoded here from `docs/sources.md`.
