//! The shell "spy" action — `cs:0x8149` (records `0xEE99`, file
//! `0x38149`–`0x382BD`): the `[0x9184]` kind gates (`0xA` → `0x760E`,
//! `4` → `0x762C`, `1` → `0x7647` through the `0x82A7` tail), else the
//! `0xAFCC` five-record modal picker that feeds `[0x91DC]` via the
//! `0x80D2`–`0x80EA` mode actions; a pick stages the record's stocks
//! into the `0x83A4..` report cells, runs the per-mode `0x82BE`
//! charges (modes 1/4 → `0x3E8`, 3/4 → `0x898`, 2/4 → `0x5F0` —
//! cumulative on mode `4`) and enqueues each report string.

use super::super::cells::{MENU_COUNT, MENU_LIST, MENU_SEL};
use super::super::consts::{HOT_COUNT, HOT_LIST, HOT_SEL, REC_CUR};
use super::super::vhost::Call;
use super::super::vops::{faction_rec, kind_of, vsync, Ctx};
use super::super::{dialog, hover, menu, selrec, seq, text};
use super::{bump, snd};

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `[0x91DC]` — the picker mode cell the `0x80D2`–`0x80EA` records
/// write (`5` = cancel).
const MODE: u16 = 0x91DC;
/// `[0x81FE]` — the mission-report flag the tail arms.
const REPORT_F: u16 = 0x81FE;
/// The picker list's saved selection cell.
const PICK_SEL: u16 = 0xAFCA;
/// The five-record picker list (`ds:0xAFCC`).
const PICK_LIST: u16 = 0xAFCC;
/// Picker record count.
const PICK_N: u16 = 5;

/// `cs:0x8149` — the spy button (file `0x38149`–`0x382B5`).
pub fn spy_menu(c: &mut Ctx) {
    snd(c, 3, 0xA);
    dialog::pump(c);
    let rec = c.vm.r16(c.st, a!(REC_CUR));
    match kind_of(c, rec) {
        0xA => gate(c, 0x760E),
        4 => gate(c, 0x762C),
        1 => gate(c, 0x7647),
        _ => pick(c, rec),
    }
}

/// `0x82B6`/`0x817B`/`0x818A` — enqueue the kind's message then the
/// `0x82A7` tail.
fn gate(c: &mut Ctx, s: u16) {
    text::enqueue(c, s);
    tail(c);
}

/// `0x8193`–`0x382A6` — the `0x768C` prompt, the `0xAFCC` modal
/// picker loop, the shell-list restore, the `5` cancel, the stock
/// staging + report enqueues, the per-mode charges.
fn pick(c: &mut Ctx, rec: u16) {
    text::enqueue(c, 0x768C);
    dialog::pump(c);
    install(c, PICK_SEL, PICK_LIST, PICK_N);
    c.vm.w8(c.st, MODE, 0);
    while c.vm.r8(c.st, MODE) == 0 {
        iter(c);
    }
    install(c, a!(MENU_SEL), a!(MENU_LIST), MENU_COUNT);
    let mode = c.vm.r8(c.st, MODE);
    if mode == 5 {
        text::enqueue(c, 0x7123); // the `jmp 0x6B06` cancel — no tail
        return;
    }
    stage(c, rec);
    for s in [0x776B, rec + 0xE, 0x7667] {
        text::enqueue(c, s);
    }
    missions(c, mode);
}

/// `0x8238`–`0x382A6` — the per-mode `0x82BE` charges; each success
/// enqueues its report, a failure leaves through the shared tail.
fn missions(c: &mut Ctx, mode: u8) {
    if mode == 1 || mode == 4 {
        if !charge(c, 0x3E8) {
            return tail(c);
        }
        for s in [0x8398, 0x83A6, 0x83B4, 0x83C2] {
            text::enqueue(c, s);
        }
    }
    if mode == 3 || mode == 4 {
        if !charge(c, 0x898) {
            return tail(c);
        }
        text::enqueue(c, 0x83E0);
    }
    if mode == 2 || mode == 4 {
        if !charge(c, 0x5F0) {
            return tail(c);
        }
        text::enqueue(c, 0x83D0);
    }
    tail(c);
}

/// One `0x81B3` picker iteration — vsync, input, menu, oval,
/// `0x83AA` re-mark, sequencer, rng churn, frame bump.
fn iter(c: &mut Ctx) {
    vsync(c);
    c.host.svc(Call::Native(0xA369));
    menu::service(c);
    hover::hover_check(c);
    selrec::mark_sel(c);
    seq::seq_step(c);
    let _ = c.rng.next16();
    bump(c);
}

/// The `[0x9CC8] = [sel]`/`[0x9194]`/`[0x91A8]` list install (file
/// `0x3819C`/`0x381D5`).
fn install(c: &mut Ctx, sel: u16, list: u16, n: u16) {
    let s = c.vm.r16(c.st, sel);
    c.vm.w16(c.st, a!(HOT_SEL), s);
    c.vm.w16(c.st, a!(HOT_LIST), list);
    c.vm.w16(c.st, a!(HOT_COUNT), n);
}

/// `0x81F4` — stage the record's stock words into the report number
/// cells `0x83A4`/`0x83B2`/`0x83C0`/`0x83CE`/`0x83DE`/`0x83F4`.
fn stage(c: &mut Ctx, rec: u16) {
    for (o, cell) in [
        (0x2Cu16, 0x83A4u16),
        (0x30, 0x83B2),
        (0x32, 0x83C0),
        (0x34, 0x83CE),
        (0x2A, 0x83DE),
        (0x26, 0x83F4),
    ] {
        let v = c.vm.r16(c.st, rec + o);
        c.vm.w16(c.st, cell, v);
    }
}

/// `cs:0x82BE` — charge the faction `ax` credits: insufficient funds
/// enqueue `0x7669` and answer `false`; else the `sub`/`sbb` pair
/// writes `+0x36/+0x38` back (file `0x382BE`–`0x382E4`).
fn charge(c: &mut Ctx, ax: u16) -> bool {
    let f = faction_rec(c);
    let funds = u32::from(c.vm.r16(c.st, f + 0x38)) << 16 | u32::from(c.vm.r16(c.st, f + 0x36));
    if funds < u32::from(ax) {
        text::enqueue(c, 0x7669);
        return false;
    }
    let v = funds.wrapping_sub(u32::from(ax));
    c.vm.w16(c.st, f + 0x36, u16::try_from(v & 0xFFFF).unwrap_or(0));
    c.vm.w16(c.st, f + 0x38, u16::try_from(v >> 16).unwrap_or(0));
    true
}

/// `0x82A7` — the shared tail: `0x712C` queued, `[0x81FE] = 0xFF`,
/// `0x2FE7` pump.
fn tail(c: &mut Ctx) {
    text::enqueue(c, 0x712C);
    c.vm.w8(c.st, REPORT_F, 0xFF);
    dialog::pump(c);
}
