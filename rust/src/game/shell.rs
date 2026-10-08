//! The UI shell — `cs:0x2CC5` (file `0x32CC5`–`0x32D85`), the
//! uimode-0 menu-screen frame loop the galaxy loop `jmp`s back to on
//! `[0x9CDA] & 2`.
//!
//! Entry (`shell_enter`): two identical sound-select blocks (each
//! `0x8A5B`/`al=3 0x8A3B`/`cl=0xA` → `call far [cs:0x127C]`, all gated
//! on `[0x91D3] != 0xFF`) around the `[0x91EA]`/`[0x91E8]` clears —
//! the extra `pop ax` between them discards the return address a menu
//! action abandons when it `jmp`s here instead of returning. Then the
//! screen setup: template slots `0x1260`/`0x125C`, the `0x89B3` CGA
//! border, image 0 backdrop, dispatch-0 palette, slot `0x1272`,
//! `[0x928B] = [0x91A6]`, refresh `0x1264`, the `0xA0AA`/`0xA0AC`
//! hotspot install (`0x31` records, `uimode = 0`), `0x5C97` panel,
//! `0x83AA` mark, `0x6A8E` counter, `0xA3BC` section, `0x1278` image
//! load, `0xA36A` input install.
//!
//! Loop (`shell_step`, one call per iteration): vsync `0x2CA9` →
//! `0x5C1E` sequencer → `0xA369` input stub → `0xA10F` menu →
//! `0x73D9` tick → `0x83AA` mark → `0x5D52` row click → `0x6B4A`
//! blink (tail-falls into the ticker when idle) → `0x8DC6` oval →
//! `0x6B8F` ticker when `[0x91D1] == 0` → `inc [0x91D4]`. The asm
//! loop has no exit test — menu actions `jmp` out of it.

use super::cells::{
    BLINK_T, MENU_COUNT, MENU_LIST, MENU_SEL, OVERLAY_F, SHELL_A6, SHELL_SRC, TICK_FLAG,
};
use super::consts::{FRAME, HOT_COUNT, HOT_LIST, HOT_SEL, SND_ON, UIMODE};
use super::vhost::Call;
use super::vops::Ctx;
use super::{blink, hover, menu, selrec, seq, status, tick_step, ticker, Tick};

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `cs:0x2CC5`–`0x32D5A` — the shell entry/setup half.
pub fn shell_enter(c: &mut Ctx) {
    snd_block(c);
    c.vm.w8(c.st, a!(OVERLAY_F), 0);
    c.vm.w8(c.st, a!(TICK_FLAG), 0);
    snd_block(c);
    setup(c);
}

/// The shared sound-select block — `Chan(0x8A5B)`, `Chan(0x8A3B,3)`,
/// `Sound(0xA)`, all suppressed when `[0x91D3] == 0xFF` (file
/// `0x32CC7`/`0x32CF0`).
fn snd_block(c: &mut Ctx) {
    if c.vm.r8(c.st, a!(SND_ON)) == 0xFF {
        return;
    }
    c.host.svc(Call::Chan(0x8A5B, 0));
    c.host.svc(Call::Chan(0x8A3B, 3));
    c.host.svc(Call::Sound(0xA));
}

/// The `0x2D0A`–`0x2D5A` setup tail — slots, backdrop, hotspot list,
/// panel, mark, status, image load, input install.
fn setup(c: &mut Ctx) {
    c.host.svc(Call::Slot(0x1260, 0));
    c.host.svc(Call::Slot(0x125C, 0));
    c.host.svc(Call::Native(0x89B3));
    c.host.svc(Call::Image(0));
    c.host.svc(Call::Present(0));
    c.host.svc(Call::Slot(0x1272, 0));
    c.vm.w16(c.st, a!(SHELL_A6), c.vm.r16(c.st, a!(SHELL_SRC)));
    c.host.svc(Call::Refresh);
    menu_install(c);
    c.host.svc(Call::PlanetPanel);
    selrec::mark_sel(c);
    status::tick_day(c);
    c.host.svc(Call::Native(0xA3BC));
    c.host.svc(Call::Slot(0x1278, 0));
    c.host.svc(Call::Native(0xA36A));
}

/// The hotspot-list install — `[0x9CC8] = [0xA0AA]`,
/// `[0x9194] = 0xA0AC`, `[0x91A8] = 0x31`, `uimode = 0` (file
/// `0x32D31`–`0x32D47`).
fn menu_install(c: &mut Ctx) {
    let sel = c.vm.r16(c.st, a!(MENU_SEL));
    c.vm.w16(c.st, a!(HOT_SEL), sel);
    c.vm.w16(c.st, a!(HOT_LIST), a!(MENU_LIST));
    c.vm.w16(c.st, a!(HOT_COUNT), MENU_COUNT);
    c.vm.w8(c.st, a!(UIMODE), 0);
}

/// `cs:0x2D5B`–`0x2D84` — one shell frame; returns the `tick_step`
/// arm that ran. The asm loops forever (`jmp 0x2D5B`); exits happen
/// via menu-action `jmp`s out of [`menu::service`].
pub fn shell_step(c: &mut Ctx) -> Tick {
    c.host.svc(Call::Ui(0x2CA9));
    seq::seq_step(c);
    c.host.svc(Call::Native(0xA369));
    menu::service(c);
    let t = tick_step(c);
    selrec::mark_sel(c);
    selrec::row_click(c);
    blink::blink_step(c);
    hover::hover_check(c);
    if c.vm.r8(c.st, a!(BLINK_T)) == 0 {
        ticker::ticker_step(c);
    }
    let f = c.vm.r8(c.st, a!(FRAME)).wrapping_add(1);
    c.vm.w8(c.st, a!(FRAME), f);
    t
}
