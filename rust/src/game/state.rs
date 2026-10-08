use super::consts::{REC_BASE, REC_COUNT, REC_STRIDE, STATE_LEN, STATE_OFS};

/// The saveable game-state block — the exact `0x1F8B` bytes at
/// `ds:0x7D39..0x9CC4` the save routine writes whole (file `0x383F3`)
/// and the load routine reads back (`0x383CE`), and that the heap-staging
/// copies (`0x38432`/`0x3844B`) move to/from the loader heap segment.
///
/// Indices are `ds:` offsets minus [`super::STATE_OFS`]; the accessor
/// methods take the real `ds:` addresses the asm uses.
pub struct State {
    /// `ds:0x7D39..0x9CC4` — strings workspace, the `0x3A`-stride record
    /// array (anchored per galaxy preset), scalars and menu state.
    pub block: Box<[u8; STATE_LEN]>,
}

impl State {
    /// A zeroed block — the image ships the initial contents, so prefer
    /// [`State::from_image`] when the unpacked exe is available.
    #[must_use]
    pub fn new() -> Self {
        Self {
            block: Box::new([0; STATE_LEN]),
        }
    }

    /// Build from the unpacked exe's data — the `0x1F8B` bytes at file
    /// `0x189D9` (`ds:0x7D39`) are the shipped initial state.
    #[must_use]
    pub fn from_image(img: &[u8]) -> Self {
        let mut st = Self::new();
        let lo = STATE_OFS + 0x10CA0;
        st.block.copy_from_slice(&img[lo..lo + STATE_LEN]);
        st
    }

    /// `ds:` offset of record `i`'s base — `[0x9158] + i·0x3A`, the
    /// `cs:0x5E3A` accessor (file `0x35E3A`–`0x35E59`).
    #[must_use]
    pub fn rec_ofs(&self, i: u16) -> u16 {
        self.word(REC_BASE)
            .wrapping_add(i.wrapping_mul(u16::try_from(REC_STRIDE).unwrap_or(0)))
    }

    /// Planet count `[0x91B8]` — also the index of the appended player
    /// faction record.
    #[must_use]
    pub fn planets(&self) -> u16 {
        self.word(REC_COUNT)
    }

    fn i(ds: u16) -> usize {
        usize::from(ds) - STATE_OFS
    }

    /// Little-endian word at `ds:` offset `o`.
    #[must_use]
    pub fn word(&self, o: usize) -> u16 {
        let i = Self::i(u16::try_from(o).unwrap_or(0));
        u16::from_le_bytes([self.block[i], self.block[i + 1]])
    }

    /// Write little-endian word at `ds:` offset `o`.
    pub fn set_word(&mut self, o: usize, v: u16) {
        let i = Self::i(u16::try_from(o).unwrap_or(0));
        self.block[i..i + 2].copy_from_slice(&v.to_le_bytes());
    }

    /// Little-endian dword at `ds:` offset `o`.
    #[must_use]
    pub fn dword(&self, o: usize) -> u32 {
        let i = Self::i(u16::try_from(o).unwrap_or(0));
        u32::from_le_bytes(self.block[i..i + 4].try_into().unwrap_or([0; 4]))
    }

    /// Write little-endian dword at `ds:` offset `o`.
    pub fn set_dword(&mut self, o: usize, v: u32) {
        let i = Self::i(u16::try_from(o).unwrap_or(0));
        self.block[i..i + 4].copy_from_slice(&v.to_le_bytes());
    }

    /// Byte at `ds:` offset `o`.
    #[must_use]
    pub fn byte(&self, o: usize) -> u8 {
        self.block[Self::i(u16::try_from(o).unwrap_or(0))]
    }

    /// Write byte at `ds:` offset `o`.
    pub fn set_byte(&mut self, o: usize, v: u8) {
        self.block[Self::i(u16::try_from(o).unwrap_or(0))] = v;
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
