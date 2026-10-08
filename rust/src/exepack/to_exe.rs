use super::mz_write::mz_write;
use super::Unpacked;

impl Unpacked {
    /// Repackage as an `.exe`: MZ header, canonical `seg:ofs` fixup table at
    /// `0x1C`, paragraph padding, then the unrelocated image — the on-disk
    /// counterpart of what [`super::unpack`] produces in memory.
    #[must_use]
    pub fn to_exe(&self) -> Vec<u8> {
        let hdr_paras = (0x1C + self.relocs.len() * 4).div_ceil(16);
        let mut out = mz_write(self, hdr_paras);
        for &r in &self.relocs {
            let ofs = u16::try_from(r & 0xF).unwrap_or_default();
            let seg = u16::try_from(r >> 4).unwrap_or_default();
            out.extend_from_slice(&ofs.to_le_bytes());
            out.extend_from_slice(&seg.to_le_bytes());
        }
        out.resize(hdr_paras * 16, 0);
        out.extend_from_slice(&self.image);
        out
    }
}
