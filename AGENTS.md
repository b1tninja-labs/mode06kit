# Instructions for coding agents

`mode06kit` parses GM's own published Mode $06 (On-Board Monitoring Test Results, SAE J1979 /
ISO 15031-5) documents into a typed model. `PLAN.md` says what is in this slice, `ROADMAP.md` is the
work queue, and `docs/sources.md` records the GM documents the parser follows. This file takes
precedence over `D:\code\AGENTS.md` for work in this project.

## Commands

Run from this directory, never from `D:\code`:

    python -m pytest -q
    python -O -m pytest -q
    python -m ruff format mode06kit tests
    python -m ruff check mode06kit tests
    python -m mypy mode06kit

Use the pinned toolchain in `projecthub/src/projecthub/shells/requirements-dev.txt` when it is
installed.

## Invariants

1. **The matrix and the code agree.** A `ROADMAP.md` row is `[ok]` only when it names a callable and
   a test that exist. `tests/test_roadmap_rows.py` checks it. A `[ ]` / `[no]` row names no callable.
2. **Nothing from memory.** Every parsed value traces to GM's own text read through the
   `primarysource` `gm_*` tools (document `mode6-can`). Do not state a Mode $06 / PID / scaling rule
   from memory; read it first. ISO status is tracked in `docs/iso-map.md`. ISO 15031-5 is **not**
   loaded, so the UASID stays a raw byte — do not invent its unit table.
3. **The parser is fed text, not hardcoded.** `parser.py` operates on text passed to it. The bundled
   `data/gm_can_mode06_sample.txt` is a provenance-headed excerpt actually read through `gm_*`; keep
   that header, and do not paste GM values straight into the model.
4. **No vehicle contact.** The package parses documents. It opens no bus, decodes no live response
   PDU, and writes nothing to a vehicle.
5. **No dependency on `udsdiag`.** The OBD-group catalog is re-encoded here. `udsdiag.mode06` only
   catalogs the documents; this package parses their contents, and the two stay independent.
