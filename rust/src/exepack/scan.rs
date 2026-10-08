use super::consts::SCAN_MAX;
use super::Error;

/// Locate the top of the command stream: the stub runs `repe scasb`
/// (`al=0xFF`, `cx=0x10`, `DF=1`) over the 16 bytes below the EXEPACK block
/// (`0x23ABB`-`0x23AC6`), finding the last non-`0xFF` byte — the final
/// command byte.
pub(crate) fn stream_end(img: &[u8], block_ofs: usize) -> Result<usize, Error> {
    let lo = block_ofs.saturating_sub(SCAN_MAX);
    (lo..block_ofs.min(img.len()))
        .rev()
        .find(|&i| img[i] != 0xFF)
        .ok_or(Error::Corrupt(0))
}
