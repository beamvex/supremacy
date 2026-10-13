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
    /// The `[0x1262]` popup frame — `(ax,bx)`/`(cx,dx)` are the opposing
    /// corners in `(4px-col, row)` units, order unspecified; normalised
    /// and drawn as a 1px outline (approximation of `0x2EDCE`; the asm
    /// fills black, the outline keeps the bordered look in every mode).
    pub(super) fn rect(&mut self, ax: u16, bx: u16, cx: u16, dx: u16) {
        let (l, r) = (ax.min(cx).min(79) * 4, ax.max(cx).min(79) * 4 + 3);
        let (t, bot) = (bx.min(dx).min(199), bx.max(dx).min(199));
        for x in usize::from(l)..=usize::from(r) {
            self.scr.put_pixel(x, usize::from(t), self.ink);
            self.scr.put_pixel(x, usize::from(bot), self.ink);
        }
        for y in usize::from(t)..=usize::from(bot) {
            self.scr.put_pixel(usize::from(l), y, self.ink);
            self.scr.put_pixel(usize::from(r), y, self.ink);
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
}
