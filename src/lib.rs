//! Parse GM's own published **Mode $06** (On-Board Monitoring Test Results, SAE J1979 /
//! ISO 15031-5) documents into a typed model of OBD Monitor IDs, Test IDs, unit-and-scaling IDs and
//! value ranges.
//!
//! Each Mode $06 result is addressed by an OBD Monitor ID (OBDMID) and a Test ID (TID), and its
//! value is interpreted through a unit-and-scaling ID (UASID) and a pair of min/max limits.
//! Manufacturers publish the monitor/test-ID definitions for the independent aftermarket; this
//! crate parses GM's own `GM CAN mode $06 data` reference (`docs/sources.md`) into
//! [`model::Mode06Record`] values and reads them back through a small read-only CLI.
//!
//! It reads GM's published text and models it; it opens no bus, sends no frame, and writes nothing
//! to a vehicle. It is the *parser* of these documents; the application-layer catalog of them lives
//! in the sibling `udsdiag`, and this crate does not depend on it.

pub mod catalog;
pub mod cli;
pub mod groups;
pub mod model;
pub mod parser;
