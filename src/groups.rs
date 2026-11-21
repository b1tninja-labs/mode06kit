//! OBD groups and the GM documents that apply to each, with the Chevrolet Volt selection.
//!
//! GM's 2017 OBD certification set maps each vehicle to an OBD Group and publishes a per-group
//! diagnostic-parameter file plus the two generic Mode $06 monitor/test-ID references (CAN and
//! legacy Class 2). The Chevrolet Volt is OBD Group 1A, so the applicable files are
//! `17OBDG01 Hybrid Diagnostics` and the generic `GM CAN mode $06 data` reference that this crate
//! parses. This catalog is re-encoded from GM's own document index (`docs/sources.md`).

/// The GM Mode $06 document site root.
pub const GM_MODE6_BASE: &str = "https://gsitlc.ext.gm.com/gmspo/mode6/pdf/";

/// The OBD group of the Chevrolet Volt.
pub const VOLT_OBD_GROUP: &str = "1A";

/// The generic CAN Mode $06 reference that [`crate::parser`] parses (and the bundled sample excerpts).
pub const CAN_MODE06_FILE: &str = "GM CAN mode $06 data final_dm.pdf";

/// One GM OBD reference PDF. An empty `obd_groups` marks a generic (all-vehicle) file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    /// The PDF's filename as GM publishes it.
    pub filename: &'static str,
    /// The OBD groups the file applies to; empty means every vehicle.
    pub obd_groups: &'static [&'static str],
    /// A short description of what the file contains.
    pub what: &'static str,
}

impl Document {
    /// True when the file applies to every vehicle rather than one OBD group.
    #[must_use]
    pub const fn is_generic(&self) -> bool {
        self.obd_groups.is_empty()
    }
}

/// GM 2017 OBD reference PDFs (`docs/sources.md`). The generic CAN file is what the parser reads.
static DOCS: &[Document] = &[
    Document {
        filename: "2017_OBD_GROUP_SUMMARY.pdf",
        obd_groups: &[],
        what: "index: Engine/Transmission diagnostic parameter file matrix",
    },
    Document {
        filename: "17OBDG01 Hybrid Diagnostics.pdf",
        obd_groups: &["1A", "1B"],
        what: "Hybrid powertrain diagnostic parameters (Volt, ELR, CT6)",
    },
    Document {
        filename: "17OBDG02 Hybrid Diagnostics.pdf",
        obd_groups: &["2"],
        what: "Hybrid powertrain diagnostic parameters (Malibu 1.8L hybrid)",
    },
    Document {
        filename: "17OBDG03 Engine Diagnostics.pdf",
        obd_groups: &["3"],
        what: "Engine diagnostic parameters (conventional gas 1.4L-6.2L)",
    },
    Document {
        filename: "17OBDG03 Transmisson Diagnostics.pdf",
        obd_groups: &["3"],
        what: "Transmission diagnostic parameters (GM's spelling)",
    },
    Document {
        filename: "17OBDG04 Engine Diagnostics.pdf",
        obd_groups: &["4"],
        what: "Engine diagnostic parameters (diesels: LH7, LWN, L5P Duramax)",
    },
    Document {
        filename: "17OBDG04 Transmisson Diagnostics.pdf",
        obd_groups: &["4"],
        what: "Transmission diagnostic parameters (GM's spelling)",
    },
    Document {
        filename: "17OBDG05 Engine Diagnostics.pdf",
        obd_groups: &["5"],
        what: "Engine diagnostic parameters (6.0L L96 HD)",
    },
    Document {
        filename: "17OBDG05 Transmission Diagnostics.pdf",
        obd_groups: &["5"],
        what: "Transmission diagnostic parameters",
    },
    Document {
        filename: "17OBDG06 Engine Diagnostics.pdf",
        obd_groups: &["6"],
        what: "Engine diagnostic parameters (6.0L L96)",
    },
    Document {
        filename: "17OBDG06 Transmission Diagnostics.pdf",
        obd_groups: &["6"],
        what: "Transmission diagnostic parameters",
    },
    Document {
        filename: "GM CAN mode $06 data final_dm.pdf",
        obd_groups: &[],
        what: "Generic Mode $06 monitor/test-ID definitions (CAN/GMLAN, ISO 15031-5)",
    },
    Document {
        filename: "GM Class2 mode $06 data final_dm.pdf",
        obd_groups: &[],
        what: "Generic Mode $06 monitor/test-ID definitions (legacy Class 2)",
    },
];

/// Every catalogued GM OBD reference PDF.
#[must_use]
pub fn documents() -> &'static [Document] {
    DOCS
}

/// The documents applicable to an OBD group: its group-specific files plus the generic ones.
#[must_use]
pub fn files_for_group(group: &str) -> Vec<&'static Document> {
    DOCS.iter()
        .filter(|doc| doc.is_generic() || doc.obd_groups.contains(&group))
        .collect()
}

/// The documents applicable to the Chevrolet Volt (OBD Group 1A).
#[must_use]
pub fn volt_files() -> Vec<&'static Document> {
    files_for_group(VOLT_OBD_GROUP)
}
