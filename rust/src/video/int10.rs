use crate::args::Video;

impl Video {
    /// `int 10/AH=0` mode number — byte 0 of the parameter template
    /// (MCG `0x13`, EGA `0x0D`, CGA `0x05`, TGA `0x09`).
    #[must_use]
    pub fn int10_mode(self) -> u8 {
        match self {
            Self::Cga => 0x05,
            Self::Tga => 0x09,
            Self::Ega => 0x0D,
            Self::Mcga => 0x13,
        }
    }

    /// Only MCGA runs the `[0x1258]` slot for real (`int 10/AX=1012` at file
    /// `0x2EDBB`); the other modes land on the `0xF294` stub.
    #[must_use]
    pub fn programs_palette(self) -> bool {
        matches!(self, Self::Mcga)
    }
}
