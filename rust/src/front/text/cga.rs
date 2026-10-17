//! CGA `menu_text` tile painters — the `0x30F89` glyph blit and the
//! `0x30FCB` space clear (file `0x30F89`–`0x30FFD`). `si`/`di` is the
//! banked offset (`(y>>1)·0x50 + (y&1)·0x2000`); each row writes 4
//! bytes then `di` walks `^0x2000` with a `+0x50` on the bank-1→0
//! edge — the same interlace [`crate::video::Banked`] uses.

/// File offset of the `cs:0x1684` menu font — `0x60` bytes
/// (24 rows × 4) per glyph. `cs:` maps to file `+0x292B0`.
const FONT: usize = 0x2A934;
/// Bank toggle and line step.
const BANK: usize = 0x2000;
const PITCH: usize = 0x50;

/// `0x30F89` — `0x18` rows × 4 bytes, `di` bank-walked.
pub fn blit(buf: &mut [u8], img: &[u8], idx: u8, si: u16) {
    let mut s = FONT + usize::from(idx) * 0x60;
    let mut di = usize::from(si);
    for _ in 0..0x18 {
        for _ in 0..2 {
            put(buf, di, word(img, s));
            s += 2;
            di += 2;
        }
        di = next(di);
    }
}

/// `0x30FCB` — `0x18` rows × 4 zero bytes, same walk.
pub fn clear(buf: &mut [u8], si: u16) {
    let mut di = usize::from(si);
    for _ in 0..0x18 {
        for _ in 0..2 {
            put(buf, di, 0);
            di += 2;
        }
        di = next(di);
    }
}

/// `di -= w; di ^= 0x2000; di < 0x2000 → di += 0x50` — the banked
/// next-row step after a 4-byte write (`w = 4` backed out first).
fn next(di: usize) -> usize {
    let di = (di - 4) ^ BANK;
    if di < BANK {
        di + PITCH
    } else {
        di
    }
}

/// One `stosw` — bytes past the modelled VRAM span are dropped.
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
