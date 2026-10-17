//! EGA `menu_text` tile painters — the `0x30595` glyph blit and the
//! `0x305FB` space clear (file `0x30595`–`0x3061E`). `si`/`di` is a
//! within-plane offset (`0x28`/line): the blit writes the same `di`
//! in all four planes under a walked map mask, so the port's `buf`
//! (the planes laid out at `0x1F40` strides) takes one pass per
//! plane. Writes past a plane's `0x1F40` visible span drop — in the
//! asm they land in the plane window's invisible tail.

/// File offset of the `ss:0x7734` plane-interleaved menu font —
/// `0x30` bytes (24 rows × 2) per glyph per plane, `0x5A0` per plane
/// pass (30 glyph slots). `ss:` maps to file `+0xC00` (MZ header).
const FONT: usize = 0x27A24;
/// Plane stride in `buf`.
const PLANE: usize = 0x1F40;
/// Plane stride in the font.
const PSTRIDE: usize = 0x5A0;

/// `0x30595` — `0x18` rows × 2 bytes × 4 planes.
pub fn blit(buf: &mut [u8], img: &[u8], idx: u8, di: u16) {
    let s0 = FONT + usize::from(idx) * 0x30;
    for p in 0..4 {
        for r in 0..0x18usize {
            let off = usize::from(di) + r * 0x28;
            if off + 1 < PLANE {
                put(buf, p * PLANE + off, word(img, s0 + p * PSTRIDE + r * 2));
            }
        }
    }
}

/// `0x305FB` — `0x18` rows × 2 zero bytes in every plane (the asm
/// writes through the ambient map mask; a space is blank in all of
/// them).
pub fn clear(buf: &mut [u8], di: u16) {
    for p in 0..4 {
        for r in 0..0x18usize {
            let off = usize::from(di) + r * 0x28;
            if off + 1 < PLANE {
                put(buf, p * PLANE + off, 0);
            }
        }
    }
}

/// One `stosw` — per-byte bounds like `mcg::put`.
fn put(buf: &mut [u8], di: usize, v: u16) {
    let [lo, hi] = v.to_le_bytes();
    if let Some(b) = buf.get_mut(di) {
        *b = lo;
    }
    if let Some(b) = buf.get_mut(di + 1) {
        *b = hi;
    }
}

fn word(img: &[u8], s: usize) -> u16 {
    u16::from_le_bytes([
        img.get(s).copied().unwrap_or(0),
        img.get(s + 1).copied().unwrap_or(0),
    ])
}
