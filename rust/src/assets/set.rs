use crate::gph::Record;

/// A loaded asset set: the fixed-up `.GPH` index plus the whole `.BIN`
/// payload, mirroring the post-loader memory state (`ds:0x158` records +
/// heap segment `0x3A22`).
pub struct AssetSet {
    /// Index records with `ofs_para` already relocated by `heap_seg`.
    pub records: Vec<Record>,
    /// Raw `.BIN` bytes — all the set's compressed streams concatenated on
    /// 16-byte boundaries.
    pub bin: Vec<u8>,
    /// Segment the `.BIN` was loaded at, used by the `ofs_para` fixup.
    pub heap_seg: u16,
}
