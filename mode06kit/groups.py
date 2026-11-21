"""OBD groups and the GM documents that apply to each, with the Chevrolet Volt selection.

GM's 2017 OBD certification set maps each vehicle to an **OBD Group** and publishes a per-group
diagnostic-parameter file plus the two generic Mode $06 monitor/test-ID references (CAN and legacy
Class 2). The Chevrolet Volt is **OBD Group 1A**, so the applicable files are ``17OBDG01 Hybrid
Diagnostics`` and the generic ``GM CAN mode $06 data`` reference that this package parses.

This catalog is re-encoded from GM's own document index (``docs/sources.md``); it does not depend
on the sibling ``udsdiag``.
"""

from __future__ import annotations

from dataclasses import dataclass

GM_MODE6_BASE = "https://gsitlc.ext.gm.com/gmspo/mode6/pdf/"
VOLT_OBD_GROUP = "1A"


@dataclass(frozen=True, slots=True)
class Document:
    """One GM OBD reference PDF. An empty ``obd_groups`` marks a generic (all-vehicle) file."""

    filename: str
    obd_groups: tuple[str, ...]
    what: str

    @property
    def is_generic(self) -> bool:
        """True when the file applies to every vehicle rather than one OBD group."""
        return not self.obd_groups


# GM 2017 OBD reference PDFs (docs/sources.md). The generic CAN Mode $06 file is what parser.py reads.
_DOCS: tuple[Document, ...] = (
    Document("2017_OBD_GROUP_SUMMARY.pdf", (), "index: Engine/Transmission diagnostic parameter file matrix"),
    Document(
        "17OBDG01 Hybrid Diagnostics.pdf", ("1A", "1B"), "Hybrid powertrain diagnostic parameters (Volt, ELR, CT6)"
    ),
    Document("17OBDG02 Hybrid Diagnostics.pdf", ("2",), "Hybrid powertrain diagnostic parameters (Malibu 1.8L hybrid)"),
    Document("17OBDG03 Engine Diagnostics.pdf", ("3",), "Engine diagnostic parameters (conventional gas 1.4L-6.2L)"),
    Document("17OBDG03 Transmisson Diagnostics.pdf", ("3",), "Transmission diagnostic parameters (GM's spelling)"),
    Document(
        "17OBDG04 Engine Diagnostics.pdf", ("4",), "Engine diagnostic parameters (diesels: LH7, LWN, L5P Duramax)"
    ),
    Document("17OBDG04 Transmisson Diagnostics.pdf", ("4",), "Transmission diagnostic parameters (GM's spelling)"),
    Document("17OBDG05 Engine Diagnostics.pdf", ("5",), "Engine diagnostic parameters (6.0L L96 HD)"),
    Document("17OBDG05 Transmission Diagnostics.pdf", ("5",), "Transmission diagnostic parameters"),
    Document("17OBDG06 Engine Diagnostics.pdf", ("6",), "Engine diagnostic parameters (6.0L L96)"),
    Document("17OBDG06 Transmission Diagnostics.pdf", ("6",), "Transmission diagnostic parameters"),
    Document(
        "GM CAN mode $06 data final_dm.pdf", (), "Generic Mode $06 monitor/test-ID definitions (CAN/GMLAN, ISO 15031-5)"
    ),
    Document(
        "GM Class2 mode $06 data final_dm.pdf", (), "Generic Mode $06 monitor/test-ID definitions (legacy Class 2)"
    ),
)

# The generic CAN reference parser.py parses; also the file bundled (in excerpt) as the sample.
CAN_MODE06_FILE = "GM CAN mode $06 data final_dm.pdf"


def documents() -> tuple[Document, ...]:
    """Every catalogued GM OBD reference PDF."""
    return _DOCS


def files_for_group(group: str) -> tuple[Document, ...]:
    """The documents applicable to an OBD group: its group-specific files plus the generic ones."""
    return tuple(doc for doc in _DOCS if group in doc.obd_groups or doc.is_generic)


def volt_files() -> tuple[Document, ...]:
    """The documents applicable to the Chevrolet Volt (OBD Group 1A)."""
    return files_for_group(VOLT_OBD_GROUP)
