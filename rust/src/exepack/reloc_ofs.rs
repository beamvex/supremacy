use super::consts::{CORRUPT_MSG, FALLBACK_TABLE};

/// Offset of the packed relocation stream inside the EXEPACK block — it
/// follows the corrupt-file message the stub prints via `int 21/AH=40`
/// (`dx=0x10F` for GAME's stub). Fallback is GAME's hardcoded `si=0x125`.
pub(crate) fn table_ofs(block: &[u8]) -> usize {
    block
        .windows(CORRUPT_MSG.len())
        .position(|w| w == CORRUPT_MSG)
        .map_or(FALLBACK_TABLE, |p| p + CORRUPT_MSG.len())
}
