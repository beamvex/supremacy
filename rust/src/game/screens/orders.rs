//! The uimode-3 screen — `cs:0x9F42`/`0x9F45` (file
//! `0x331F2`–`0x332DA`), the 0x2A-record list (`0xA85A`) the planet
//! detail "orders" buttons live in.

use super::super::consts::{
    BUTTONS, DIRTY0, DIRTY1, REC_CUR, SEL_REC, SEQ_AUX, SEQ_DELAY, SEQ_PERIOD, SEQ_PHASE, SEQ_PTR,
};
use super::super::vhost::Call;
use super::super::vops::{faction_rec, kind_of, set_bits, vsync, Ctx};
use super::super::{dialog, menu, selrec, seq, status, tick_step};
use super::{bump, install, leave};

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `cs:0x9F42` — orders entry: the `0x2CC5`-pattern sound block, the
/// `[0x9198]`/`[0x919A]`/`[0x9144]`/`[0x9146]` clears, the kind check
/// that re-points `[0x917C]` at the faction record when the selection
/// isn't `kind 0xA`, then the `0xA85A` list, `uimode 3` and the frame
/// loop. `win` is the `0x9F45` variant — the `[0x1274]` window slot
/// first (file `0x331F2`–`0x332AD`).
pub fn enter_orders(c: &mut Ctx, win: bool) {
    if win {
        c.host.svc(Call::Slot(0x1274, 0));
    }
    dialog::snd_block(c);
    c.vm.w16(c.st, a!(SEQ_PTR), 0);
    c.vm.w16(c.st, a!(SEQ_AUX), 0);
    c.vm.w16(c.st, a!(DIRTY0), 0);
    c.vm.w16(c.st, a!(DIRTY1), 0);
    let cur = c.vm.r16(c.st, a!(REC_CUR));
    let sel = if kind_of(c, cur) == 0xA {
        cur
    } else {
        faction_rec(c)
    };
    c.vm.w16(c.st, a!(SEL_REC), sel);
    set_bits(c, a!(DIRTY0), 0xC3F6);
    set_bits(c, a!(DIRTY1), 7);
    install(c, 0xA858, 0xA85A, 0x2A, 3);
    c.host.svc(Call::Slot(0x1260, 0));
    c.host.svc(Call::Slot(0x125C, 0));
    c.host.svc(Call::Native(0x89C3));
    c.host.svc(Call::Image(0x40));
    c.host.svc(Call::Present(3));
    c.host.svc(Call::Native(0x38CE));
    c.host.svc(Call::Native(0x6F53));
    status::tick_day(c);
    c.host.svc(Call::Native(0x334F));
    c.vm.w16(c.st, a!(SEQ_DELAY), 0);
    c.vm.w8(c.st, a!(SEQ_PHASE), 0);
    c.vm.w8(c.st, a!(SEQ_PERIOD), 6);
    c.vm.w16(c.st, a!(SEQ_PTR), 0x3CC);
    frame_loop(c);
}

/// One orders frame (file `0x332AE`–`0x332D8`): vsync, sequencer,
/// input, menu, `0x701D`, tick, frame count, `0x3438` — exits on
/// `[0x9CDA] & 2` via `0x5EE6` + selection stash + `jmp 0x2CC5`.
fn frame_loop(c: &mut Ctx) {
    loop {
        vsync(c);
        seq::seq_step(c);
        c.host.svc(Call::Native(0xA369));
        menu::service(c);
        c.host.svc(Call::Native(0x701D));
        let _ = tick_step(c);
        bump(c);
        c.host.svc(Call::Native(0x3438));
        if c.vm.r16(c.st, a!(BUTTONS)) & 2 != 0 {
            break;
        }
    }
    selrec::sync_sel(c);
    leave(c, 0xA858);
}
