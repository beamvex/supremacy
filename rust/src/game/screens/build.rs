//! The uimode-4 machine/build screen — `cs:0xB8F7`/`0xB8FA` (file
//! `0x34BA7`–`0x34C88`), the `0xA482` list's `0x2E` records.

use super::super::consts::{
    BUTTONS, DIRTY0, DIRTY1, REC_CUR, SEL_MACH, SEL_REC, SEQ_DELAY, SEQ_PERIOD, SEQ_PHASE, SEQ_PTR,
};
use super::super::vhost::Call;
use super::super::vops::{vsync, Ctx};
use super::super::{dialog, menu, selrec, seq, status, tick_step};
use super::{bump, install, leave};

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `cs:0xB8F7` — build-screen entry: `[0x91EA]` clear, the sound
/// block, `uimode 4`, the `0xA482` list install, the screen draw, the
/// machine/selection cells and the sequencer arming — then the frame
/// loop. `win` is the `0xB8FA` variant — the `[0x1274]` window slot
/// first (file `0x34BA7`–`0x34C51`).
pub fn enter_build(c: &mut Ctx, win: bool) {
    if win {
        c.host.svc(Call::Slot(0x1274, 0));
    }
    c.vm.w8(c.st, 0x91EA, 0);
    dialog::snd_block(c);
    install(c, 0xA480, 0xA482, 0x2E, 4);
    c.host.svc(Call::Slot(0x1260, 0));
    c.host.svc(Call::Slot(0x125C, 0));
    c.host.svc(Call::Native(0x89C3));
    c.host.svc(Call::Image(0x2A));
    c.host.svc(Call::Present(4));
    c.host.svc(Call::Native(0x5994));
    c.host.svc(Call::Slot(0x128C, 0x33));
    cells(c);
    c.host.svc(Call::Native(0x4C8B));
    c.host.svc(Call::Native(0x6FBF));
    status::tick_day(c);
    c.vm.w16(c.st, a!(SEQ_PTR), 0x424);
    c.vm.w16(c.st, a!(SEQ_DELAY), 0);
    c.vm.w8(c.st, a!(SEQ_PHASE), 0);
    c.vm.w8(c.st, a!(SEQ_PERIOD), 5);
    frame_loop(c);
}

/// The machine/selection block (file `0x34C10`–`0x34C33`): clear the
/// `[0x9180]`/`[0x9182]` machine pointers, mirror `[0x9184]` into
/// `[0x917C]`, arm the dirty words, clear `[0x91E3]`.
fn cells(c: &mut Ctx) {
    c.vm.w16(c.st, a!(SEL_MACH), 0);
    c.vm.w16(c.st, 0x9182, 0);
    c.vm.w16(c.st, a!(SEL_REC), c.vm.r16(c.st, a!(REC_CUR)));
    c.vm.w16(c.st, a!(DIRTY0), 0x200);
    c.vm.w16(c.st, a!(DIRTY1), 0);
    c.vm.w8(c.st, 0x91E3, 0);
}

/// One build frame (file `0x34C52`–`0x34C88`): `0x4EB6`, the
/// `[0x91E2]`-keyed `0x7119`/`0x701D` call, vsync, sequencer, tick,
/// input, menu — exits via `0x5EE6` + stash + `jmp 0x2CC5`.
fn frame_loop(c: &mut Ctx) {
    loop {
        c.host.svc(Call::Native(0x4EB6));
        let f = if c.vm.r8(c.st, 0x91E2) == 0 {
            0x701D
        } else {
            0x7119
        };
        c.host.svc(Call::Native(f));
        vsync(c);
        seq::seq_step(c);
        let _ = tick_step(c);
        c.host.svc(Call::Native(0xA369));
        menu::service(c);
        bump(c);
        if c.vm.r16(c.st, a!(BUTTONS)) & 2 != 0 {
            break;
        }
    }
    selrec::sync_sel(c);
    leave(c, 0xA480);
}
