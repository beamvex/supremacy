//! The uimode-6 planet-status screen — `cs:0xC5B7`/`0xC5CA` (file
//! `0x35867`–`0x35959`), guarded entries over the shared `0x589B`
//! body and its `0x9D60` 0x13-record list.

use super::super::consts::{BUTTONS, REC_CUR, SEL_MACH, SEL_REC, SEQ_PTR};
use super::super::rec::F_OWNER;
use super::super::vhost::Call;
use super::super::vops::{kind_of, vsync, Ctx};
use super::super::{menu, selrec, shell, text, tick_step};
use super::{bump, install, leave};

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `ds:` slot of the status-screen animation flag (`[0x91DD]`).
const ANIM: u16 = 0x91DD;
/// The `0x7402` "not colonised" refusal message.
const S_REFUSE: u16 = 0x7402;
/// The `0x6AD6` "no population" refusal message.
const S_EMPTY: u16 = 0x6AD6;

/// `cs:0xC5B7` — `0x5EE6` resync then enter status on `[0x917C]`'s
/// kind: `!= 0xA` refuses with `0x7402` + `jmp 0x2CC5` (file
/// `0x35867`–`0x35879`).
pub fn enter_status(c: &mut Ctx) {
    selrec::sync_sel(c);
    if kind_of(c, c.vm.r16(c.st, a!(SEL_REC))) != 0xA {
        text::enqueue(c, S_REFUSE);
        shell::shell_enter(c);
        return;
    }
    body(c);
}

/// `cs:0xC5CA` — the `[0x1274]` window variant on `[0x9184]` with the
/// extra `+0x24` owner guard (`0x6AD6` when unowned); both refusals
/// are bare enqueues (file `0x3587A`–`0x3589A`).
pub fn enter_status_win(c: &mut Ctx) {
    let cur = c.vm.r16(c.st, a!(REC_CUR));
    if c.vm.r8(c.st, cur + u16::try_from(F_OWNER).unwrap_or(0)) == 0 {
        text::enqueue(c, S_EMPTY);
        return;
    }
    if kind_of(c, cur) != 0xA {
        text::enqueue(c, S_REFUSE);
        return;
    }
    c.host.svc(Call::Slot(0x1274, 0));
    body(c);
}

/// The `0x589B` shared body — sound block, cell clears, the `0x9D60`
/// list install, the screen draw, the name/stat prints, `uimode 6`,
/// then the frame loop (file `0x3589B`–`0x35934`).
fn body(c: &mut Ctx) {
    super::super::dialog::snd_block(c);
    c.vm.w16(c.st, a!(SEQ_PTR), 0);
    c.vm.w16(c.st, a!(SEL_MACH), 0);
    c.vm.w8(c.st, ANIM, 0);
    c.host.svc(Call::Slot(0x1260, 0));
    c.host.svc(Call::Slot(0x125C, 0));
    c.host.svc(Call::Native(0x89C3));
    c.host.svc(Call::Image(2));
    c.host.svc(Call::Present(6));
    install(c, 0x9D5E, 0x9D60, 0x13, 6);
    c.vm.w16(c.st, a!(SEL_REC), c.vm.r16(c.st, a!(REC_CUR)));
    let name = c.vm.r16(c.st, a!(REC_CUR)) + 0xE;
    text::print_str(c, name, 3, 0x76);
    c.host.svc(Call::Native(0x595C));
    text::print_str(c, 0x7BB8, 4, 0xB4);
    c.host.svc(Call::Native(0x59EB));
    c.host.svc(Call::Native(0xEA12));
    frame_loop(c);
}

/// One status frame (file `0x35935`–`0x35958`): vsync, `0x833B`,
/// `0xEA2E`, tick, input, menu, frame count — exits on
/// `[0x9CDA] & 2` via stash + `jmp 0x2CC5`.
fn frame_loop(c: &mut Ctx) {
    loop {
        vsync(c);
        c.host.svc(Call::Native(0x833B));
        c.host.svc(Call::Native(0xEA2E));
        let _ = tick_step(c);
        c.host.svc(Call::Native(0xA369));
        menu::service(c);
        bump(c);
        if c.vm.r16(c.st, a!(BUTTONS)) & 2 != 0 {
            break;
        }
    }
    leave(c, 0x9D5E);
}
