use super::Decoder;

/// Decode one stream from `src[i]` into a flat buffer; returns the decoded
/// bytes and the index just past the `00 00` terminator (packed-mode
/// semantics — the trailing zero is not consumed).
#[must_use]
pub fn decode_at(src: &[u8], i: usize) -> (Vec<u8>, usize) {
    let mut out = Vec::new();
    let mut j = i;
    Decoder::new().run(src, &mut j, &mut out, false);
    (out, j)
}
