/// File offset of the master DAC table in `GAME.unpacked.exe`
/// (dgroup `0xFAA:0x4994`).
pub const DAC_OFS: usize = 0x15034;

/// Table size — 256 colours × 3 six-bit components.
pub const DAC_LEN: usize = 768;

/// File offset of the `ds:0x4095` working table (dgroup `0xFAA:0x4095`)
/// — `pal_adj` (`0x2F1CF`) programs DAC regs `0x21..=0xFF` from its
/// colours `33..=255`.
pub const WORK_OFS: usize = 0x14735;

/// File offset of the `ds:0x4394` row table — sixteen `0x60`-byte
/// (32-colour) rows ending at the master table; the `[0x125E]` present
/// slot loads row `dx`'s colours `1..=31` into DAC regs `1..=31`.
pub const ROWS_OFS: usize = 0x14A34;
