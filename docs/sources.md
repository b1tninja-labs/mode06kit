# Sources

`mode06kit` parses GM's own published Mode $06 (On-Board Monitoring Test Results) documents. Two
kinds of source feed it: the GM documents themselves (read through the `primarysource` `gm_*`
tools) and the OBD-group catalog context recorded in the `volt` assessment write-ups. Nothing here
was produced against a live vehicle; the package opens no bus and writes nothing.

## GM's published Mode $06 documents

- **`GM CAN mode $06 data final_dm.pdf`** — the generic Mode $06 monitor/test-ID / unit-and-scaling
  definitions for CAN/GMLAN vehicles. This is the document `parser.py` parses. Read through the
  `primarysource` `gm_*` tools as document **`mode6-can`** (*"GM Mode $06 data definitions (CAN)"*).
  - Columns, left to right: **OBDMID** (OBD Monitor ID), **TID** (Test ID, optionally with a `(n)`
    footnote marker), **UASID** (unit-and-scaling ID, "For Information ONLY"), **description**,
    **units and range**, **resolution**.
  - The **FOOTNOTES** section (markers 1..20) records model-year / vehicle-specific caveats (e.g.
    marker 6: *"OBDMID $31 TID $A9 test limits and test value should be multiplied by 4"*; marker
    16: *"All 2017 model year and beyond"*). `parser.py` captures the marker number; parsing the
    footnote bodies is a `ROADMAP.md` `[ ]` item.
  - Cite: `gm_cite(mode6-can, mon-oxygen-sensor-monitor-bank-1-sensor-1)` →
    *"GM Mode $06 data definitions (CAN) §mon-oxygen-sensor-monitor-bank-1-sensor-1; local copy of
    GM CAN mode $06 data final_dm.pdf; Heading: Oxygen Sensor Monitor Bank 1 Sensor 1"*.
- **`GM Class2 mode $06 data final_dm.pdf`** — the same definitions for legacy Class 2 (pre-CAN GM
  bus) vehicles. Catalogued in `groups.py`; parsing it is a `ROADMAP.md` `[ ]` item.
- **`2017_OBD_GROUP_SUMMARY.pdf`** and the per-group `17OBDGxx …` files — the OBD-group matrix and
  the per-group diagnostic-parameter files. Catalogued (filename / group / description) in
  `groups.py`; their contents are out of scope here.

### Bundled sample

`mode06kit/data/gm_can_mode06_sample.txt` is a representative single-monitor excerpt (the *Oxygen
Sensor Monitor Bank 1 Sensor 1* section) of the CAN document, read through the `gm_*` tools and
carried with a provenance header so the CLI and tests parse something real without a local copy of
the full PDF. Point `mode06kit --source` at a full text extraction to parse the whole document.

## OBD-group catalog context

The OBD-group → applicable-file mapping and the Volt = **Group 1A** selection (`groups.py`) are
re-encoded from the read-only `volt` write-up `D:\code\volt\docs\GM-OBD-REFERENCES.md`. GM hosts the
documents at `https://gsitlc.ext.gm.com/gmspo/mode6/pdf/`. This package re-encodes that catalog and
does **not** depend on the sibling `udsdiag` (whose `mode06.py` carries the same catalog on the
application-diagnostics side).

## Scope boundary

This package parses GM's documents into a typed model and reads them back. It does **not** open a
CAN interface or any transport, decode a live Mode $06 response, or write to a vehicle. The
standardized Mode $06 service semantics are never stated from memory: they come from GM's own text
read through `primarysource`, with the ISO status recorded in [`iso-map.md`](iso-map.md).
