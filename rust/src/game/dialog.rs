//! Dialog plumbing — the `cs:0x2E21` save/load modal, the
//! `cs:0x2F9B` two-option confirm, the `cs:0x2FE7` ticker pump, and
//! the shared list installs/prints (file `0x32E21`–`0x33039`).
//!
//! Both dialogs reuse the shell's frame phases (vsync, input stub,
//! menu service, oval, sequencer) and swap the hotspot list for their
//! own `0xB05A`/`0xB098` lists; exits come from menu actions `jmp`ing
//! out or, for the main dialog, `[0x9CDA] & 2` (right button).

use super::cells::{
    BLINK_B, BLINK_HOLD, BLINK_PH, BLINK_T, CNF_LIST, CNF_SEL, DLG_FLAG, DLG_LIST, DLG_SEL,
    MENU_COUNT, MENU_LIST, MENU_SEL, S_CNF_B, S_CNF_T, TICK_LEFT,
};
use super::consts::{BUTTONS, FRAME, HOT_COUNT, HOT_LIST, HOT_SEL, SND_ON, UIMODE};
use super::text;
use super::vhost::Call;
use super::vops::Ctx;
use super::{blink, hover, menu, selrec, seq, ticker};

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// The `0x2CC5`-pattern sound block — `Chan(0x8A5B)` / `al=3
/// 0x8A3B` / `cl=0xA` driver call, suppressed on `[0x91D3]==0xFF`.
pub fn snd_block(c: &mut Ctx) {
    if c.vm.r8(c.st, a!(SND_ON)) == 0xFF {
        return;
    }
    c.host.svc(Call::Chan(0x8A5B, 0));
    c.host.svc(Call::Chan(0x8A3B, 3));
    c.host.svc(Call::Sound(0xA));
}

/// `cs:0x2F24` — print a `ds:` script-string at `(0x2A,0xB1)` via
/// `0x6709` (file `0x32F24`–`0x32F2C`).
pub fn dlg_text(c: &mut Ctx, s: u16) {
    text::print_str(c, s, 0x2A, 0xB1);
}

/// `cs:0x2FD7` — the dialog frame box, `jmp [0x1262]` with
/// `ax=0x14,bx=0x5A,cx=8,dx=0x66` (file `0x32FD7`).
fn frame_rect(c: &mut Ctx) {
    c.host.svc(Call::Rect(0x14, 0x5A, 8, 0x66));
}

/// `cs:0x2EEE`/`0x2F0F` — install the 3-record dialog list
/// `ds:0xB05C`, selection from `ds:0xB05A` (file `0x32EEE`).
pub fn dlg_list(c: &mut Ctx) {
    install(c, a!(DLG_SEL), a!(DLG_LIST), 3);
}

/// Restore the shell's main hotspot list (`0xA0AA`/`0xA0AC`/`0x31`,
/// file `0x32E81`–`0x32E93`).
pub fn main_list(c: &mut Ctx) {
    install(c, a!(MENU_SEL), a!(MENU_LIST), MENU_COUNT);
}

/// Shared list install: `[0x9CC8]=sel`, `[0x9194]=list`,
/// `[0x91A8]=n`.
fn install(c: &mut Ctx, sel: u16, list: u16, n: u16) {
    let s = c.vm.r16(c.st, sel);
    c.vm.w16(c.st, a!(HOT_SEL), s);
    c.vm.w16(c.st, a!(HOT_LIST), list);
    c.vm.w16(c.st, a!(HOT_COUNT), n);
}

/// `cs:0x2FE7` — pump the ticker to completion: `[0x91F2]=0xFF` hold,
/// loop `{vsync, blink, ≤3 ticker steps, seq, oval, mark_sel, frame++}`
/// while `[0x91C2]` is nonzero; then clear the hold and, for
/// `uimode == 0`, reset the blink phase and reprint `0x77E3` (the
/// `jmp 0x2D86` tail) (file `0x32FE7`–`0x33039`).
pub fn pump(c: &mut Ctx) {
    c.vm.w8(c.st, a!(BLINK_HOLD), 0xFF);
    while c.vm.r16(c.st, a!(TICK_LEFT)) != 0 {
        pump_iter(c);
    }
    c.vm.w8(c.st, a!(BLINK_HOLD), 0);
    pump_tail(c);
}

/// One pump iteration (file `0x32FEC`–`0x33024`).
fn pump_iter(c: &mut Ctx) {
    c.host.svc(Call::Ui(0x2CA9));
    blink::blink_step(c);
    for _ in 0..3 {
        if c.vm.r8(c.st, a!(BLINK_T)) == 0 {
            ticker::ticker_step(c);
        }
    }
    seq::seq_step(c);
    hover::hover_check(c);
    selrec::mark_sel(c);
    let f = c.vm.r8(c.st, a!(FRAME)).wrapping_add(1);
    c.vm.w8(c.st, a!(FRAME), f);
}

/// The `0x3030` tail — `uimode==0` falls into `0x2D86` (phase reset +
/// `0x77E3` reprint); other modes just clear the phase and `ret`.
fn pump_tail(c: &mut Ctx) {
    c.vm.w8(c.st, a!(BLINK_PH), 0);
    if c.vm.r8(c.st, a!(UIMODE)) == 0 {
        text::print_str(c, a!(BLINK_B), 0x4D, 0x67);
    }
}

/// `cs:0x2E21` — open the save/load dialog: sound, pump, `0x1274`
/// slot, frame, dialog list, image `0x12B` (file `0x32E21`–`0x32E5F`).
pub fn dlg_enter(c: &mut Ctx) {
    snd_block(c);
    pump(c);
    c.host.svc(Call::Slot(0x1274, 0));
    frame_rect(c);
    dlg_list(c);
    c.host.svc(Call::Image(0x12B));
}

/// `cs:0x2E60` — one modal iteration; `true` when `[0x9CDA] & 2`
/// (file `0x32E60`–`0x32E79`).
pub fn dlg_step(c: &mut Ctx) -> bool {
    c.host.svc(Call::Ui(0x2CA9));
    c.host.svc(Call::Native(0xA369));
    menu::service(c);
    hover::hover_check(c);
    seq::seq_step(c);
    c.host.svc(Call::Native(0x1E2E));
    c.vm.r16(c.st, a!(BUTTONS)) & 2 != 0
}

/// The `0x2E7A` close path — frame, `0x1272` slot, main list.
pub fn dlg_close(c: &mut Ctx) {
    frame_rect(c);
    c.host.svc(Call::Slot(0x1272, 0));
    main_list(c);
}

/// `cs:0x2F9B` — the two-option confirm: title + body prints, the
/// `0xB098` list, then a modal wait on `[0x91DA]` written by the
/// `0x2F01`/`0x2F08`/`0x2F95` actions; closes back into `0x2EEE`
/// (file `0x32F9B`–`0x32FD6`).
pub fn confirm(c: &mut Ctx) {
    text::print_str(c, a!(S_CNF_T), 0x2A, 0xB1);
    dlg_text(c, a!(S_CNF_B));
    install(c, a!(CNF_SEL), a!(CNF_LIST), 2);
    c.vm.w8(c.st, a!(DLG_FLAG), 0);
    while c.vm.r8(c.st, a!(DLG_FLAG)) == 0 {
        cnf_iter(c);
    }
    text::print_str(c, a!(S_CNF_T), 0x2A, 0xB1);
    dlg_list(c);
}

/// One confirm-loop iteration — vsync, input, menu, oval, seq (file
/// `0x32FBB`–`0x32FCF`).
fn cnf_iter(c: &mut Ctx) {
    c.host.svc(Call::Ui(0x2CA9));
    c.host.svc(Call::Native(0xA369));
    menu::service(c);
    hover::hover_check(c);
    seq::seq_step(c);
}
