//! `ds:` offsets and sizes for the saveable state block — the real
//! addresses the asm uses, so `State` accessors take `ds:` values.

/// `ds:` offset where the save block starts (`dx` in the read/write calls,
/// file `0x383E1`/`0x38415`).
pub const STATE_OFS: usize = 0x7D39;
/// Save block length: `cx = 0x9CC4 − 0x7D39` (file `0x383DD`/`0x38411`).
/// Every savegame file is exactly this many bytes.
pub const STATE_LEN: usize = 0x1F8B;
/// `ds:` slot of the record-array base pointer (`[0x9158]`, written by the
/// galaxy preset at file `0x36809`).
pub const REC_BASE: usize = 0x9158;
/// `ds:` slot of the total record count (`[0x91B6]`, file `0x36811`).
pub const REC_TOTAL: usize = 0x91B6;
/// `ds:` slot of the planet count — also the index of the appended player
/// faction record (`[0x91B8]`, file `0x36815`).
pub const REC_COUNT: usize = 0x91B8;
/// `ds:` slot of the difficulty byte (`[0x91E4]`, file `0x3681B`).
pub const DIFFICULTY: usize = 0x91E4;
/// `ds:` slot the preset also mirrors the record base into (`[0x917C]`).
pub const REC_SEL: usize = 0x917C;
/// `ds:` slot the preset also mirrors the record base into (`[0x9184]`).
pub const REC_CUR: usize = 0x9184;
/// `ds:` slot the preset also mirrors the record base into (`[0x9178]`).
pub const REC_ALT: usize = 0x9178;
/// `ds:` slot of the per-difficulty countdown (`[0x9164]`: 6/14/30).
pub const DIFF_PACE: usize = 0x9164;
/// Byte stride of a planet/faction record (`[0x9CB6] = 0x3A`, file
/// `0x35E41`).
pub const REC_STRIDE: usize = 0x3A;
