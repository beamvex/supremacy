/// One `.GPH` index record (`[si+0]`..`[si+0xA]` in the draw routines).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Record {
    /// Draw position, pixels — `[si+0]`.
    pub x: u16,
    /// Draw position, rows — `[si+2]`.
    pub y: u16,
    /// Bytes per row for the mode — `ax` on decoder entry, `[si+4]`.
    pub w: u16,
    /// Row count — `[si+6]`.
    pub h: u16,
    /// Stream offset in paragraphs; the loader adds the heap segment in
    /// place, so after fixup this is an absolute segment (`ds` at `0x2FB7A`).
    pub ofs_para: u16,
    /// `0` = opaque blit, `1` = XOR overlay — `[si+0xA]`.
    pub mode_flag: u16,
}

/// On-disk record size — the `add si,0xC` stride at `0x320E2`.
pub const RECORD_LEN: usize = 12;
