use super::Video;

impl Video {
    /// Asset-set stem — what the mode select patches into the `???.???`
    /// filename templates at `ds:0x1208`+ (`MCG.BIN`, `EGA.GPH`, …).
    #[must_use]
    pub fn stem(self) -> &'static str {
        match self {
            Self::Cga => "CGA",
            Self::Tga => "TGA",
            Self::Ega => "EGA",
            Self::Mcga => "MCG",
        }
    }
}
