"""The record model: hex notation and the range/scaling parsers, including GM's en-dash minus."""

from __future__ import annotations

from mode06kit.model import Mode06Record

A = Mode06Record(
    obdmid=0x01,
    tid=0x04,
    uasid=0x0A,
    description="Lean to Rich Sensor Switch Time",
    unit_range="0 to 65535 ms",
    resolution="1 ms / bit",
    footnote=None,
    monitor="Oxygen Sensor Monitor Bank 1 Sensor 1",
    line=5,
)


def test_identifiers_render_in_gm_dollar_hex():
    assert A.obdmid_hex == "$01"
    assert A.tid_hex == "$04"
    assert A.uasid_hex == "$0A"


def test_a_blank_uasid_renders_as_none():
    record = Mode06Record(0x01, 0x05, None, "", "", "", None, None, 1)
    assert record.uasid_hex is None


def test_value_range_parses_min_max_and_unit():
    value = A.value_range
    assert value is not None
    assert value.minimum == 0.0
    assert value.maximum == 65535.0
    assert value.unit == "ms"


def test_scaling_parses_per_bit_and_unit():
    scaling = A.scaling
    assert scaling is not None
    assert scaling.per_bit == 1.0
    assert scaling.unit == "ms"


def test_an_en_dash_is_read_as_a_minus_sign():
    # GM prints some negative limits with an en dash (U+2013) instead of a hyphen.
    record = Mode06Record(0x01, 0x84, 0x04, "", "\N{EN DASH}32.768 to +32.767 kPa", "", None, None, 1)
    value = record.value_range
    assert value is not None
    assert value.minimum == -32.768
    assert value.maximum == 32.767
    assert value.unit == "kPa"


def test_a_unit_with_a_slash_survives_the_scaling_parse():
    record = Mode06Record(0x01, 0x84, 0x04, "", "", "2 mV/s / bit", None, None, 1)
    scaling = record.scaling
    assert scaling is not None
    assert scaling.per_bit == 2.0
    assert scaling.unit == "mV/s"


def test_a_wrapped_range_cell_returns_none_not_a_guess():
    record = Mode06Record(0x01, 0x85, 0x0C, "O2 Sensor Delayed Response", "", "", None, None, 1)
    assert record.value_range is None
    assert record.scaling is None
