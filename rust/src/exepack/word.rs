use super::Error;

/// Bounds-checked little-endian `u16` read.
pub(super) fn word(buf: &[u8], i: usize) -> Result<u16, Error> {
    let b = buf.get(i..i + 2).ok_or(Error::Corrupt(0))?;
    Ok(u16::from_le_bytes([b[0], b[1]]))
}
