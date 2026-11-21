# Instructions for coding agents

`mode06kit` is a Rust crate (edition 2024, MSRV 1.85) that parses GM's own published Mode $06
(On-Board Monitoring Test Results, SAE J1979 / ISO 15031-5) documents into a typed model. It is the
Mode $06 parser and data source for the end-to-end live diagnostic tool. `ROADMAP.md` is the work
queue (Rust `Status | Obligation | Anchor | Test` matrix), and `docs/sources.md` records the GM
documents the parser follows. Read `docs/layout.md` before adding modules. This file takes
precedence over `D:\code\AGENTS.md` for work in this project.

## Commands

Run these from this directory; all must pass before a change is done.

    cargo fmt --check
    cargo clippy --all-targets -- -D warnings
    cargo test
    cargo doc --no-deps

`python -m projecthub shells verify --rust D:/code/mode06kit` runs them in one pass. CI also builds
on 1.85, so the MSRV is tested.

## Invariants

1. **Nothing from memory.** Every parsed value traces to GM's own text read through the
   `primarysource` `gm_*` tools (document `mode6-can`). Do not state a Mode $06 / PID / scaling rule
   from memory; read it first (a `D:\code\AGENTS.md` rule). ISO status is tracked in
   `docs/iso-map.md`. ISO 15031-5 is **not** loaded, so the UASID stays a raw byte — do not invent
   its unit table.
2. **The parser is fed text, not hardcoded.** `parser.rs` operates on text passed to it. The
   embedded `src/data/gm_can_mode06_sample.txt` is a provenance-headed excerpt actually read through
   `gm_*`; keep that header, and do not paste GM values straight into the model.
3. **No vehicle contact.** The crate parses documents. It opens no bus, decodes no live response
   PDU, and writes nothing to a vehicle. Decoding a live PDU is a non-goal (see `ROADMAP.md`).
4. **No dependency on `udsdiag`.** The OBD-group catalog is re-encoded here. The two stay
   independent.
5. `unsafe` is forbidden by the lint table in `Cargo.toml`; do not relax it. Do not add a dependency
   without saying why in the commit message. Do not raise `rust-version` without updating
   `clippy.toml`, `docs/msrv.md` and CI together.
6. Never run `cargo fmt` or `cargo clippy --fix` outside this directory. Write scratch output outside
   the repository. Commit messages carry no AI attribution trailer.
