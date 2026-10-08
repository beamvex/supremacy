/// An unpacked program image plus its real entry/stack and fixup list —
/// everything the stub hands to the game at `0x23B56`-`0x23B77`.
pub struct Unpacked {
    /// `dest_len` paragraphs of decompressed bytes — the load image DOS
    /// would hold in memory, *before* segment fixups are applied.
    pub image: Vec<u8>,
    /// Real `CS` (paragraph, unrelocated).
    pub cs: u16,
    /// Real `IP`.
    pub ip: u16,
    /// Real `SS` (paragraph, unrelocated).
    pub ss: u16,
    /// Real `SP`.
    pub sp: u16,
    /// Linear offsets of every `add [es:di],bx` fixup site.
    pub relocs: Vec<u32>,
}
