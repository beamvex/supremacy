//! The simple event ops — flag/flag-wait, print, register load,
//! copy/store/add immediates, countdown delay, menu/dialog host calls.
//! GAM-80: operands are baked in by `xgen`; these are the shared bodies
//! the compiled `script::evNN` functions call.

use super::consts::REC_COUNT;
use super::text;
use super::vhost::Call;
use super::vm::Flow;
use super::vops::Ctx;

/// `0x7769`/`0x78C7` — suspend while `byte[p] != set` / while
/// `byte[p] == 0`.
pub fn wait_b(c: &mut Ctx, p: u16, set: bool) -> Flow {
    if (c.vm.r8(c.st, p) != 0) == set {
        Flow::Cont
    } else {
        Flow::Ret
    }
}

/// `0x7796` — suspend until `byte[p] == imm8`.
pub fn wait_eq_b(c: &mut Ctx, p: u16, i: u8) -> Flow {
    if c.vm.r8(c.st, p) == i {
        Flow::Cont
    } else {
        Flow::Ret
    }
}

/// `0x77A5`/`0x77B4` — suspend until `word[p] == imm16`.
pub fn wait_eq_w(c: &mut Ctx, p: u16, i: u16) -> Flow {
    if c.vm.r16(c.st, p) == i {
        Flow::Cont
    } else {
        Flow::Ret
    }
}

/// `0x7774` — enqueue the message string at `p` (`call 0x6B06`).
pub fn print(c: &mut Ctx, p: u16) -> Flow {
    print_at(c, p)
}

/// `0x77C3` — print the current planet's name record.
pub fn print_planet(c: &mut Ctx) -> Flow {
    print_at(c, super::vops::planet_rec(c) + 0xE)
}

/// `0x77E1` — print the `[0x81EE]` selected record's name.
pub fn print_sel(c: &mut Ctx) -> Flow {
    print_at(c, c.vm.r16(c.st, super::vops::R_SEL) + 0xE)
}

/// `0x7BE8` — print at the `[0x81F2]` machine pointer.
pub fn print_mach(c: &mut Ctx) -> Flow {
    print_at(c, c.vm.r16(c.st, super::vops::R_MACH))
}

/// Print helper shared by the name-print ops.
pub fn print_at(c: &mut Ctx, addr: u16) -> Flow {
    text::enqueue(c, addr);
    Flow::Cont
}

/// `0x7782`/`0x778C` — `byte[p] = 0xFF`/`0`.
pub fn set_flag(c: &mut Ctx, p: u16, v: u8) -> Flow {
    c.vm.w8(c.st, p, v);
    Flow::Cont
}

/// `cs:0xE713` modal-dialog flag — the `0x79C3` op sets it entering the
/// loop; the host clears it when the dialog is dismissed.
pub fn enter_modal(c: &mut Ctx) {
    c.vm.modal = true;
}

/// `0x77F0` — `[0x81EE]` = a random planet record (`rand & [0x91B8]`,
/// retry on `==`).
pub fn rand_planet(c: &mut Ctx) -> Flow {
    let mask = c.vm.r16(c.st, u16::try_from(REC_COUNT).unwrap_or(0));
    let mut i = c.rng.next16() & mask;
    while i == mask {
        i = c.rng.next16() & mask;
    }
    let rec = c.vm.rec_ofs(c.st, i);
    c.vm.w16(c.st, super::vops::R_SEL, rec);
    Flow::Cont
}

/// `0x7811`/`0x7825`/`0x7839` — load `n` bytes through
/// `word[p] + imm` into the scratch registers.
pub fn ld_ind(c: &mut Ctx, p: u16, i: u16, reg: u16, n: u16) -> Flow {
    let a = c.vm.r16(c.st, p).wrapping_add(i);
    for k in 0..n {
        let b = c.vm.r8(c.st, a.wrapping_add(k));
        c.vm.w8(c.st, reg.wrapping_add(k), b);
    }
    Flow::Cont
}

/// `0x7853`/`0x7866`/`0x7879` — copy `n` bytes `src → dst`.
pub fn cp(c: &mut Ctx, src: u16, dst: u16, n: u16) -> Flow {
    for k in 0..n {
        let b = c.vm.r8(c.st, src.wrapping_add(k));
        c.vm.w8(c.st, dst.wrapping_add(k), b);
    }
    Flow::Cont
}

/// `0x7893`/`0x78A2`/`0x78B1` — add an immediate to `byte`/`word`/
/// `dword` at `ptr` (the dword form `adc`s the carry).
pub fn add_imm(c: &mut Ctx, ptr: u16, add: u16, n: u16) -> Flow {
    if n == 1 {
        let sum =
            c.vm.r8(c.st, ptr)
                .wrapping_add(u8::try_from(add).unwrap_or(0));
        c.vm.w8(c.st, ptr, sum);
        return Flow::Cont;
    }
    let (lo, carry) = c.vm.r16(c.st, ptr).overflowing_add(add);
    c.vm.w16(c.st, ptr, lo);
    if n == 4 {
        let hi = c.vm.r16(c.st, ptr + 2);
        c.vm.w16(c.st, ptr + 2, hi.wrapping_add(u16::from(carry)));
    }
    Flow::Cont
}

/// `0x78D5`/`0x78E4` — store immediate `byte`/`word` at `p`.
pub fn st_imm(c: &mut Ctx, p: u16, lo: u16, _n: u16) -> Flow {
    c.vm.w16(c.st, p, lo);
    Flow::Cont
}

/// `0x78F3` — store an immediate `dword` at `p`.
pub fn st_imm4(c: &mut Ctx, p: u16, lo: u16, hi: u16) -> Flow {
    c.vm.w16(c.st, p, lo);
    c.vm.w16(c.st, p + 2, hi);
    Flow::Cont
}

/// `0x7908` — `jmp word [si]`: a raw escape to a `cs:` routine. Nothing
/// returns to the VM, so it suspends.
pub fn native(c: &mut Ctx, cs: u16) -> Flow {
    c.host.svc(Call::Native(cs));
    Flow::Ret
}

/// `0x79AA` — countdown cell: suspend decrementing while nonzero.
pub fn delay(c: &mut Ctx, p: u16) -> Flow {
    let v = c.vm.r16(c.st, p);
    if v == 0 {
        Flow::Cont
    } else {
        c.vm.w16(c.st, p, v - 1);
        Flow::Ret
    }
}

/// `0x79BB` — `call 0x2FE7` menu service, then continue.
pub fn menu(c: &mut Ctx) -> Flow {
    c.host.svc(Call::Menu);
    Flow::Cont
}

/// `0x79C3` — the modal dialog block: sets the menu context at
/// `[0x9CC8]`/`[0x9194]`/`[0x91A8]`, runs the UI loop, then restores
/// menu `0x31`.
pub fn dialog(c: &mut Ctx) -> Flow {
    enter_modal(c);
    let v = c.vm.r16(c.st, 0xB030);
    c.vm.w16(c.st, 0x9CC8, v);
    c.vm.w16(c.st, 0x9194, 0xB032);
    c.vm.w16(c.st, 0x91A8, 2);
    c.host.svc(Call::Dialog);
    let v = c.vm.r16(c.st, 0xA0AA);
    c.vm.w16(c.st, 0x9CC8, v);
    c.vm.w16(c.st, 0x9194, 0xA0AC);
    c.vm.w16(c.st, 0x91A8, 0x31);
    Flow::Cont
}
