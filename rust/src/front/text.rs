//! `menu_text` (`cs:0x2F629`, file `0x2F629`–`0x2F6C9`) — the paged
//! text-stream renderer the ending sequences call through `[0x1296]`.
//!
//! The stream lives at `ss:0x20000+` (the tail of the unpacked image):
//! each page is `{fb14, fb15}` header, `{xoff}` column, then ops —
//! `0x00` ends the stream (the caller latches `[0x9CCB]`), `0xFF`
//! suspends with the cursor left on the next page header, `0xF0`+arg
//! starts a new line, `0x20` is a space and any other byte is a
//! 16×24 glyph tile the asm blits via `0x2F4E3`. The glyphs aren't
//! drawn here (the menu font isn't the HUD font); they're counted for
//! the caller's `dropped` tally.

/// `ss:` stream base in the unpacked image (`file 0x20000`).
const STREAM: usize = 0x20000;

/// One page at `img[STREAM + *cur]` (`u16::MAX` keeps `*cur`).
/// Returns `(ax, glyphs)` — `ax` = `0x100` when the `0x00` terminator
/// ran, OR'd with the page's `fb15` byte (the scroll variants test
/// it); `glyphs` counts undrawn tile ops.
pub fn page(img: &[u8], cur: &mut u16, arg: u16) -> (u16, usize) {
    if arg != u16::MAX {
        *cur = arg;
    }
    let mut di = STREAM + usize::from(*cur);
    let fb = u16::from(byte(img, di + 1));
    di += 2;
    let mut glyphs = 0usize;
    loop {
        di += 1; // the line's xoff
        loop {
            let op = byte(img, di);
            di += 1;
            match op {
                0x00 => return (0x100 | fb, glyphs),
                0xFF => {
                    *cur = u16::try_from(di - STREAM).unwrap_or(0);
                    return (fb, glyphs);
                }
                0xF0 => {
                    di += 1; // the newline arg
                    break;
                }
                _ => glyphs += 1,
            }
        }
    }
}

fn byte(img: &[u8], i: usize) -> u8 {
    img.get(i).copied().unwrap_or(0)
}
