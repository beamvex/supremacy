/// File offset of the master DAC table in `GAME.unpacked.exe`
/// (dgroup `0xFAA:0x4994`).
pub const DAC_OFS: usize = 0x15034;

/// Table size — 256 colours × 3 six-bit components.
pub const DAC_LEN: usize = 768;
