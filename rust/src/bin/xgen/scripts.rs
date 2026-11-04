//! Event-VM scripts (`ds:0x51FB+`, table `ds:0x5183`, op table
//! `ds:0x50E3`) → one native `evNN::run` per slot plus a `script::run`
//! that reads the slot's `si` and dispatches on the script address —
//! the `cs:0x773D`/`0x7758` pair compiled.

use super::blob::{b16, Blobs};
use super::out::Out;

const JT: u16 = 0x50E3;
const TAB: u16 = 0x5183;
const SLOTS: u16 = 30;
const DELTA: u16 = 0x6D50;

/// `ds:0x50E3` op-table word → `cs:` handler label.
fn handler(b: &Blobs, w: u16) -> u16 {
    let ofs = b16(&b.ds, JT.wrapping_add(w));
    if ofs >= 0x8000 {
        ofs.wrapping_sub(DELTA)
    } else {
        ofs
    }
}

/// Operand letters per handler: `s` = 4-byte slot, `i` = `u16` imm,
/// `c` = `u16` `cs:` operand, `t` = terminator (`ret`, no call).
fn shape(t: u16) -> &'static str {
    match t {
        0x7769 | 0x78C7 | 0x7774 | 0x7782 | 0x778C | 0x79AA => "s",
        0x7796 | 0x77A5 | 0x7811 | 0x7825 | 0x7839 => "si",
        0x77B4 | 0x7853 | 0x7866 | 0x7879 => "ss",
        0x7893 | 0x78A2 | 0x78B1 | 0x78D5 | 0x78E4 => "is",
        0x78F3 => "iis",
        0x7908 => "c",
        0x7768 | 0x7AF0 | 0x7773 => "t",
        0x77C3 | 0x77E1 | 0x77F0 | 0x790A | 0x7980 | 0x798E | 0x799C | 0x79BB | 0x79C3 | 0x79C4
        | 0x7A11 | 0x7A45 | 0x7ADD | 0x7AF1 | 0x7BAB | 0x7BE8 | 0x7BF4 | 0x7C05 | 0x7C16
        | 0x7C30 => "n",
        _ => "",
    }
}

/// The `si += n` an operand letter consumes.
fn take(ch: char, b: &Blobs, si: &mut u16, out: &mut Vec<u16>) {
    let n = if ch == 's' { 4 } else { 2 };
    let v = b16(&b.ds, *si);
    *si = si.wrapping_add(n);
    out.push(v);
}

/// The call expression for one op; `None` for the ret terminators.
fn call(t: u16, o: &[u16]) -> Option<String> {
    let f = |s: String| Some(s);
    let (p, i, j) = (
        o.first().copied().unwrap_or(0),
        o.get(1).copied().unwrap_or(0),
        o.get(2).copied().unwrap_or(0),
    );
    match t {
        0x7769 => f(format!("vimpl::wait_b(c, {p:#06X}, false)")),
        0x78C7 => f(format!("vimpl::wait_b(c, {p:#06X}, true)")),
        0x7774 => f(format!("vimpl::print(c, {p:#06X})")),
        0x7782 => f(format!("vimpl::set_flag(c, {p:#06X}, 0xFF)")),
        0x778C => f(format!("vimpl::set_flag(c, {p:#06X}, 0)")),
        0x7796 => f(format!("vimpl::wait_eq_b(c, {p:#06X}, {:#04X})", i & 0xFF)),
        0x77A5 | 0x77B4 => f(format!("vimpl::wait_eq_w(c, {p:#06X}, {i:#06X})")),
        0x77C3 => f("vimpl::print_planet(c)".into()),
        0x77E1 => f("vimpl::print_sel(c)".into()),
        0x7BE8 => f("vimpl::print_mach(c)".into()),
        0x77F0 => f("vimpl::rand_planet(c)".into()),
        0x7811 => f(format!("vimpl::ld_ind(c, {p:#06X}, {i:#06X}, 0x81F7, 1)")),
        0x7825 => f(format!("vimpl::ld_ind(c, {p:#06X}, {i:#06X}, 0x81F8, 2)")),
        0x7839 => f(format!("vimpl::ld_ind(c, {p:#06X}, {i:#06X}, 0x81FA, 4)")),
        0x7853 | 0x7866 | 0x7879 => {
            let n = match t {
                0x7853 => 1,
                0x7866 => 2,
                _ => 4,
            };
            f(format!("vimpl::cp(c, {p:#06X}, {i:#06X}, {n})"))
        }
        0x7893 | 0x78A2 | 0x78B1 => {
            let n = match t {
                0x7893 => 1,
                0x78A2 => 2,
                _ => 4,
            };
            f(format!("vimpl::add_imm(c, {i:#06X}, {p:#06X}, {n})"))
        }
        0x78D5 | 0x78E4 => {
            let n = if t == 0x78D5 { 1 } else { 2 };
            f(format!("vimpl::st_imm(c, {i:#06X}, {p:#06X}, {n})"))
        }
        0x78F3 => f(format!("vimpl::st_imm4(c, {j:#06X}, {p:#06X}, {i:#06X})")),
        0x7908 => f(format!("vimpl::native(c, {p:#06X})")),
        0x79AA => f(format!("vimpl::delay(c, {p:#06X})")),
        0x79BB => f("vimpl::menu(c)".into()),
        0x79C3 | 0x79C4 => f("vimpl::dialog(c)".into()),
        0x790A => f("vplan::kill_sel(c)".into()),
        0x7980 => f("vplan::wait_kind(c, 0xA)".into()),
        0x798E => f("vplan::wait_kind(c, 7)".into()),
        0x799C => f("vplan::wait_kind(c, 4)".into()),
        0x7A11 => f("vplan::levy(c)".into()),
        0x7A45 => f("vdir::disaster(c)".into()),
        0x7ADD => f("vmac::unhide(c)".into()),
        0x7AF1 => f("vmac::raze(c)".into()),
        0x7BAB => f("vmac::wait_derelict(c)".into()),
        0x7BF4 => f("vplan::depopulate(c)".into()),
        0x7C05 => f("vplan::charge(c)".into()),
        0x7C16 => f("vmac::reset_timers(c)".into()),
        0x7C30 => f("vmac::wait_ripe(c)".into()),
        _ => None,
    }
}

/// One decoded op — the call text (`None` for the `ret` stubs) and
/// whether it is the suspending `native` escape.
struct Op {
    call: Option<String>,
    native: bool,
}

/// Decode one script — runs to the ret stub or the `native` escape.
fn decode(b: &Blobs, mut si: u16) -> Vec<Op> {
    let mut ops = Vec::new();
    loop {
        let t = handler(b, b16(&b.ds, si));
        si = si.wrapping_add(2);
        let sh = shape(t);
        assert!(!sh.is_empty(), "{si:#06X}: unknown op {t:#06X}");
        let mut o = Vec::new();
        for ch in sh.chars().filter(|c| !matches!(c, 'n' | 't')) {
            take(ch, b, &mut si, &mut o);
        }
        ops.push(Op {
            call: call(t, &o),
            native: sh == "c",
        });
        if sh == "t" || sh == "c" {
            return ops;
        }
        assert!(ops.len() <= 0x100, "{si:#06X}: unterminated script");
    }
}

const HDR: &str = "//! @generated by `xgen` — see `src/bin/xgen`. Native event script.\n\n#[allow(unused_imports)]\nuse super::super::vops::Ctx;\nuse super::super::vm::Flow;\n#[allow(unused_imports)]\nuse super::super::{vdir, vimpl, vmac, vplan};\n\n";

/// Emit `src/game/script/` — `ev00..ev29`, `mod.rs`, `run.rs`.
pub fn emit(b: &Blobs, o: &mut Out) {
    let (mut mods, mut arms) = (String::new(), String::new());
    for i in 0..SLOTS {
        let si = b16(&b.ds, TAB + i * 4);
        let ops = decode(b, si);
        let head = format!(
            "/// Slot {i} — script at `ds:{si:#06X}`.\npub fn run(c: &mut Ctx) -> u16 {{\n"
        );
        let body = ops
            .iter()
            .enumerate()
            .map(|(k, op)| line(k + 1, op))
            .collect::<String>();
        o.file(
            &format!("game/script/ev{i:02}.rs"),
            &format!("{HDR}{head}{body}}}\n"),
        );
        mods += &format!("mod ev{i:02};\n");
        arms += &format!("        {si:#06X} => ev{i:02}::run(c),\n");
    }
    mods += "mod run;\n\npub use run::run;\n";
    o.file(
        "game/script/mod.rs",
        &format!("//! @generated by `xgen` — the shipped event scripts compiled.\n\n{mods}"),
    );
    o.file("game/script/run.rs", &run_rs(&arms));
}

/// One op line — suspending forms `if … == Ret { return k }`, the
/// `native` escape and the ret stub close the fn.
fn line(k: usize, op: &Op) -> String {
    match (&op.call, op.native) {
        (Some(e), true) => format!("    {e};\n    {k}\n"),
        (Some(e), false) => format!("    if {e} == Flow::Ret {{ return {k} }}\n"),
        (None, _) => format!("    {k}\n"),
    }
}

/// The `run.rs` dispatcher text.
fn run_rs(arms: &str) -> String {
    let uses = (0..30)
        .map(|i| format!("ev{i:02}"))
        .collect::<Vec<_>>()
        .join(", ");
    format!("//! @generated by `xgen` — event dispatch on the `ds:0x5183` slot `si`.\n\nuse super::super::vops::Ctx;\nuse super::super::vm::TAB_BASE;\nuse super::{{{uses}}};\n\n/// `cs:0x773D` — fetch slot `idx`'s `si` and run that compiled script.\npub fn run(c: &mut Ctx, idx: u16) -> u16 {{\n    let si = c.vm.r16(c.st, TAB_BASE + (idx & 0xFF) * 4);\n    at(c, si)\n}}\n\n/// `si` → compiled script (unknown cursors run nothing).\nfn at(c: &mut Ctx, si: u16) -> u16 {{\n    match si {{\n{arms}        _ => 0,\n    }}\n}}\n")
}
