"""The parser: leading-ID extraction, footnote markers, cell splitting, grouping and lookup."""

from __future__ import annotations

import pathlib

from mode06kit.catalog import sample_records
from mode06kit.parser import by_obdmid, lookup, parse_records, parse_section

FIXTURES = pathlib.Path(__file__).parent / "fixtures"


def test_a_clean_row_yields_the_three_leading_ids():
    (record,) = parse_records("01        03       0A     Rich to Lean Sensor Switch Time    0 to 65535 ms   1 ms / bit")
    assert (record.obdmid, record.tid, record.uasid) == (0x01, 0x03, 0x0A)
    assert record.description == "Rich to Lean Sensor Switch Time"
    assert record.unit_range == "0 to 65535 ms"
    assert record.resolution == "1 ms / bit"
    assert record.footnote is None


def test_a_tid_footnote_marker_is_captured():
    (record,) = parse_records(
        "01        10 (1)   06     Dynamic Response Performance      0 to 1.999    0.122 mV / bit"
    )
    assert record.tid == 0x10
    assert record.footnote == 1
    assert record.uasid == 0x06


def test_a_wrapped_continuation_line_is_not_a_record():
    text = "01   01   0A   Rich to Lean Sensor Threshold Voltage   0.0000 to 7.9900 Volts   0.122 mV / bit\n"
    text += "          Lean to Rich Sensor Threshold Voltage   0.0000 to 7.9900 Volts   0.122 mV / bit"
    records = parse_records(text)
    assert len(records) == 1


def test_description_starting_with_hex_letters_is_not_misread_as_a_row():
    # "B1S1 WRAF Slow Response" leads with B1 but is a wrapped description, not OBDMID B1.
    assert parse_records("          B1S1 WRAF Slow Response   -40 to 6513.5 C   0.1 C / bit") == []


def test_boilerplate_and_comments_are_skipped():
    text = "# provenance\n[page 30]\nFOOTNOTES\nGMLAN rev3\n01 01 0A Foo 0 to 5 V 1 V / bit"
    records = parse_records(text)
    assert len(records) == 1
    assert records[0].obdmid == 0x01


def test_a_slash_unit_resolution_is_split_from_the_range():
    text = (FIXTURES / "two_rows.txt").read_text(encoding="utf-8")
    records = parse_section(text)
    switch = next(r for r in records if r.tid == 0x84)
    assert switch.footnote == 16
    assert switch.unit_range == "-65536 to +65534 mV/s"
    assert switch.resolution == "2 mV/s / bit"


def test_parse_section_names_the_monitor_from_the_heading():
    records = sample_records()
    assert records
    assert all(r.monitor == "Oxygen Sensor Monitor Bank 1 Sensor 1" for r in records)


def test_the_sample_is_all_obdmid_01():
    grouped = by_obdmid(sample_records())
    assert set(grouped) == {0x01}


def test_lookup_filters_by_obdmid_and_tid():
    records = sample_records()
    (hit,) = list(lookup(records, 0x01, 0x04))
    assert hit.tid == 0x04
    assert hit.description == "Lean to Rich Sensor Switch Time"
    assert list(lookup(records, 0x05)) == []
