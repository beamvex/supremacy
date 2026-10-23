//! `Host` draw helpers — the `[0x1262]` popup frame, the `int 33`
//! mouse trap and the DAC/`map_draw`/star-grid slot bodies the `svc`
//! dispatch arms call.

use crate::args::Video;
use crate::game::{State, Vm};
use crate::palette::from_dac;

use super::super::map;
use super::Host;

/// `cs:` near offset → file offset — the real code segment
/// (`CS = 0x286B` + the `0xC00` MZ header): `cs:0x58DA` = file
/// `0x2EB8A`, `cs:0x5C1E` = file `0x2EECE` (`dispatch_tables.txt`).
/// Every runtime `cs:` value lives in this space — the `0x1384` DAC
/// table, the dispatch targets and the `[0x9198]` seq streams. The
/// `file − 0x30000` `cs:` names elsewhere in the docs are labels only.
const TPL_OFS: usize = 0x292B0;
/// `cs:0x1384` in template space — the 768-byte DAC table the
/// `[0x1258]`/`[0x1260]` flushes program all 256 regs from (file
/// `0x2A634`; ships zeroed and nothing writes it = instant black).
const PAL_BUF: usize = 0x1384 + TPL_OFS;

impl Host {
    /// The `[0x1262]` region clear (MCG `0x2EDCE`, file
    /// `0x2EDCE`–`0x2EE1E`): `ax` = left edge in 8-px columns, `bx` =
    /// top row, `cx` = `width/16 − 1`, `dx` = height in rows — then
    /// `rep stosw`s colour 0 over the box. A black fill, not a frame:
    /// the EGA twin `0x30198` runs the same register shuffle into a
    /// per-plane `stosb` of the just-cleared background byte, and the
    /// CGA/TGA twins match.
    pub(super) fn rect(&mut self, ax: u16, bx: u16, cx: u16, dx: u16) {
        let (x, w) = (usize::from(ax) * 8, usize::from(cx) * 16 + 16);
        let (y, h) = (usize::from(bx), usize::from(dx));
        for row in y..(y + h).min(200) {
            for col in x..(x + w).min(320) {
                self.scr.put_pixel(col, row, 0);
            }
        }
    }

    /// `int 33` — `ax = 4` sets the position, `ax = 8` the y range,
    /// `ax = 0x14` swaps the event-handler mask (`0xA3BC`/`0xA36A` —
    /// `cx` = `0x1E` buttons-only vs `0x1F` full; `dx` is the handler
    /// offset the port doesn't model).
    pub(super) fn int33(&mut self, ax: u16, cx: u16, dx: u16) {
        match ax {
            4 => self.pump.move_to(cx, dx),
            8 => self.pump.set_range(cx, dx),
            0x14 => self.pump.set_handler(cx),
            _ => self.dropped += 1,
        }
    }

    /// `[0x1258]`/`[0x1260]` (`0x2EDBB`/`0x2EDA8`) — `int 10/AX=1012`
    /// over `CX=0x100` regs from `cs:0x1384` (the earlier reg-0 note
    /// misread `CX`). The template ships the buffer zeroed — the
    /// refresh pairs use it as an instant fade-to-black before the
    /// next `Present`. MCGA-only: the EGA/TGA slots are `int 10/AX=1002`
    /// palette-register writes (`0x30038`/`0x31AF8`, covered by the
    /// fixed palettes) and CGA's is `int 10/AH=5 AL=1` page select
    /// (`0x3081E`, one buffer).
    pub(super) fn dac_flush(&mut self) {
        self.pal = from_dac(self.img.get(PAL_BUF..PAL_BUF + 0x300).unwrap_or(&[]));
    }

    /// `[0x1278]` `map_draw` — the MCG `0x2F2C5` walk in [`map`]; the
    /// other modes' twins (`0x2F89E`/`0x30CFE`/`0x31824`) aren't ported.
    pub(super) fn map_draw(&mut self, vm: &Vm, st: &State) {
        if self.scr.mode == Video::Mcga {
            self.map.draw(&mut self.scr.buf, &self.img, vm, st);
        } else {
            self.dropped += 1;
        }
    }

    /// A `[0x127C]`/`[0x127E]`/`[0x1280]` star-grid painter call — the
    /// `di`/`rows`/`pairs`/`tail` parameters are the MCG slot's.
    pub(super) fn grid(&mut self, di: u16, rows: u16, pairs: u16, tail: u16) {
        if self.scr.mode == Video::Mcga {
            map::grid(&mut self.scr.buf, &mut self.img, di, rows, pairs, tail);
        } else {
            self.dropped += 1;
        }
    }

    /// `[0x1270]` — the message-window scroll the `0x6B86` blink tail
    /// runs once per ticker `0xFD` boundary (MCG `0x5D40`, EGA `0x6DF1`,
    /// CGA `0x7743`, TGA `0x8209`): the (x `0xA0..0x130`, y `0x5B..0xBE`)
    /// region shifted up one 6px text line and the bottom rows blanked,
    /// so each new ticker line pushes the previous up instead of
    /// overwriting it. (The asm's `[0x1298]` head only registers a
    /// hover rect the port doesn't track; `0xA479` is the present.)
    pub(super) fn msg_scroll(&mut self) {
        if self.msg_snow {
            // The new-game wait's `[0x127C]` fill left the window's
            // star grid painted — sweep its footprint before the copy
            // carries the dots up behind the text (GAM-60). Rows
            // `0x5A`/`0xBE..0xC0` sit outside the scroll's span, so the
            // sweep has to cover the painter's whole rect.
            self.msg_snow = false;
            for y in 0x5A..0xC0usize {
                for x in 0xA0..0x130usize {
                    self.scr.put_pixel(x, y, 0);
                }
            }
        }
        match self.scr.mode {
            Video::Mcga => scroll_region(&mut self.scr.buf, 0x140, 0xA0, 0x90, |_, _| 0),
            Video::Ega => {
                for p in 0..4usize {
                    let base = p * 0x1F40;
                    let fill = self.scr.buf[base + 0x30];
                    scroll_region(
                        &mut self.scr.buf[base..base + 0x1F40],
                        0x28,
                        0x14,
                        0x12,
                        |_, _| fill,
                    );
                }
            }
            Video::Cga => banked(&mut self.scr.buf, cga_ofs, 0x24),
            Video::Tga => banked(&mut self.scr.buf, tga_ofs, 0x48),
        }
    }
}

/// Window top/bottom-1 rows and the 6px line pitch, shared by all four
/// variants (dst rows `Y0..Y0+ROWS` pull from `y + LINE`; `Y0+ROWS+1..`
/// blanked — the asm's `bx = 0x5C`/`0x6` counts).
const Y0: usize = 0x5B;
const ROWS: usize = 0x5C;
const LINE: usize = 6;

/// Linear-region scroll — `ROWS` rows of `w` bytes at `x` byte offset,
/// `pitch`-strided; `fill(y, i)` supplies the blank-row byte.
fn scroll_region(
    buf: &mut [u8],
    pitch: usize,
    x: usize,
    w: usize,
    fill: impl Fn(usize, usize) -> u8,
) {
    let old = buf.to_vec();
    for y in Y0..Y0 + ROWS {
        let (d, s) = (y * pitch + x, (y + LINE) * pitch + x);
        buf[d..d + w].copy_from_slice(&old[s..s + w]);
    }
    for (i, y) in (Y0 + ROWS + 1..Y0 + ROWS + 1 + LINE).enumerate() {
        buf[y * pitch + x..y * pitch + x + w].fill(fill(y, i));
    }
}

/// CGA visual row → byte offset of the window's first byte.
fn cga_ofs(y: usize) -> usize {
    (y >> 1) * 0x50 + (y & 1) * 0x2000 + 0x28
}

/// TGA visual row → byte offset of the window's first byte.
fn tga_ofs(y: usize) -> usize {
    (y >> 2) * 0xA0 + (y & 3) * 0x2000 + 0x50
}

/// Banked-layout scroll — same region, per-row offsets via `ofs` (the
/// clone covers the bank-interleaved src/dst overlap).
fn banked(buf: &mut [u8], ofs: fn(usize) -> usize, w: usize) {
    let old = buf.to_vec();
    for y in Y0..Y0 + ROWS {
        let (d, s) = (ofs(y), ofs(y + LINE));
        buf[d..d + w].copy_from_slice(&old[s..s + w]);
    }
    for y in Y0 + ROWS + 1..Y0 + ROWS + 1 + LINE {
        buf[ofs(y)..ofs(y) + w].fill(0);
    }
}
