use std::ops::Range;

/// The MZ header fields the unpacker needs.
pub(crate) struct Mz {
    /// Byte range of the load image (everything after the header).
    pub image: Range<usize>,
    /// Entry `CS` — paragraph of the EXEPACK block inside the load image.
    pub entry_cs: u16,
}
