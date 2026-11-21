"""Parse the text of GM's ``GM CAN mode $06 data`` reference into :class:`Mode06Record` values.

GM's document is a column layout flattened to text: each data row leads with the OBDMID, the TID
(optionally carrying a ``(n)`` footnote marker) and the UASID, then the description, value range
and resolution cells, which the PDF separates by runs of whitespace. Description cells sometimes
wrap onto the next line; those wrapped continuations have no leading IDs and are left out of the
structured record (the reliable core is the three IDs and the footnote -- see ``ROADMAP.md`` for
the deeper description-reconstruction work that is deliberately not built yet).

The parser only ever runs on text the caller read out of GM's own document; it invents nothing.
"""

from __future__ import annotations

import re
from collections.abc import Iterable, Iterator

from mode06kit.model import EN_DASH, Mode06Record, Scaling, ValueRange

# A data row: OBDMID, TID (+ optional "(n)" footnote), UASID, then the rest of the row.
_ROW = re.compile(
    r"^\s*"
    r"(?P<obdmid>[0-9A-Fa-f]{2})\s+"
    r"(?P<tid>[0-9A-Fa-f]{2})"
    r"(?:\s*\((?P<footnote>\d+)\))?"
    r"\s+(?P<uasid>[0-9A-Fa-f]{2})"
    r"(?:\s+(?P<rest>.*\S))?"
    r"\s*$"
)

# Page furniture and our own provenance comments: never rows, never monitor headings.
_BOILERPLATE = re.compile(
    r"^\s*(?:#"
    r"|\[page\b"
    r"|GM mode \$06 data definitions"
    r"|Some items have footnotes"
    r"|FOOTNOTES\b"
    r"|Footnotes are explained"
    r"|GMLAN rev\d+\s*$"
    r")"
)

_CELLS = re.compile(r"\s{2,}")
# The same cell shapes the model parses, used here to classify a cell during the row split.
_RANGE_CELL = re.compile(rf"[-+{EN_DASH}]?\d+(?:\.\d+)?\s+to\s+[-+{EN_DASH}]?\d+(?:\.\d+)?\b")
_SCALING_CELL = re.compile(r"\d+(?:\.\d+)?\s*.*?/\s*bit\s*$")


def _is_boilerplate(line: str) -> bool:
    return bool(_BOILERPLATE.match(line))


def _split_rest(rest: str) -> tuple[str, str, str]:
    """Split a row's trailing text into (description, unit_range, resolution) by its column gaps."""
    description: list[str] = []
    unit_range = ""
    resolution = ""
    for cell in _CELLS.split(rest.strip()):
        if not cell:
            continue
        if not resolution and _SCALING_CELL.search(cell):
            resolution = cell.strip()
        elif not unit_range and _RANGE_CELL.search(cell):
            unit_range = cell.strip()
        else:
            description.append(cell.strip())
    return " ".join(description), unit_range, resolution


def _record(match: re.Match[str], monitor: str | None, line_no: int) -> Mode06Record:
    footnote = match["footnote"]
    description, unit_range, resolution = _split_rest(match["rest"] or "")
    return Mode06Record(
        obdmid=int(match["obdmid"], 16),
        tid=int(match["tid"], 16),
        uasid=int(match["uasid"], 16),
        description=description,
        unit_range=unit_range,
        resolution=resolution,
        footnote=int(footnote) if footnote is not None else None,
        monitor=monitor,
        line=line_no,
    )


def parse_records(text: str, monitor: str | None = None) -> list[Mode06Record]:
    """Parse every data row in ``text`` into records, all tagged with ``monitor`` (default ``None``)."""
    records: list[Mode06Record] = []
    for line_no, line in enumerate(text.splitlines(), start=1):
        if _is_boilerplate(line):
            continue
        match = _ROW.match(line)
        if match is not None:
            records.append(_record(match, monitor, line_no))
    return records


def parse_section(text: str) -> list[Mode06Record]:
    """Parse one monitor section: the first non-row, non-boilerplate line names the monitor.

    This matches how a single section reads out of the GM document (the monitor heading, then its
    rows). For a whole multi-section document, run this per section; whole-document segmentation is
    a ``ROADMAP.md`` ``[ ]`` item.
    """
    monitor: str | None = None
    records: list[Mode06Record] = []
    for line_no, line in enumerate(text.splitlines(), start=1):
        if not line.strip() or _is_boilerplate(line):
            continue
        match = _ROW.match(line)
        if match is None:
            if monitor is None:
                monitor = line.strip()
            continue
        records.append(_record(match, monitor, line_no))
    return records


def by_obdmid(records: Iterable[Mode06Record]) -> dict[int, list[Mode06Record]]:
    """Group records by OBDMID, preserving the order they were parsed in."""
    grouped: dict[int, list[Mode06Record]] = {}
    for record in records:
        grouped.setdefault(record.obdmid, []).append(record)
    return grouped


def lookup(records: Iterable[Mode06Record], obdmid: int, tid: int | None = None) -> Iterator[Mode06Record]:
    """Yield records matching ``obdmid`` (and ``tid`` when given)."""
    for record in records:
        if record.obdmid != obdmid:
            continue
        if tid is not None and record.tid != tid:
            continue
        yield record


__all__ = [
    "Mode06Record",
    "Scaling",
    "ValueRange",
    "by_obdmid",
    "lookup",
    "parse_records",
    "parse_section",
]
