//! `impl VmHost for Host` — the `svc`/`svc_st` dispatch plus the
//! per-call bodies the arms need (`load`, `menu_text`, `scroll`,
//! `star`; the draw helpers live in [`draw`][super::draw]).

use crate::game::{Call, VmHost, SND_ON};

use super::super::font;
use super::Host;

/// `cs:` near offset → file offset — the real code segment
/// (`CS = 0x286B` + the `0xC00` MZ header): `cs:0x58DA` = file
/// `0x2EB8A`, `cs:0x5C1E` = file `0x2EECE` (`dispatch_tables.txt`).
/// Every runtime `cs:` value lives in this space — the `0x1384` DAC
/// table, the dispatch targets and the `[0x9198]` seq streams. The
/// `file − 0x30000` `cs:` names elsewhere in the docs are labels only.
const TPL_OFS: usize = 0x292B0;
/// `ds:[0x9CA1]` — the palette shade offset `0x2ED30`/`0x2F1CF` add to
/// every nonzero component (lives in the saveable `State` block).
const SHADE: u16 = 0x9CA1;

impl Host {
    /// `Call::Load` — swap the active `AssetSet` via the loader;
    /// `false` when unconfigured or the pair can't be read (the asm's
    /// `0x8AC0` insert-disk retry isn't modeled).
    fn load(&mut self, ofs: u16) -> bool {
        let Some(loader) = &mut self.loader else {
            return false;
        };
        loader.load(&mut self.set, ofs)
    }

    /// `Call::Text` — the `menu_text` crawl (`front/text.rs`): a fresh
    /// cursor arms the sequence — the first block paints in place and
    /// the snow is swept before it shows; `u16::MAX` pulls the next
    /// block. All four video modes draw their tiles (GAM-59).
    fn menu_text(&mut self, cur: u16) -> u16 {
        let ax = self.crawl.call(&mut self.scr, &self.img, cur);
        if cur != u16::MAX {
            self.stars.clear(&mut self.scr, self.ink);
        }
        ax
    }

    /// `Call::Scroll` — the crawl step the shipped `[0x1290]`/`0x8983`
    /// `ret`s stand in for: shift the framebuffer up `px` rows and feed
    /// the pending block in at the bottom.
    fn scroll(&mut self, px: u16) {
        self.crawl.step(&mut self.scr, usize::from(px));
    }

    /// `Call::Stars` — `0x8891`'s `cs:0x4EC4` walk; the MCG `[0x1294]`
    /// mark erases each star's previous pixel and only plots onto
    /// empty cells (no trails, no text clobber). While the crawl is
    /// scrolling the marks stay suppressed — the snow is swept.
    fn star(&mut self, cx: u16) {
        self.stars.step(
            usize::from(cx),
            &mut self.scr,
            self.ink,
            !self.crawl.scrolling(),
        );
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
            Call::Slot(0x125C, _) => {
                self.crawl.reset();
                self.scr.buf.fill(0);
            }
            Call::Slot(0x127C, _) => self.grid(0x7120, 0x66, 0x48, 0xB0),
            Call::Slot(0x127E, _) => self.grid(0x4610, 0x75, 0x28, 0xF0),
            Call::Slot(0x1280, _) => self.grid(0x4670, 0x75, 0x20, 0x100),
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
            Call::Scroll(px) => self.scroll(px),
            Call::Stars(cx) => self.star(cx),
            Call::Menu
            | Call::Dialog
            | Call::Refresh
            | Call::Present(_)
            | Call::Flash
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
            Call::Slot(0x1278, _) => self.map_draw(vm, st),
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
        if let Some(mut src) = self.vsync_src.take() {
            src(self);
            self.vsync_src.get_or_insert(src);
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
