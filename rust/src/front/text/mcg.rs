//! MCG `menu_text` tile painters — the `0x2F4E3` glyph blit and the
//! `0x2F519` space clear (file `0x2F4E3`–`0x2F540`). `es:di` is the
//! linear 8bpp offset, `di += 0x130` the inside-tile row advance
//! (320 − 16).

/// File offset of the `cs:0x21C4` 16×24 menu font (`+0x292B0` template
/// space) — one `0x180`-byte opaque record per glyph, `op − 0x41`
/// index, 8 words × 24 rows, colours baked.
const FONT: usize = 0x2B474;

/// `0x2F4E3` — 8 `stosw` per row × `0x18` rows from
/// `cs:[idx·0x180 + 0x21C4]` into `es:di`.
pub(super) fn blit(buf: &mut [u8], img: &[u8], idx: u8, di: u16) {
    let mut s = FONT + ((usize::from(idx) * 0x180) & 0xFFFF);
    let mut di = usize::from(di);
    for _ in 0..0x18 {
        for _ in 0..8 {
            put(buf, di, word(img, s));
            s += 2;
            di += 2;
        }
        di += 0x130;
    }
}

/// `0x2F519` — `rep stosw` of 8 zero words per row × `0x18` rows.
pub(super) fn clear(buf: &mut [u8], di: u16) {
    let mut di = usize::from(di);
    for _ in 0..0x18 {
        if let Some(row) = buf.get_mut(di..di + 0x10) {
            row.fill(0);
        }
        di += 0x140;
    }
}

/// One `stosw` — the bytes past the modelled VRAM span are dropped.
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
