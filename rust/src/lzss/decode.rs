use super::decode_at;

/// Decode one LZSS stream from the start of `src` (packed-mode terminator
/// semantics). Equivalent to `tools/lzss.py`'s `decode`.
#[must_use]
pub fn decode(src: &[u8]) -> Vec<u8> {
    decode_at(src, 0).0
}
