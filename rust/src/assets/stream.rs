use super::AssetSet;

impl AssetSet {
    /// Record `idx`'s compressed stream — `ds:si` in the decoder, i.e. the
    /// `.BIN` bytes from `(ofs_para - heap_seg) * 16` to end of file (the
    /// decoder stops at the stream's own terminator; EGA records hold four
    /// consecutive plane streams).
    #[must_use]
    pub fn stream(&self, idx: usize) -> &[u8] {
        let ofs = usize::from(self.records[idx].ofs_para.wrapping_sub(self.heap_seg)) * 16;
        self.bin.get(ofs..).unwrap_or(&[])
    }
}
