use crate::args::Video;

impl Video {
    /// Start `di` for a record drawn at (`x`,`y`) — the per-mode address
    /// computation in each draw routine:
    ///
    /// - MCG `0x2EBB1`: `di = y*0x140 + x`
    /// - EGA `0x2FB69`: `di = y*0x28 + x` (per plane)
    /// - CGA `0x3064F`: `di = (y>>1)*0x50 + (y&1)*0x2000 + x`
    /// - TGA `0x310F9`: `di = (y>>2)*0xA0 + (y&3)*0x2000 + x`
    #[must_use]
    pub fn di_at(self, x: usize, y: usize) -> usize {
        match self {
            Self::Cga => (y >> 1) * 0x50 + (y & 1) * 0x2000 + x,
            Self::Tga => (y >> 2) * 0xA0 + (y & 3) * 0x2000 + x,
            Self::Ega => y * 0x28 + x,
            Self::Mcga => y * 0x140 + x,
        }
    }
}
