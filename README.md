# mode06kit

Parse GM's own published **Mode $06** (On-Board Monitoring Test Results, SAE J1979 / ISO 15031-5)
documents into a typed model of OBD Monitor IDs, Test IDs, unit-and-scaling IDs and value ranges.

Mode $06 is the generic OBD-II service that reports on-board monitor test results. Each result is
addressed by an **OBD Monitor ID (OBDMID)** and a **Test ID (TID)**, and its value is interpreted
through a **unit-and-scaling ID (UASID)** and min/max test limits. Manufacturers publish the
monitor/test-ID definitions for the independent aftermarket; `mode06kit` reads GM's own
`GM CAN mode $06 data` reference and turns its monitor/test-ID tables into structured records.

It reads GM's published text and models it. It opens no bus, sends no frame, and writes nothing to a
vehicle. The application-layer sibling [`udsdiag`](../udsdiag) only *catalogs* these documents
(filename → OBD group); `mode06kit` is the *parser* of their contents and does not depend on it.

## Install

```
pip install -e .
```

## Use

```
mode06kit records              # list the parsed records from the bundled GM CAN sample
mode06kit lookup 01            # every TID under OBDMID $01
mode06kit lookup 01 04         # OBDMID $01, TID $04
mode06kit monitors             # the monitor headings found and their row counts
mode06kit groups               # the OBD-group document mapping and the Volt (Group 1A) selection
mode06kit records --source extracted.txt   # parse a full local text extraction instead of the sample
```

As a library:

```python
from mode06kit import catalog, parser

records = catalog.sample_records()
hit = next(parser.lookup(records, 0x01, 0x04))
print(hit.obdmid_hex, hit.tid_hex, hit.description, hit.value_range, hit.scaling)
# $01 $04 Lean to Rich Sensor Switch Time ValueRange(0.0, 65535.0, 'ms') Scaling(1.0, 'ms')
```

## What is in this slice

The bundled sample is one representative monitor section of the CAN document, carried with a
provenance header (`mode06kit/data/`), so the tools work out of the box; point `--source` at a full
extraction for the whole file. The reliable core is the three leading identifiers plus the footnote
marker; GM's heavily wrapped descriptions and the ISO 15031-5 UASID → unit expansion are tracked as
not-built items in [`ROADMAP.md`](ROADMAP.md). Sources and the ISO status are in
[`docs/sources.md`](docs/sources.md) and [`docs/iso-map.md`](docs/iso-map.md); scope is in
[`PLAN.md`](PLAN.md).

## Scope

Defensive / interoperability research on owned hardware. GM's Mode $06 documents are the owner's
licensed copies. This package parses documents; it does not touch a vehicle.

Apache-2.0.
