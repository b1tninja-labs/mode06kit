"""Load the bundled GM CAN Mode $06 sample and parse it on demand.

The package ships one representative monitor section of GM's ``GM CAN mode $06 data`` reference
(``data/gm_can_mode06_sample.txt``, provenance in its header) so the CLI and tests parse something
real without a local copy of the full PDF. Point the CLI's ``--source`` at a full text extraction
to parse the whole document with the same parser.
"""

from __future__ import annotations

from importlib import resources

from mode06kit.model import Mode06Record
from mode06kit.parser import parse_section

SAMPLE_RESOURCE = "gm_can_mode06_sample.txt"


def sample_text() -> str:
    """The bundled GM CAN Mode $06 sample text, including its provenance header comments."""
    return resources.files("mode06kit.data").joinpath(SAMPLE_RESOURCE).read_text(encoding="utf-8")


def sample_records() -> list[Mode06Record]:
    """The bundled sample parsed into records."""
    return parse_section(sample_text())
