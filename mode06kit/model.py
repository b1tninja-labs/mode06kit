"""The typed model for a GM Mode $06 test definition.

A Mode $06 record is one row of GM's monitor/test-ID tables. The reliably structured fields are
the three identifiers GM prints in the leading columns -- the OBD Monitor ID (OBDMID), the Test
ID (TID) and the unit-and-scaling ID (UASID, "For Information ONLY" per ISO 15031-5) -- plus an
optional footnote marker. The human description and the value range/resolution are carried as the
text GM published; :attr:`Mode06Record.value_range` and :attr:`Mode06Record.scaling` parse the
clean cases and return ``None`` when GM's layout is too wrapped to split safely.

Nothing here is stated from memory: a record is only ever produced from text read out of GM's own
document (``docs/sources.md``). The ISO 15031-5 UASID -> unit table is not loaded in
``primarysource`` (``docs/iso-map.md``), so the UASID is kept as its raw byte, not expanded.
"""

from __future__ import annotations

import re
from dataclasses import dataclass

# GM prints some negative limits with an en dash (U+2013) instead of a hyphen-minus.
EN_DASH = "\N{EN DASH}"

# A GM range cell: "0.0000 to 7.9900 Volts", "0 to 65535 ms", "-40 to 6513.5 C",
# "-32.768 to +32.767 kPa" (the leading minus may be an en dash).
_RANGE = re.compile(
    rf"(?P<lo>[-+{EN_DASH}]?\d+(?:\.\d+)?)\s+to\s+(?P<hi>[-+{EN_DASH}]?\d+(?:\.\d+)?)\s*(?P<unit>.*?)\s*$"
)
# A GM resolution cell: "0.122 mV / bit", "1 ms / bit", "2 mV/s / bit", "0.000305 / bit".
_SCALING = re.compile(r"(?P<factor>\d+(?:\.\d+)?)\s*(?P<unit>.*?)\s*/\s*bit\s*$")


def _to_float(token: str) -> float:
    """Parse a GM numeric token, treating the en dash GM uses for minus as a minus sign."""
    return float(token.replace(EN_DASH, "-"))


@dataclass(frozen=True, slots=True)
class ValueRange:
    """The min and max of a test value, in the unit GM named for it."""

    minimum: float
    maximum: float
    unit: str


@dataclass(frozen=True, slots=True)
class Scaling:
    """The per-bit resolution of a test value, in the unit GM named for it."""

    per_bit: float
    unit: str


@dataclass(frozen=True, slots=True)
class Mode06Record:
    """One Mode $06 monitor/test-ID definition read from a GM document."""

    obdmid: int
    """OBD Monitor ID, e.g. 0x01 for Oxygen Sensor Monitor Bank 1 Sensor 1."""
    tid: int
    """Test ID within the monitor, e.g. 0x01."""
    uasid: int | None
    """Unit-and-scaling ID (ISO 15031-5), or ``None`` when GM left the cell blank."""
    description: str
    """The test description GM printed (best effort; may be empty when the cell wrapped away)."""
    unit_range: str
    """The raw range cell, e.g. ``"0.0000 to 7.9900 Volts"`` (empty when none was on the row)."""
    resolution: str
    """The raw resolution cell, e.g. ``"0.122 mV / bit"`` (empty when none was on the row)."""
    footnote: int | None
    """The footnote marker on the TID, e.g. 1, or ``None``. Footnotes are model-year caveats."""
    monitor: str | None
    """The monitor section heading this row fell under, or ``None`` when parsed without one."""
    line: int
    """1-based line number of the row within the parsed text, for citation back to the source."""

    @property
    def obdmid_hex(self) -> str:
        """The OBDMID in GM's ``$HH`` notation."""
        return f"${self.obdmid:02X}"

    @property
    def tid_hex(self) -> str:
        """The TID in GM's ``$HH`` notation."""
        return f"${self.tid:02X}"

    @property
    def uasid_hex(self) -> str | None:
        """The UASID in GM's ``$HH`` notation, or ``None`` when blank."""
        return None if self.uasid is None else f"${self.uasid:02X}"

    @property
    def value_range(self) -> ValueRange | None:
        """The parsed min/max/unit of :attr:`unit_range`, or ``None`` if it does not parse cleanly."""
        match = _RANGE.fullmatch(self.unit_range.strip())
        if match is None:
            return None
        return ValueRange(_to_float(match["lo"]), _to_float(match["hi"]), match["unit"].strip())

    @property
    def scaling(self) -> Scaling | None:
        """The parsed per-bit resolution of :attr:`resolution`, or ``None`` if it does not parse cleanly."""
        match = _SCALING.fullmatch(self.resolution.strip())
        if match is None:
            return None
        return Scaling(_to_float(match["factor"]), match["unit"].strip())
