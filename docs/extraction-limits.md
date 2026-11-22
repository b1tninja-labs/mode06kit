# Extraction limits: wrapped descriptions in GM's flattened Mode $06 tables

GM's `GM CAN mode $06 data` document is a multi-column table. Once flattened to text (the form
`mode06kit` parses, and the form the `primarysource` `gm_*` tools return), the table's columns are
**vertically offset**: the ID column and the description column do not share a baseline, so a
description fragment can land on the *text line of a different row*. This file records what that
means for reconstruction, with lines cited from the document (read through `gm_*`, document
`mode6-can`), and what `mode06kit` does about it.

## What the flattening does

Three row shapes appear, all from `gm:mode6-can`:

1. **Definition wrapped entirely onto the next line** (lines 20-21): the ID line carries only the
   identifiers, and the description, range and resolution are on the following line.

   ```
   20: 01        03       0A
   21:                      Rich to Lean Sensor Switch Time        0 to 65535 ms     1 ms / bit
   ```

2. **A stacked second definition** (lines 16-17): one `(OBDMID, TID)` prints two full definitions,
   the second on a line with no identifiers. It is a different test, not a continuation.

   ```
   16: 01   01   0A   Rich to Lean Sensor Threshold Voltage   0.0000 to 7.9900 Volts   0.122 mV / bit
   17:                Lean to Rich Sensor Threshold Voltage   0.0000 to 7.9900 Volts   0.122 mV / bit
   ```

3. **A fragment stranded on a neighbouring row's ID line** (lines 36-38): the description
   "Particulate Matter Sensor Electrode Current value during **regeneration**" is split, and the
   word `regeneration` sits on line 38 — which carries its *own* different row's identifiers
   (`01-80` UASID `2B`, footnote 10). The range for line 36 is stranded on line 37.

   ```
   36: 01   80   16   Particulate Matter Sensor Electrode Current value during        0.01 uAmps / bit
   37:                                                             0 to 655.35 uAmps
   38: 01   80 (10) 2B                regeneration                                    0.001 / bit
   ```

Shape 3 is the hard case: from the flattened text alone there is no reliable way to know that
`regeneration` belongs to the line-36 row rather than the line-38 row. The association was carried by
x-position in the PDF, which the flattening dropped.

## What `mode06kit` does (and does not) reconstruct

- **Lossless capture.** Every non-row line that follows a row (before the next row) is kept verbatim
  on `Mode06Record::continuations`, in document order. Nothing is silently dropped.
- **Best-effort fill, from the row's own continuations only.** A row's empty description, range or
  resolution is filled from its following continuation lines (shapes 1 and the stranded range in 3).
  A reconstructed description is marked `DescriptionSource::Reconstructed` — a low-confidence join,
  because the fragment could instead be a stacked second definition (shape 2).
- **No cross-row stitching.** The parser never pulls text off a *different* row's ID line, so it does
  **not** fabricate "…value during regeneration" from line 38. The description stays the honest,
  possibly truncated, line-36 text (`DescriptionSource::Primary`). This is the deliberate limit: a
  guess that reads well but is wrong is worse than a short true string (`AGENTS.md`, invariant 1).

Run `mode06kit review --source <extraction>` to list the reconstructed (low-confidence) rows with
their raw continuation lines — the rows worth checking against the PDF.

## The open item: page / PDF back-reference

Fully resolving shape 3 needs the real document, not the flattened text: a page number or a handle to
the source PDF so a reader (or a layout-aware re-extraction) can recover the column geometry. That is
a request on the `primarysource` side — have the `gm_*` tools optionally return a page reference /
PDF handle per section or line, and look into a layout-preserving table extraction. Until then,
`ROADMAP.md` keeps cross-row fragment stitching as an open row, and the `review` command plus this
file mark exactly which rows need it.
