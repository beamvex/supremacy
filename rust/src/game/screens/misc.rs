//! The non-screen menu actions — the minimap click (`cs:0x23C2`,
//! records `0x9112`), the status ticker report (`cs:0x2D97`, records
//! `0x9AE7`), the restart entry (`cs:0x8427`, records `0xF177`/
//! `0xF180`), the sound toggle (`cs:0x854A`, records `0xF29A`), and
//! the inert grid-cell stub (`cs:0x8544`, records `0xF294`).

use super::super::consts::{
    ARMED, BUTTONS, CUR_X, CUR_Y, KEY_CODE, KEY_PEND, REC_BASE, REC_COUNT, REC_CUR, REC_STRIDE,
    REC_TOTAL, SND_ON,
};
use super::super::vhost::Call;
use super::super::vops::{vsync, Ctx};
use super::super::{dialog, dlg_io, init, text};
use super::snd;

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// Ticker-queue guard the report reads (`[0x91C2]`, file `0x32DB3`).
const QUEUE_POS: u16 = 0x91C2;
/// `bl >= 0x74` refuses the report (file `0x32DB7`).
const QUEUE_CAP: u8 = 0x74;

/// `cs:0x23C2` — the minimap action (file `0x323C2`–`0x32425`): the
/// `al=0x20`/`cl=0x17` chirp, then a release-wait and a press-wait
/// loop (`{0x2CA9, 0xA369, 0xA1CA}` polling `[0x9CDA] & 1`); a press
/// inside `[0x111..=0x132] × [0x13..=0x2B]` loads image `1`, an
/// outside press re-arms the wait.
pub fn map_click(c: &mut Ctx) {
    snd(c, 0x20, 0x17);
    poll(c, false);
    loop {
        poll(c, true);
        let x = c.vm.r16(c.st, a!(CUR_X));
        let y = c.vm.r16(c.st, a!(CUR_Y));
        if (0x111..=0x132).contains(&x) && (0x13..=0x2B).contains(&y) {
            break;
        }
    }
    c.host.svc(Call::Image(1));
}

/// One `{vsync, 0xA369, 0xA1CA}` poll pass, looping while the left
/// button is in the `!held` state (file `0x323DE`/`0x323EF`).
fn poll(c: &mut Ctx, held: bool) {
    loop {
        vsync(c);
        c.host.svc(Call::Native(0xA369));
        c.host.svc(Call::Native(0xA1CA));
        if (c.vm.r16(c.st, a!(BUTTONS)) & 1 != 0) == held {
            break;
        }
    }
}

/// `cs:0x2D97` — the status report (file `0x32D97`–`0x32E1F`): sound
/// block, the `[0x91C2] >= 0x74` refuse, the header/name/kind strings
/// around `[0x9184]`, the `0x37C7` kind counts into the `0x8364`/
/// `0x8378`/`0x838C` number cells, then the count strings.
pub fn report(c: &mut Ctx) {
    dialog::snd_block(c);
    if c.vm.r8(c.st, QUEUE_POS) >= QUEUE_CAP {
        return;
    }
    let cur = c.vm.r16(c.st, a!(REC_CUR));
    let kind = kind_str(c, cur);
    for s in [0x7161, cur + 0xE, 0x7180, cur + 0x19, 0x718A, 0x718E, kind] {
        text::enqueue(c, s);
    }
    counts(c);
}

/// The `0x37C7` count + number-cell writeback (file `0x32DF2`–
/// `0x32E07`): `bp` = kind-`0xA` records → `[0x8364]`, `ax` = kind-`7`
/// → `[0x8378]`, `[0x91B6] − (ax+bp)` → `[0x838C]` — then the count
/// strings `0x8351`/`0x8366`/`0x837A`/`0x712C`.
fn counts(c: &mut Ctx) {
    let base = c.vm.r16(c.st, a!(REC_BASE));
    let n = c.vm.r16(c.st, a!(REC_COUNT));
    let stride = a!(REC_STRIDE);
    let (mut kinds7, mut kinds_a) = (0u16, 0u16);
    for i in 0..=n {
        match c
            .vm
            .r16(c.st, base.wrapping_add(i.wrapping_mul(stride)) + 0xC)
        {
            0xA => kinds_a = kinds_a.wrapping_add(1),
            7 => kinds7 = kinds7.wrapping_add(1),
            _ => {}
        }
    }
    c.vm.w16(c.st, 0x8364, kinds_a);
    c.vm.w16(c.st, 0x8378, kinds7);
    let free =
        c.vm.r16(c.st, a!(REC_TOTAL))
            .wrapping_sub(kinds7.wrapping_add(kinds_a));
    c.vm.w16(c.st, 0x838C, free);
    for s in [0x8351, 0x8366, 0x837A, 0x712C] {
        text::enqueue(c, s);
    }
}

/// `cs:0x303A` — record → kind string (file `0x3303A`–`0x3305A`):
/// kind `4` → `0x71AE`, `1` → `0x71B9`, `7` → `0x71A3`, else `0x7198`.
fn kind_str(c: &mut Ctx, rec: u16) -> u16 {
    match c.vm.r16(c.st, rec + 0xC) {
        4 => 0x71AE,
        1 => 0x71B9,
        7 => 0x71A3,
        _ => 0x7198,
    }
}

/// `cs:0x8427`/`0x8430` — the restart action (file `0x38427`–
/// `0x38430`): the `0x2F91` stage snapshot (the `pop si` ×3 abandons
/// the dialog frames the action `jmp`s out of), then the `jmp 0x30A0`
/// re-entry — record init and the new-game tail. The port's
/// [`init::act_newgame`] replays that tail; its `stage_load` head is
/// an identity copy over the just-saved snapshot.
pub fn restart(c: &mut Ctx) {
    dlg_io::act_stage(c);
    init::act_newgame(c);
}

/// `cs:0x854A` — the sound toggle (file `0x3854A`–`0x38576`): flip
/// `[0x91D3]`; muting enqueues `0x7C94`, un-muting plays the confirm
/// chirp and enqueues `0x7CA9`.
pub fn snd_toggle(c: &mut Ctx) {
    let on = c.vm.r8(c.st, a!(SND_ON)) ^ 0xFF;
    c.vm.w8(c.st, a!(SND_ON), on);
    if on == 0 {
        dialog::snd_block(c);
        text::enqueue(c, 0x7CA9);
        return;
    }
    text::enqueue(c, 0x7C94);
}

/// `cs:0x8544` — the inert grid-cell action (file `0x38544`): clears
/// the `[0x91CA]` press latch and returns.
pub fn stub(c: &mut Ctx) {
    c.vm.w8(c.st, a!(ARMED), 0);
}

/// `cs:0x8539` — the grid-cell key-synth (file `0x38539`–`0x38543`):
/// arms the pending-keystroke pair — `[0x9CCC] = 0x1C` (enter),
/// `[0x9CCB] = 1` — and returns; the `0xF289` records tile the
/// planet-grid list.
pub fn cell_pick(c: &mut Ctx) {
    c.vm.w8(c.st, a!(KEY_CODE), 0x1C);
    c.vm.w8(c.st, a!(KEY_PEND), 1);
}

/// `cs:0x80D2`–`0x80EA` — the five mode-cell setters: each record
/// carries the routine that writes its own index `1..=5` into
/// `[0x91DC]` and returns (file `0x380D2`–`0x380EF`).
pub fn mode_pick(c: &mut Ctx, n: u8) {
    c.vm.w8(c.st, 0x91DC, n);
}
