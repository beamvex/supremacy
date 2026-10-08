use super::Error;

/// `rep movsb` with `DF=1` (`0x23B0E`): copy `len` bytes ending at
/// `src_end` (descending) to `di` (descending). Sources and destination
/// live in the same buffer — that's the in-place trick: writes always land
/// above the read cursor because compressed data is smaller.
pub(super) fn copy(
    buf: &mut [u8],
    di: &mut usize,
    src_end: usize,
    len: usize,
) -> Result<(), Error> {
    if *di >= buf.len() || *di + 1 < len || src_end >= buf.len() || src_end + 1 < len {
        return Err(Error::Corrupt(0xB2));
    }
    for k in 0..len {
        buf[*di - k] = buf[src_end - k];
    }
    *di -= len;
    Ok(())
}
