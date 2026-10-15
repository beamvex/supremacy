/// One committed `data/*.bin` blob — a file-offset range of the
/// unpacked exe the port reads.
pub struct Region {
    /// Blob stem — `data/<name>.bin`.
    pub name: &'static str,
    /// File offset in the unpacked exe where the blob lands.
    pub ofs: usize,
    /// The committed bytes.
    pub bytes: &'static [u8],
}

/// The extracted regions (GAM-55): `low` = audio modules + embedded
/// songs, `dgroup` = the `ds:` image plus the `ss:` space up to the
/// `cs:` base (the `menu_text` stream lives there), `cs` = the
/// code-segment space (`TPL_OFS`-relative reads, seq streams, tables).
pub const REGIONS: [Region; 3] = [
    Region {
        name: "low",
        ofs: 0x0C00,
        bytes: include_bytes!("../../data/low.bin"),
    },
    Region {
        name: "dgroup",
        ofs: 0x106A0,
        bytes: include_bytes!("../../data/dgroup.bin"),
    },
    Region {
        name: "cs",
        ofs: 0x292B0,
        bytes: include_bytes!("../../data/cs.bin"),
    },
];
