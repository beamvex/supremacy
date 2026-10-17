//! TGA `menu_text` tile painters — the `0x31C2B` glyph blit and the
//! `0x31C72` space clear (file `0x31C2B`–`0x31CA9`). `si`/`di` is the
//! four-bank interleaved offset (`(y>>2)·0xA0 + (y&3)·0x2000`); each
//! row writes 8 bytes then `di` steps `+0x2000`, wrapping `0x8000`
//! back to `+0xA0` — the TGA variant of the CGA bank walk.

/// File offset of the `ss:0x60B4` menu font — `0xC0` bytes
/// (24 rows × 8) per glyph. `ss:` maps to file `+0xC00` (MZ header).
const FONT: usize = 0x263B4;
/// Bank step, wrap span and line step.
const BANK: usize = 0x2000;
const SPAN: usize = 0x8000;
const PITCH: usize = 0xA0;

/// `0x31C2B` — `0x18` rows × 8 bytes (4 `movsw`), `di` bank-walked.
pub fn blit(buf: &mut [u8], img: &[u8], idx: u8, si: u16) {
    let mut s = FONT + usize::from(idx) * 0xC0;
    let mut di = usize::from(si);
    for _ in 0..0x18 {
        for _ in 0..4 {
            put(buf, di, word(img, s));
            s += 2;
            di += 2;
        }
        di = next(di);
    }
}

/// `0x31C72` — `0x18` rows × 8 zero bytes, same walk.
pub fn clear(buf: &mut [u8], si: u16) {
    let mut di = usize::from(si);
    for _ in 0..0x18 {
        for _ in 0..4 {
            put(buf, di, 0);
            di += 2;
        }
        di = next(di);
    }
}

/// `di += 0x2000; di ≥ 0x8000 → di -= 0x8000 − 0xA0` — the banked
/// next-row step after an 8-byte write (`8` backed out first).
fn next(di: usize) -> usize {
    let di = di - 8 + BANK;
    if di >= SPAN {
        di - SPAN + PITCH
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
