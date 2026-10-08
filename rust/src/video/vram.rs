use crate::args::Video;

impl Video {
    /// VRAM segment — word 4 of the parameter template: `0xA000` for the
    /// `int 10` modes on `A000`, `0xB800` for the CGA/Tandy modes.
    #[must_use]
    pub fn vram_segment(self) -> u16 {
        match self {
            Self::Cga | Self::Tga => 0xB800,
            Self::Ega | Self::Mcga => 0xA000,
        }
    }

    /// Bytes of the mode's VRAM span modelled by [`crate::video::Screen`]:
    /// `0xFA00` packed MCG, `0x4000`/`0x8000` banked CGA/TGA, four
    /// `0x1F40`-byte planes for EGA.
    #[must_use]
    pub fn vram_len(self) -> usize {
        match self {
            Self::Cga => 0x4000,
            Self::Tga => 0x8000,
            Self::Ega => 4 * 0x1F40,
            Self::Mcga => 0xFA00,
        }
    }
}
