//! EXEPACK format constants.

/// `'RB'` — the signature word at block `+0xE`.
pub(crate) const SIG: &[u8; 2] = b"RB";

/// The corrupt-file message; the relocation table follows it immediately
/// (GAME: block `0x125`, SUPCHT: `0x132`).
pub(crate) const CORRUPT_MSG: &[u8] = b"Packed file is corrupt";

/// Where GAME's stub reads the relocation table (`mov si,0x125`); used when
/// the message can't be located.
pub(crate) const FALLBACK_TABLE: usize = 0x125;

/// `repne scasb` count — the stub scans at most 16 bytes for the `0xFF` pad.
pub(crate) const SCAN_MAX: usize = 16;
