//! `impl VmHost for Host` — the `svc`/`svc_st` dispatch plus the
//! per-call bodies the arms need (`rect`, `int33`, `dac_flush`,
//! `load`, `menu_text`, `star`).

use crate::game::{Call, VmHost, SND_ON};
use crate::palette::from_dac;

use super::super::{font, text};
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
/// `ds:[0x9CA1]` — the palette shade offset `0x2ED30`/`0x2F1CF` add to
/// every nonzero component (lives in the saveable `State` block).
const SHADE: u16 = 0x9CA1;

impl Host {
    /// The `[0x1262]` popup frame — `(ax,bx)`/`(cx,dx)` are the opposing
    /// corners in `(4px-col, row)` units, order unspecified; normalised
    /// and drawn as a 1px outline (approximation of `0x2EDCE`; the asm
    /// fills black, the outline keeps the bordered look in every mode).
    fn rect(&mut self, ax: u16, bx: u16, cx: u16, dx: u16) {
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
    fn int33(&mut self, ax: u16, cx: u16, dx: u16) {
        match ax {
            4 => self.pump.move_to(cx, dx),
            8 => self.pump.set_range(cx, dx),
            0x14 => self.pump.set_handler(cx),
            _ => self.dropped += 1,
        }
    }

    /// `Call::Load` — swap the active `AssetSet` via the loader;
    /// `false` when unconfigured or the pair can't be read (the asm's
    /// `0x8AC0` insert-disk retry isn't modeled).
    fn load(&mut self, ofs: u16) -> bool {
        let Some(loader) = &mut self.loader else {
            return false;
        };
        loader.load(&mut self.set, ofs)
    }

    /// `Call::Text` — the `menu_text` page walk (`front/text.rs`);
    /// undrawn glyph tiles count toward `dropped`.
    fn menu_text(&mut self, cur: u16) -> u16 {
        let (ax, glyphs) = text::page(&self.img, &mut self.text_cur, cur);
        self.dropped += glyphs;
        ax
    }

    /// `Call::Stars` — `0x8891`'s `cs:0x4EC4` walk marks `cx + 1`
    /// stars in `ink`.
    fn star(&mut self, cx: u16) {
        let ink = self.ink;
        self.stars
            .step(usize::from(cx), &mut |x, y| self.scr.put_pixel(x, y, ink));
    }

    /// `[0x1258]`/`[0x1260]` (`0x2EDBB`/`0x2EDA8`) — `int 10/AX=1012`
    /// over `CX=0x100` regs from `cs:0x1384` (the earlier reg-0 note
    /// misread `CX`). The template ships the buffer zeroed — the
    /// refresh pairs use it as an instant fade-to-black before the
    /// next `Present`. MCGA-only: the EGA/TGA slots are `int 10/AX=1002`
    /// palette-register writes (`0x30038`/`0x31AF8`, covered by the
    /// fixed palettes) and CGA's is `int 10/AH=5 AL=1` page select
    /// (`0x3081E`, one buffer).
    fn dac_flush(&mut self) {
        self.pal = from_dac(self.img.get(PAL_BUF..PAL_BUF + 0x300).unwrap_or(&[]));
    }
}

impl VmHost for Host {
    fn svc(&mut self, call: Call) -> u16 {
        match call {
            Call::Image(i) => self.scr.draw_image(&self.set, usize::from(i)),
            Call::Glyph(ch, bx, dx) => font::draw(&mut self.scr, &self.img, ch, bx, dx),
            Call::Rect(a, b, x, d) => self.rect(a, b, x, d),
            Call::Mouse(ax, cx, dx) => self.int33(ax, cx, dx),
            Call::KeyPop => return u16::from(self.pump.pop_key().unwrap_or(0)),
            Call::Slot(0x125C, _) => self.scr.buf.fill(0),
            Call::Slot(0x1258 | 0x1260, _) => {
                if self.scr.mode.programs_palette() {
                    self.dac_flush();
                }
            }
            Call::Sound(cl) => self.snd.sound(cl),
            Call::Chan(t, al) => self.snd.chan(t, al),
            Call::Sfx(ax, cl) => self.snd.sfx(ax, cl),
            Call::Song(seg) => self.snd.song(seg, &self.img),
            Call::SndReset => self.snd.snd_reset(),
            Call::Load(ofs) => {
                if !self.load(ofs) {
                    self.dropped += 1;
                }
            }
            Call::Text(p) => return self.menu_text(p),
            Call::Stars(cx) => self.star(cx),
            Call::Menu
            | Call::Dialog
            | Call::Refresh
            | Call::Present(_)
            | Call::Flash
            | Call::PlanetPanel
            | Call::Blit(..)
            | Call::Slot(..)
            | Call::Native(_)
            | Call::Ui(_)
            | Call::Draw(..) => self.dropped += 1,
        }
        0
    }

    fn svc_st(&mut self, call: Call, vm: &crate::game::Vm, st: &crate::game::State) -> u16 {
        if !self.scr.mode.programs_palette() {
            return self.svc(call);
        }
        match call {
            Call::Present(dx) => self.pal.present(&self.img, dx, vm.r8(st, SHADE)),
            Call::Slot(0x128E, _) => self.pal.pal_fade(&self.img, vm.r8(st, SHADE)),
            _ => return self.svc(call),
        }
        0
    }

    fn cs_word(&mut self, ofs: u16) -> u16 {
        let i = TPL_OFS + usize::from(ofs);
        u16::from_le_bytes([
            self.img.get(i).copied().unwrap_or(0),
            self.img.get(i + 1).copied().unwrap_or(0),
        ])
    }

    fn pump_input(&mut self, vm: &mut crate::game::Vm, st: &mut crate::game::State) {
        if let Some(src) = &mut self.input_src {
            src(&mut self.pump);
        }
        self.pump.write(vm, st);
        // Keep the audio layer's [0x91D3] gate byte in sync — the
        // `0x8A80`/`0x8577` wrappers key song/sfx dispatch off it.
        self.snd
            .set_quiet(vm.r8(st, u16::try_from(SND_ON).unwrap_or(0)));
    }

    fn key_flush(&mut self) {
        self.pump.flush_keys();
    }
}
