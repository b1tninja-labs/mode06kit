"""The bundled GM CAN Mode $06 sample loads, carries its provenance, and parses to records."""

from __future__ import annotations

from mode06kit import catalog


def test_the_sample_text_carries_a_provenance_header():
    text = catalog.sample_text()
    assert "GM CAN mode $06 data" in text
    assert text.lstrip().startswith("#")


def test_the_sample_parses_to_records():
    records = catalog.sample_records()
    assert len(records) >= 10
    assert all(r.obdmid == 0x01 for r in records)


def test_the_provenance_comments_are_not_parsed_as_records():
    # Every record's source line is past the comment header block.
    records = catalog.sample_records()
    assert all(not catalog.sample_text().splitlines()[r.line - 1].startswith("#") for r in records)
