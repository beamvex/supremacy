use super::Decoder;

/// Decode `n` consecutive EGA bitplane streams starting at `src[i]`.
///
/// Port of the caller loop at `0x2FB85`-`0x2FBE4` (`0x2FBEC`-`0x2FC4A` for
/// XOR records): each plane gets a fresh decoder ring, the EGA trailing-byte
/// `lodsb` positions `si` on the next stream, and `di` is restored so all
/// four planes land at the same screen offset.
#[must_use]
pub fn decode_planes(src: &[u8], i: usize, n: usize) -> (Vec<u8>, usize) {
    let mut out = Vec::new();
    let mut j = i;
    for _ in 0..n {
        Decoder::new().run(src, &mut j, &mut out, true);
    }
    (out, j)
}
