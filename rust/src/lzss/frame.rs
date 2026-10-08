/// VRAM destination with the decoder's row-wrap logic.
///
/// Mirrors `[cs:0x5380]` (down-counter, starts at `w`) and `es:di`: each
/// emitted byte writes at `di`, and when the row counter hits zero
/// `di += stride - w` (`0x2EC31`/`0x2FC9F`). With `xor` set the byte is
/// `XOR`ed into the buffer instead of stored — the `0x2EC8C`/`0x2FCF9`
/// overlay variants.
pub struct Frame<'a> {
    /// Destination bytes indexed by `di` (the VGA buffer or a stand-in).
    pub buf: &'a mut [u8],
    /// Current write offset — `es:di` as a flat index.
    pub di: usize,
    /// Row width `w` in bytes — `ax` on entry, `[cs:0x5382]`.
    pub width: usize,
    /// Row stride — `0x140` for packed modes, `0x28` for EGA planes.
    pub stride: usize,
    /// `true` = XOR overlay draw (`mode_flag` 1 in the `.GPH` record).
    pub xor: bool,
    pub(super) row_left: usize,
}
