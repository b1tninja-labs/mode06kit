"""The read-only command line over the parsed definitions."""

from __future__ import annotations

import pytest

from mode06kit.cli import main


def test_records_lists_and_summarises(capsys):
    assert main(["records"]) == 0
    out = capsys.readouterr().out
    assert "OBDMID $01 TID $04 UASID $0A" in out
    assert "records across 1 OBDMID(s)" in out


def test_lookup_by_obdmid_and_tid(capsys):
    assert main(["lookup", "01", "04"]) == 0
    out = capsys.readouterr().out
    assert "Lean to Rich Sensor Switch Time" in out


def test_lookup_reports_a_miss_without_failing(capsys):
    assert main(["lookup", "05"]) == 0
    out = capsys.readouterr().out
    assert "no record for OBDMID $05" in out


def test_a_footnote_marker_shows_in_the_listing(capsys):
    assert main(["lookup", "01", "10"]) == 0
    out = capsys.readouterr().out
    assert "(fn 1)" in out


def test_monitors_lists_the_heading(capsys):
    assert main(["monitors"]) == 0
    out = capsys.readouterr().out
    assert "Oxygen Sensor Monitor Bank 1 Sensor 1" in out


def test_groups_shows_the_volt_selection(capsys):
    assert main(["groups"]) == 0
    out = capsys.readouterr().out
    assert "Chevrolet Volt is OBD Group 1A" in out
    assert "17OBDG01 Hybrid Diagnostics.pdf" in out


def test_source_reads_a_local_file(tmp_path, capsys):
    path = tmp_path / "x.txt"
    path.write_text("My Monitor\n02   07   24   Foo Test   0 to 9 counts   1 count / bit\n", encoding="utf-8")
    assert main(["records", "--source", str(path)]) == 0
    out = capsys.readouterr().out
    assert "OBDMID $02 TID $07 UASID $24" in out


def test_no_command_is_an_error():
    with pytest.raises(SystemExit):
        main([])
