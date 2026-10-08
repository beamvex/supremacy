use super::consts::N;

/// LZSS decoder state.
///
/// `ring` is the 4 KB buffer the asm keeps at `SS:0x0000-0x0FFF` (indexed by
/// `bp`/`bx`), `r` is the write cursor `bp`, and `flags` is the `dx` register
/// holding the LSB-first flag bits with the `0x100` sentinel.
pub struct Decoder {
    pub(super) ring: [u8; N],
    pub(super) r: usize,
    pub(super) flags: u16,
}
