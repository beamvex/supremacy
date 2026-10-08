use super::Video;

impl Video {
    /// dgroup offset of the mode's `0x115B`-byte parameter template — the
    /// `si` values loaded at `0x32009`-`0x32024`, copied to `ds:0x143` by the
    /// `rep movsb` at `0x32034`.
    #[must_use]
    pub fn template_seg(self) -> u16 {
        match self {
            Self::Cga => 0xD3A2,
            Self::Tga => 0xE4FD,
            Self::Ega => 0xC247,
            Self::Mcga => 0xB0EC,
        }
    }
}
