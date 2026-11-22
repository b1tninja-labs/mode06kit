# mode06kit

Parse GM's own published **Mode $06** (On-Board Monitoring Test Results, SAE J1979 / ISO 15031-5)
OBD documents into a typed model of OBD Monitor IDs, Test IDs, unit-and-scaling IDs and value
ranges. A Rust crate (edition 2024, MSRV 1.85).

Mode $06 is the generic OBD-II service that reports on-board monitor test results. Each result is
addressed by an **OBD Monitor ID (OBDMID)** and a **Test ID (TID)**, and its value is interpreted
through a **unit-and-scaling ID (UASID)** and min/max test limits. Manufacturers publish the
monitor/test-ID definitions for the independent aftermarket; `mode06kit` reads GM's own
`GM CAN mode $06 data` reference and turns its monitor/test-ID tables into structured records.

It reads GM's published text and models it. It opens no bus, sends no frame, and writes nothing to a
vehicle. It is the Mode $06 parser and data source for the end-to-end live diagnostic tool; the
UDS/GMLAN codec is the sibling `udsdiag`, and this crate does not depend on it.

## Use

```
cargo run -- records              # list the parsed records from the embedded GM CAN sample
cargo run -- lookup 01            # every TID under OBDMID $01
cargo run -- lookup 01 04         # OBDMID $01, TID $04
cargo run -- monitors             # the monitor headings found and their row counts
cargo run -- footnotes --source extracted.txt   # the FOOTNOTES section bodies (marker 1..20)
cargo run -- groups               # the OBD-group document mapping and the Volt (Group 1A) selection
cargo run -- records --source extracted.txt   # parse a full local text extraction instead of the sample
```

As a library:

```rust
use mode06kit::{catalog, parser};

let records = catalog::sample_records();
let hit = parser::lookup(&records, 0x01, Some(0x04));
let record = hit[0];
println!("{} {} {} {:?} {:?}", record.obdmid_hex(), record.tid_hex(), record.description, record.value_range(), record.scaling());
// $01 $04 Lean to Rich Sensor Switch Time Some(ValueRange { minimum: 0.0, maximum: 65535.0, unit: "ms" }) Some(Scaling { per_bit: 1.0, unit: "ms" })
```

## What is in this slice

The embedded sample is one representative monitor section of the CAN document, carried with a
provenance header (`src/data/`), so the tools work out of the box; point `--source` at a full
extraction for the whole file, which `parser::parse_document` splits into its monitor sections
(`Oxygen Sensor Monitor Bank 1 Sensor 3`, `EGR Monitor Bank 1`, `Catalyst Monitor Bank 1`, …),
tagging each record with the monitor it falls under. Each record also keeps the raw continuation
lines that follow it (`continuations`); a row whose description wrapped onto a later line adopts it
as a best-effort, confidence-tracked join (`description_source`), never stitching text across rows
(`mode06kit review` lists the low-confidence joins; [`docs/extraction-limits.md`](docs/extraction-limits.md)
explains the limit). The reliable core is the three leading
identifiers plus the footnote
marker; GM's heavily wrapped descriptions and the ISO 15031-5 UASID → unit expansion are tracked as
not-built rows in [`ROADMAP.md`](ROADMAP.md). Sources and the ISO status are in
[`docs/sources.md`](docs/sources.md) and [`docs/iso-map.md`](docs/iso-map.md).

## Develop

```
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo doc --no-deps
```

Minimum supported Rust version: 1.85 (see `docs/msrv.md`). Contributing: `CONTRIBUTING.md`.

## Scope

Defensive / interoperability research on owned hardware. GM's Mode $06 documents are the owner's
licensed copies. This crate parses documents; it does not touch a vehicle. Apache-2.0.
