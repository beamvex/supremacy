/// `lodsw` — read a little-endian match token at `src[i]` and advance.
pub(super) fn token(src: &[u8], i: &mut usize) -> Option<u16> {
    let t = u16::from_le_bytes([*src.get(*i)?, *src.get(*i + 1)?]);
    *i += 2;
    Some(t)
}
