//! Shared pieces of the ending sequences — the `0x8891` starframe,
//! the `[0x1296]` `menu_text` call, the input-latch tests, the
//! seq-stream arm and the `[0x1260]`/`[0x125C]` refresh pair.

use super::super::consts::{KEY_PEND, SEQ_DELAY, SEQ_PERIOD, SEQ_PHASE, SEQ_PTR};
use super::super::vhost::Call;
use super::super::vops::{vsync, Ctx};
use super::seq::SCENARIO;

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `[0x9CA1]` — the pal shade offset the image phases write.
pub(super) const SHADE: u16 = 0x9CA1;
/// `[0x9205]` — starfield aux cleared by the seq arm.
const STAR_AUX: u16 = 0x9205;
/// `[0x9206]` — the live star count; grows toward `0x63` in `0x8891`.
const STARS: u16 = 0x9206;

/// `0x8891` — one starfield frame: `0x2CA9` vsync, the `cs:0x4EC4`
/// scroll, then `[0x9206]` grows toward `0x63` (file `0x38891`).
pub(super) fn stars(c: &mut Ctx) {
    vsync(c);
    let n = c.vm.r16(c.st, STARS);
    c.host.svc(Call::Stars(n));
    if n != 0x63 {
        c.vm.w16(c.st, STARS, n.wrapping_add(1));
    }
}

/// `call word [0x1296]` — `menu_text` on `cur` (`u16::MAX` keeps
/// `cs:0xF672`). Mirrors the asm's `[0x9CCB] = 0xFF` end write and
/// returns `cs:0xFB15 != 0` for the scroll variants.
pub(super) fn text_page(c: &mut Ctx, cur: u16) -> bool {
    let r = c.host.svc(Call::Text(cur));
    if r & 0x100 != 0 {
        c.vm.w8(c.st, a!(KEY_PEND), 0xFF);
    }
    r & 0xFF != 0
}

/// `test byte [0x9CCB],0xFF` — stream-end or a latched key.
pub(super) fn pending(c: &mut Ctx) -> bool {
    c.vm.r8(c.st, a!(KEY_PEND)) != 0
}

/// `cmp word [0x91E5]` — the scenario index.
pub(super) fn scenario(c: &mut Ctx) -> u16 {
    c.vm.r16(c.st, SCENARIO)
}

/// A's page by scenario — `0x5712`/`0x578A`/`0x5802`/`0x587A` (file
/// `0x38B67`–`0x38B85`).
pub(super) fn page_a(c: &mut Ctx) -> u16 {
    match scenario(c) {
        0 => 0x5712,
        1 => 0x578A,
        2 => 0x5802,
        _ => 0x587A,
    }
}

/// `[0x9CA1]=0, [0x9205]=0, [0x91D6]=8, [0x9198]=stream, [0x91A0]=0,
/// [0x91D7]=0` — the arm before the image phase.
pub(super) fn seq_arm(c: &mut Ctx, stream: u16) {
    c.vm.w8(c.st, SHADE, 0);
    c.vm.w8(c.st, STAR_AUX, 0);
    c.vm.w8(c.st, a!(SEQ_PERIOD), 8);
    c.vm.w16(c.st, a!(SEQ_PTR), stream);
    c.vm.w16(c.st, a!(SEQ_DELAY), 0);
    c.vm.w8(c.st, a!(SEQ_PHASE), 0);
}

/// The `0x8BE8`/`0x8D47` reveal — `[0x9CCB]=0`, `Image(0)`, shade 0,
/// `Present(dx)`, shade `0xF`, period `8` and the stream re-arm.
pub(super) fn reveal(c: &mut Ctx, dx: u16, stream: u16) {
    c.vm.w8(c.st, a!(KEY_PEND), 0);
    c.host.svc(Call::Image(0));
    c.vm.w8(c.st, SHADE, 0);
    c.host.svc(Call::Present(dx));
    c.vm.w8(c.st, SHADE, 0xF);
    c.vm.w8(c.st, a!(SEQ_PERIOD), 8);
    c.vm.w16(c.st, a!(SEQ_PTR), stream);
}

/// `[0x1260]` + `[0x125C]` — the DAC-write/screen-clear pair.
pub(super) fn refresh(c: &mut Ctx) {
    c.host.svc(Call::Slot(0x1260, 0));
    c.host.svc(Call::Slot(0x125C, 0));
}
