//! The uimode-1 fleet screen — `cs:0xCC62`/`0xCC65` (file
//! `0x35F12`–`0x3600B`), the `0xAEEE` list's `0xB` records.

use super::super::consts::{BUTTONS, SEQ_DELAY, SEQ_PERIOD, SEQ_PHASE, SEQ_PTR};
use super::super::vhost::Call;
use super::super::vops::{vsync, Ctx};
use super::super::{dialog, menu, seq, tick_step};
use super::{bump, install, leave};

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `cs:0xCC62` — fleet-screen entry: the sound block, the `0xAEEE`
/// list install, the screen draw, the `[0x91DE]`/`[0x91DF]`/`[0x91C9]`
/// clears, `uimode 1`, the banner calls and the sequencer arming —
/// then the frame loop. `win` is the `0xCC65` variant — the `[0x1274]`
/// window slot first (file `0x35F12`–`0x35F93`).
pub fn enter_fleet(c: &mut Ctx, win: bool) {
    if win {
        c.host.svc(Call::Slot(0x1274, 0));
    }
    dialog::snd_block(c);
    install(c, 0xAEEC, 0xAEEE, 0xB, 1);
    c.host.svc(Call::Slot(0x1260, 0));
    c.host.svc(Call::Slot(0x125C, 0));
    c.host.svc(Call::Native(0x89C3));
    c.host.svc(Call::Image(0x27));
    c.host.svc(Call::Present(1));
    c.vm.w8(c.st, 0x91DE, 0);
    c.vm.w8(c.st, 0x91DF, 0);
    c.vm.w8(c.st, 0x91C9, 0);
    for f in [0x6075, 0x60B6, 0x60D4] {
        c.host.svc(Call::Native(f));
    }
    c.vm.w16(c.st, a!(SEQ_DELAY), 0);
    c.vm.w8(c.st, a!(SEQ_PHASE), 0);
    c.vm.w16(c.st, a!(SEQ_PTR), 0x360);
    c.host.svc(Call::Native(0x5C49));
    c.host.svc(Call::Native(0x601A));
    frame_loop(c);
}

/// One fleet frame (file `0x35F94`–`0x3600B`): vsync, sequencer, the
/// shadowed second sequencer pass (the `0x919C`/`0x91A2`/`0x91D8`/
/// `0x91D9` context swap), `0x8361`, frame count, tick, input, menu —
/// exits on `[0x9CDA] & 2` via stash + `jmp 0x2CC5`.
fn frame_loop(c: &mut Ctx) {
    loop {
        vsync(c);
        seq::seq_step(c);
        alt_seq(c);
        c.host.svc(Call::Native(0x8361));
        bump(c);
        let _ = tick_step(c);
        c.host.svc(Call::Native(0xA369));
        menu::service(c);
        if c.vm.r16(c.st, a!(BUTTONS)) & 2 != 0 {
            break;
        }
    }
    leave(c, 0xAEEC);
}

/// The alternate-sequencer pass — exchange the live seq context with
/// its `0x919C`/`0x91A2`/`0x91D8`/`0x91D9` shadow, run `0x5C1E` under
/// it, swap back (file `0x35F9A`–`0x35FEC`).
fn alt_seq(c: &mut Ctx) {
    swap_ctx(c);
    seq::seq_step(c);
    swap_ctx(c);
}

/// Exchange the live seq context with its `0x919C`-block shadow.
fn swap_ctx(c: &mut Ctx) {
    swap16(c, a!(SEQ_PTR), 0x919C);
    swap16(c, a!(SEQ_DELAY), 0x91A2);
    swap8(c, a!(SEQ_PERIOD), 0x91D8);
    swap8(c, a!(SEQ_PHASE), 0x91D9);
}

/// Exchange two `ds:` words.
fn swap16(c: &mut Ctx, live: u16, shadow: u16) {
    let (a, b) = (c.vm.r16(c.st, live), c.vm.r16(c.st, shadow));
    c.vm.w16(c.st, live, b);
    c.vm.w16(c.st, shadow, a);
}

/// Exchange two `ds:` bytes.
fn swap8(c: &mut Ctx, live: u16, shadow: u16) {
    let (a, b) = (c.vm.r8(c.st, live), c.vm.r8(c.st, shadow));
    c.vm.w8(c.st, live, b);
    c.vm.w8(c.st, shadow, a);
}
