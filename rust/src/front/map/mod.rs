//! The `[0x1278]` `map_draw` (MCG `0x2F2C5`) and the `[0x127C]`/`[0x127E]`/
//! `[0x1280]` star-grid painters (MCG `0x2F398`/`0x2F3CB`/`0x2F3FE`, in
//! [`grid`]).
//!
//! `map_draw` repaints the system view: walks the `[0x9158]` record array
//! (`[0x91B8]`+1 records — the appended faction record included),
//! restores each record's saved under-byte, maps the record's orbit words
//! to a map cell `(+0xA + 0x40)·80 + 0x50 + (sin·radius >> 14)` where
//! `sin` is `cs:[angle + [0x91D0] + 0x1284]`, saves the cell + under-byte
//! back, and writes `quadrant + kind` (kind `^0xD` while `[0x91D4]&0xF`
//! ≥ 8 on occupied colonies). The selected record's cell feeds the
//! `ds:0x4C95` route overlay drawn last. `[0x9CCA]` = `1`/`2` suppress
//! the writes during save/load image swaps.
//!
//! The `ds:` cells only the `map_draw` family touches — `[0x91D0]`,
//! `[0x9168]`, `[0x9174]`, the records' `+0`/`+4` and the routes' `+2`/`+6`
//! — are mirrored here: `svc_st` hands the host a read-only `&State` and
//! nothing else reads them.

mod grid;
mod pos;

pub use grid::grid;

use crate::game::{State, Vm};
use pos::{blink, pos};

/// `cs:` near offset → file offset (`CS = 0x286B` + the `0xC00` header).
const TPL_OFS: usize = 0x292B0;
/// `ds:` record-array base pointer.
const REC_BASE: u16 = 0x9158;
/// `ds:` planet count — the faction record sits at index `count`.
const REC_COUNT: u16 = 0x91B8;
/// `ds:` `[0x1278]`-slot mode byte — `1`/`2` suppress the map writes.
const SLOT_MODE: u16 = 0x9CCA;
/// `ds:` route overlay — `0x12` records `{rel_cell, saved_cell, a, b,
/// saved_under}` of 7 bytes.
const ROUTES: u16 = 0x4C95;
/// Route record count.
const ROUTE_N: usize = 0x12;
/// Record byte stride.
const REC_STRIDE: u16 = 0x3A;

/// The per-record `{+0 saved cell, +4 saved under}` mirror.
#[derive(Clone, Copy, Default)]
pub(super) struct Mark {
    /// Saved cell.
    pub cell: u16,
    /// Saved under-byte.
    pub under: u8,
}

/// Host mirror of the `map_draw` `ds:` cells.
pub struct Map {
    /// `[0x91D0]` — scroll phase, `+= 2` per draw.
    scroll: u8,
    /// `[0x9168]` — the selected record's map cell.
    sel_cell: u16,
    /// `[0x9174]` — route-dot variant selector.
    route_alt: u16,
    /// Record array base/count the marks mirror was built for.
    base: u16,
    /// Mirrored count.
    count: u16,
    /// Per-record save cells.
    marks: Vec<Mark>,
    /// Per-route `{saved cell (+2), saved under (+6)}` cells.
    routes: [(u16, u8); ROUTE_N],
}

impl Map {
    /// A clear mirror — cells all zero like the shipped `ds:` block.
    #[must_use]
    pub fn new() -> Self {
        Self {
            scroll: 0,
            sel_cell: 0,
            route_alt: 0,
            base: 0,
            count: 0,
            marks: Vec::new(),
            routes: [(0, 0); ROUTE_N],
        }
    }

    /// `0x2F2C5` — route restore, the record walk, route draw.
    pub fn draw(&mut self, buf: &mut [u8], img: &[u8], vm: &Vm, st: &State) {
        self.routes_restore(buf);
        self.sync(vm, st);
        self.scroll = self.scroll.wrapping_add(2);
        for i in 0..=self.count {
            self.rec(buf, img, vm, st, i);
        }
        self.routes_draw(buf, vm, st);
    }

    /// Rebuild the marks mirror when the record array moves/resizes.
    fn sync(&mut self, vm: &Vm, st: &State) {
        let (b, n) = (vm.r16(st, REC_BASE), vm.r16(st, REC_COUNT));
        if b != self.base || n != self.count {
            self.base = b;
            self.count = n;
            self.marks = vec![Mark::default(); usize::from(n) + 1];
        }
    }

    /// One record — restore, position, save, draw (file
    /// `0x2F2D5`–`0x2F38B`).
    fn rec(&mut self, buf: &mut [u8], img: &[u8], vm: &Vm, st: &State, i: u16) {
        let si = self.base.wrapping_add(i.wrapping_mul(REC_STRIDE));
        let mode = vm.r8(st, SLOT_MODE);
        let mut m = self.marks[usize::from(i)];
        if mode != 2 {
            put(buf, m.cell, m.under);
        }
        let (cell, q) = pos(img, vm, st, si, self.scroll, &mut m, buf);
        self.marks[usize::from(i)] = m;
        self.select(vm, st, si, cell, q);
        let q = if q == 3 { 1 } else { q };
        if mode != 1 {
            put(buf, cell, q.wrapping_add(blink(vm, st, si)));
        }
    }

    /// The selected-record latch — `[0x9168]` and the `[0x9174]`
    /// route-variant pick (file `0x2F32C`–`0x2F34C`).
    fn select(&mut self, vm: &Vm, st: &State, si: u16, cell: u16, q: u8) {
        if vm.r8(st, si + 5) != 0 {
            self.sel_cell = cell;
            self.route_alt = u16::from(q != 2);
        }
    }

    /// `0xF440` — restore every route dot's saved under-byte.
    fn routes_restore(&mut self, buf: &mut [u8]) {
        for (cell, under) in self.routes {
            put(buf, cell, under);
        }
    }

    /// `0xF457` — save under + cell per route, then paint the `+4`/`+5`
    /// variant colour at `sel_cell + rel` (skipped on `[0x9CCA] == 1`).
    fn routes_draw(&mut self, buf: &mut [u8], vm: &Vm, st: &State) {
        if vm.r8(st, SLOT_MODE) == 1 {
            return;
        }
        for i in 0..ROUTE_N {
            let rel = vm.r16(st, ROUTES + u16::try_from(i * 7).unwrap_or(0));
            let cell = self.sel_cell.wrapping_add(rel);
            self.routes[i] = (cell, get(buf, cell));
        }
        for i in 0..ROUTE_N {
            let si = ROUTES + u16::try_from(i * 7).unwrap_or(0);
            let col = vm.r8(st, si + 4 + self.route_alt.min(1));
            put(buf, self.sel_cell.wrapping_add(vm.r16(st, si)), col);
        }
    }
}

/// `es:` byte fetch — off-buffer cells read 0.
fn get(buf: &[u8], i: u16) -> u8 {
    buf.get(usize::from(i)).copied().unwrap_or(0)
}

/// `es:` byte store — off-buffer writes drop.
fn put(buf: &mut [u8], i: u16, v: u8) {
    if let Some(p) = buf.get_mut(usize::from(i)) {
        *p = v;
    }
}

/// `cs:` word fetch.
fn cs_word(img: &[u8], ofs: u16) -> u16 {
    let i = TPL_OFS + usize::from(ofs);
    u16::from_le_bytes([
        img.get(i).copied().unwrap_or(0),
        img.get(i + 1).copied().unwrap_or(0),
    ])
}

/// `cs:` word store — the `0xE93C` seed writeback.
fn cs_put(img: &mut [u8], ofs: u16, v: u16) {
    let i = TPL_OFS + usize::from(ofs);
    for (k, b) in v.to_le_bytes().iter().enumerate() {
        if let Some(p) = img.get_mut(i + k) {
            *p = *b;
        }
    }
}
