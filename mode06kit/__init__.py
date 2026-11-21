"""mode06kit: parse GM's own published Mode $06 OBD documents into a typed model.

Mode $06 (On-Board Monitoring Test Results) is a generic OBD-II service of SAE J1979 /
ISO 15031-5. Each result is addressed by an **OBD Monitor ID (OBDMID)** and a **Test ID
(TID)**, and its value is interpreted through a **unit-and-scaling ID (UASID)** and a pair of
min/max test limits. Manufacturers publish the monitor/test-ID definitions for the independent
aftermarket; this package parses GM's own ``GM CAN mode $06 data`` reference (``docs/sources.md``)
into :class:`~mode06kit.model.Mode06Record` values.

It reads GM's published text and models it; it opens no bus, sends no frame, and writes nothing
to a vehicle. The application-layer sibling ``udsdiag`` only *catalogs* these documents;
``mode06kit`` is the *parser* of their contents and does not depend on ``udsdiag``.
"""

from __future__ import annotations

__version__ = "0.0.0"
