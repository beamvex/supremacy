use super::Video;

impl Video {
    /// Port of the compare chain at `0x3200E`-`0x3202B`: `C`/`T`/`E`/`M`
    /// select CGA/TGA/EGA/MCGA; anything else falls through the final `jz`
    /// into the MCGA arm, which is why bare `GAME` gets MCGA.
    #[must_use]
    pub fn from_arg(c: u8) -> Self {
        match c {
            b'C' => Self::Cga,
            b'T' => Self::Tga,
            b'E' => Self::Ega,
            _ => Self::Mcga,
        }
    }
}
