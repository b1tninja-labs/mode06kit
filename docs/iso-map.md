# ISO / standard map

Every standards claim this crate makes is anchored to a source read through the `primarysource` MCP
tools, never stated from memory (a `D:\code\AGENTS.md` rule). This file is the index: which source
backs which claim, and which claims have **no** primary source loaded.

## Confirmed against a loaded source

| Claim | Where used | Source | Where |
|---|---|---|---|
| A Mode $06 row is addressed by OBD Monitor ID (OBDMID) and Test ID (TID); the UASID is the unit-and-scaling ID, "For Information ONLY" | `model.rs`, `parser.rs` | GM's own CAN Mode $06 document | `gm:mode6-can` column headers and rows (e.g. §`mon-oxygen-sensor-monitor-bank-1-sensor-1`) |
| Footnote markers 1..20 are model-year / vehicle caveats on specific OBDMID/TID pairs | `parser.rs` (marker only) | GM's own CAN Mode $06 document | `gm:mode6-can§footnotes` |
| Mode $06 is the generic OBD-II On-Board Monitoring Test Results service of SAE J1979 / ISO 15031-5 | crate docs | GM's own text, corroborated by `volt/docs/GM-OBD-REFERENCES.md` | named there; the GM files supply the manufacturer-specific IDs |

## Not in `primarysource` — handled conservatively

| Claim | Impact | Would-be source | Status |
|---|---|---|---|
| The ISO 15031-5 UASID → (unit, scaling, offset) table | The UASID is kept as a raw byte, **not** expanded to a unit/scaling | ISO 15031-5 | **Not loaded.** `iso_docs` lists ISO 15031-3:2004 but not 15031-5. Expanding the UASID is a `ROADMAP.md` open row, blocked on the document. The range/resolution that `model::ValueRange` / `model::Scaling` parse come from GM's own printed cells, not from the ISO UASID table. |
| SAE J1979 as the normative service definition | Referenced by name only | SAE J1979 | Not loaded; not quoted. GM's document is the working source. |

If ISO 15031-5 is later fetched into `primarysource`, read it, expand the UASID table, and replace
the UASID open row in `ROADMAP.md` with a clause citation here.

## Related loaded sources (for orientation, not used here)

`iso_docs` shows ISO 14229-1:2013 (UDS), ISO 15765-2:2016 (DoCAN / ISO-TP) and ISO 15031-3:2004
(diagnostic connector) are loaded. Those back the sibling `udsdiag` and `isotpkit`, not this crate,
which stays on the Mode $06 document surface.
