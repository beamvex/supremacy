use super::{Decoder, Frame, Kind};

/// A blit into a strided destination — the decoder's `es:di` contract.
pub struct Blit<'a> {
    /// Destination bytes (the VGA buffer or a stand-in).
    pub dst: &'a mut [u8],
    /// Start offset — `di` = `y * stride + x` (planar byte offset for EGA).
    pub di: usize,
    /// Row width in bytes (`w` from the `.GPH` record, passed in `ax`).
    pub width: usize,
    /// Bytes per destination row: `0x140` packed modes, `0x28` EGA.
    pub stride: usize,
    /// Opaque or XOR draw (`[si+0xA]` tested at `0x2FB80`).
    pub kind: Kind,
    /// EGA variants consume the terminator's trailing byte.
    pub eat_tail: bool,
}

/// Decode one stream at `src[*i]` into `blit.dst` with row wrap.
///
/// Faithful port of the draw half of `0x2EBEF`/`0x2EC8C`/`0x2FC5D`/`0x2FCF9`:
/// the asm writes straight to VRAM through `es:di`; here `dst`/`di` play that
/// role so the same routine is testable and reusable.
pub fn draw(src: &[u8], i: &mut usize, blit: &mut Blit<'_>) {
    let mut f = Frame::new(
        blit.dst,
        blit.di,
        blit.width,
        blit.stride,
        matches!(blit.kind, Kind::Xor),
    );
    Decoder::new().run(src, i, &mut f, blit.eat_tail);
    blit.di = f.di;
}
