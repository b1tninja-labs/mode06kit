"""The ``mode06kit`` command line: a read-only reader over parsed GM Mode $06 definitions.

Every sub-command parses text -- the bundled sample by default, or a local extraction named with
``--source`` -- and prints from it. Nothing here opens a bus, sends a frame, or writes to a vehicle.
"""

from __future__ import annotations

import argparse
from collections.abc import Sequence
from pathlib import Path

from mode06kit import catalog, groups
from mode06kit.model import Mode06Record
from mode06kit.parser import by_obdmid, lookup, parse_section


def _load(source: str | None) -> list[Mode06Record]:
    if source is None:
        return catalog.sample_records()
    return parse_section(Path(source).read_text(encoding="utf-8"))


def _format(record: Mode06Record) -> str:
    uasid = record.uasid_hex or "$--"
    foot = f" (fn {record.footnote})" if record.footnote is not None else ""
    pieces = [f"OBDMID {record.obdmid_hex} TID {record.tid_hex} UASID {uasid}{foot}"]
    if record.description:
        pieces.append(record.description)
    if record.unit_range:
        pieces.append(record.unit_range)
    if record.resolution:
        pieces.append(record.resolution)
    return "  ".join(pieces)


def _hex(value: str) -> int:
    return int(value, 16)


def _cmd_records(args: argparse.Namespace) -> int:
    records = _load(args.source)
    for record in records:
        print(_format(record))
    print("--")
    print(f"{len(records)} records across {len(by_obdmid(records))} OBDMID(s)")
    return 0


def _cmd_lookup(args: argparse.Namespace) -> int:
    records = _load(args.source)
    tid = _hex(args.tid) if args.tid is not None else None
    hits = list(lookup(records, _hex(args.obdmid), tid))
    if not hits:
        where = f"TID ${tid:02X}" if tid is not None else "any TID"
        print(f"no record for OBDMID ${_hex(args.obdmid):02X} {where}")
        return 0
    for record in hits:
        print(_format(record))
    return 0


def _cmd_monitors(args: argparse.Namespace) -> int:
    records = _load(args.source)
    seen: dict[str, int] = {}
    for record in records:
        name = record.monitor or "(no heading)"
        seen[name] = seen.get(name, 0) + 1
    for name, count in seen.items():
        print(f"{count:>4}  {name}")
    return 0


def _cmd_groups(_: argparse.Namespace) -> int:
    for doc in groups.documents():
        tag = ",".join(doc.obd_groups) if doc.obd_groups else "generic"
        print(f"[{tag}]  {doc.filename}  -- {doc.what}")
    print("--")
    volt = groups.volt_files()
    print(f"Chevrolet Volt is OBD Group {groups.VOLT_OBD_GROUP}: {len(volt)} applicable files")
    for doc in volt:
        print(f"    {doc.filename}")
    return 0


def build_parser() -> argparse.ArgumentParser:
    """The argument parser for the ``mode06kit`` command."""
    parser = argparse.ArgumentParser(prog="mode06kit", description="Read-only GM Mode $06 definition reader.")
    sub = parser.add_subparsers(dest="command", required=True)

    def add_source(p: argparse.ArgumentParser) -> None:
        p.add_argument("--source", metavar="FILE", help="a local Mode $06 text extraction (default: bundled sample)")

    p_records = sub.add_parser("records", help="list every parsed Mode $06 record")
    add_source(p_records)
    p_records.set_defaults(func=_cmd_records)

    p_lookup = sub.add_parser("lookup", help="look up records by OBDMID (and optional TID)")
    p_lookup.add_argument("obdmid", help="an OBD Monitor ID in hex, e.g. 01")
    p_lookup.add_argument("tid", nargs="?", help="a Test ID in hex, e.g. 04")
    add_source(p_lookup)
    p_lookup.set_defaults(func=_cmd_lookup)

    p_monitors = sub.add_parser("monitors", help="list the monitor headings found and their row counts")
    add_source(p_monitors)
    p_monitors.set_defaults(func=_cmd_monitors)

    sub.add_parser("groups", help="list the OBD-group document mapping and the Volt selection").set_defaults(
        func=_cmd_groups
    )

    return parser


def main(argv: Sequence[str] | None = None) -> int:
    """Run the ``mode06kit`` command line."""
    parser = build_parser()
    args = parser.parse_args(argv)
    func = args.func
    result: int = func(args)
    return result
