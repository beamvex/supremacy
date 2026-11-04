//! `cs:0x540C` — `build_fly`, the launch/destination pick (records
//! `0xC15C`, file `0x3540C`–`0x3551E`): the `0x56BF` heading erase,
//! the SHIP/FROM prints, the `[0x91A8] = 0x20` modal planet pick
//! (`0x6FB8` grid, `0x701D` clicks under the armed `[0x91E3]`), and
//! the `0x2CC5` abandon on right-click. The commit tail (`0x35511`+)
//! lives in [`bcommit`].

use super::super::consts::{ARMED, BUTTONS, HOT_COUNT, HOT_SEL, LAUNCH_F, SEL_MACH, SEL_REC};
use super::super::mach::M_FLAGS;
use super::super::rec::F_NAME;
use super::super::vhost::Call;
use super::super::vops::{vsync, Ctx};
use super::super::{dialog, menu, seq, shell, text, tick_step};
use super::{bcommit, bfield, bpaint, machd, ordlist};

/// `u16` casts for the `ds:`/record constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `[0x9190]` — the `[0x917C]` stash the pick saves and the
/// cancel/refuse tails restore.
pub(super) const SAVE_REC: u16 = 0x9190;
/// `[0xA480]` — the build list's selection stash (the `leave` tail).
const SEL_STASH: u16 = 0xA480;
/// List count while the pick is armed — the 32 stub grid cells.
const PICK_N: u16 = 0x20;
/// The list's own `0x2E` count — restored after each `0xA10F` bracket.
const LIST_N: u16 = 0x2E;

/// `cs:0x540C` — the guards, then the SHIP/FROM prints, the pick
/// modal and the commit/refuse tails.
pub(super) fn build_fly(c: &mut Ctx) {
    bfield::erase_head(c);
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    if m == 0 {
        return bfield::refuse(c, 0x71E5, 0x1F, 0x2000);
    }
    if guard(c, m) {
        return;
    }
    head(c, m);
    pick(c, m);
}

/// `0x3542A`/`0x3543F` — in-transit → `0x71F8`, not-in-orbit →
/// `0x71CF`.
fn guard(c: &mut Ctx, m: u16) -> bool {
    let fl = c.vm.r8(c.st, m + a!(M_FLAGS));
    if fl & 8 != 0 {
        bfield::refuse(c, 0x71F8, 0x1A, 0x2000);
        return true;
    }
    if fl & 4 == 0 {
        bfield::refuse(c, 0x71CF, 0x1D, 0x2000);
        return true;
    }
    false
}

/// `0x35454`–`0x3549E` — the SHIP name at `(0x34,0x8F)`, the `al=3`
/// blip, the `0x5085` host's name at `(0x34,0x99)` (FROM), the
/// `0x52FE`+`0x726A` prompt pair and the `[0x9190]` stash.
fn head(c: &mut Ctx, m: u16) {
    text::print_str(c, m, 0x34, 0x8F);
    dialog::snd_block(c);
    let h = machd::mach_host(c, m);
    text::print_str(c, h.wrapping_add(a!(F_NAME)), 0x34, 0x99);
    bfield::prompt(c);
    text::print_str(c, 0x726A, 0x1A, 0x5F);
    let rec = c.vm.r16(c.st, a!(SEL_REC));
    c.vm.w16(c.st, SAVE_REC, rec);
}

/// `0x354A1`–`0x35510` — `[0x917C] = 0`, the `0x6FB8` planet-list
/// repaint, the `[0x91E3]`-armed release wait, then the modal loop: a
/// grid pick (`0x701D` → nonzero `[0x917C]`) commits, right-click
/// abandons.
fn pick(c: &mut Ctx, m: u16) {
    c.vm.w16(c.st, a!(SEL_REC), 0);
    bpaint::planet_mode(c);
    c.vm.w8(c.st, a!(ARMED), 0);
    c.vm.w8(c.st, a!(LAUNCH_F), 0xFF);
    c.vm.w16(c.st, a!(HOT_COUNT), PICK_N);
    release(c);
    c.vm.w16(c.st, a!(HOT_COUNT), LIST_N);
    loop {
        step(c);
        if c.vm.r16(c.st, a!(SEL_REC)) != 0 {
            return bcommit::commit(c, m);
        }
        if c.vm.r16(c.st, a!(BUTTONS)) & 2 != 0 {
            return cancel(c);
        }
    }
}

/// `0x354CE` — one modal frame: vsync, sequencer, tick, the input
/// poll, the `0x20`-counted `0xA10F` bracket and the `0x701D` pick.
fn step(c: &mut Ctx) {
    vsync(c);
    seq::seq_step(c);
    let _ = tick_step(c);
    c.host.svc(Call::Native(0xA369));
    c.vm.w16(c.st, a!(HOT_COUNT), PICK_N);
    menu::service(c);
    c.vm.w16(c.st, a!(HOT_COUNT), LIST_N);
    ordlist::list_click(c);
}

/// `0x354BA`/`0x35511` — `{0xA369, 0xA10F}` while the left button is
/// held.
pub(super) fn release(c: &mut Ctx) {
    loop {
        c.host.svc(Call::Native(0xA369));
        menu::service(c);
        if c.vm.r16(c.st, a!(BUTTONS)) & 1 == 0 {
            return;
        }
    }
}

/// `0x354FC` — the cancel: `[0x91E3]` down, `[0x917C]` restored, the
/// `pop ax` abandon's `[0xA480]` stash, `jmp 0x2CC5`.
fn cancel(c: &mut Ctx) {
    c.vm.w8(c.st, a!(LAUNCH_F), 0);
    let rec = c.vm.r16(c.st, SAVE_REC);
    c.vm.w16(c.st, a!(SEL_REC), rec);
    let sel = c.vm.r16(c.st, a!(HOT_SEL));
    c.vm.w16(c.st, SEL_STASH, sel);
    shell::shell_enter(c);
}
