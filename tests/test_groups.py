"""The OBD-group document mapping and the Chevrolet Volt (Group 1A) selection."""

from __future__ import annotations

from mode06kit import groups


def test_the_volt_is_group_1a():
    assert groups.VOLT_OBD_GROUP == "1A"


def test_files_for_a_group_include_the_group_specific_and_the_generic():
    files = groups.files_for_group("1A")
    names = {doc.filename for doc in files}
    assert "17OBDG01 Hybrid Diagnostics.pdf" in names
    assert "GM CAN mode $06 data final_dm.pdf" in names
    # A different group's hybrid file is not included.
    assert "17OBDG02 Hybrid Diagnostics.pdf" not in names


def test_volt_files_match_group_1a():
    assert groups.volt_files() == groups.files_for_group("1A")


def test_the_generic_files_apply_to_every_group():
    generic = {doc.filename for doc in groups.documents() if doc.is_generic}
    assert "GM CAN mode $06 data final_dm.pdf" in generic
    assert generic <= {doc.filename for doc in groups.files_for_group("3")}


def test_the_can_mode06_file_is_catalogued():
    names = {doc.filename for doc in groups.documents()}
    assert groups.CAN_MODE06_FILE in names
