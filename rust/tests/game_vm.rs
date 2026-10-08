//! The scripted-event VM — operand decode, region reads/writes, the
//! suspend/re-enter contract, and host-call plumbing. Scripts are
//! hand-assembled in `Vm::low`: a script word `w` indexes
//! `JT_BASE + w` for the handler `cs:` address (asm `jmp word
//! [bx+0x50E3]`).

mod common;

use supremacy::game::{self, Call, Ctx, NullHost, Rng, State, Vm, VmHost, JT_BASE, TAB_BASE};

/// A recording host — captures every [`Call`] the ops emit.
struct Rec(Vec<Call>);

impl VmHost for Rec {
    fn svc(&mut self, call: Call) -> u16 {
        self.0.push(call);
        0
    }
}

/// Build a context over owned parts; `run` closures against it.
struct Rig {
    vm: Vm,
    st: State,
    rng: Rng,
    host: Rec,
    files: supremacy::platform::MemFs,
}

impl Rig {
    fn new() -> Self {
        Self {
            vm: Vm::new(),
            st: State::new(),
            rng: Rng::new(0x42),
            host: Rec(Vec::new()),
            files: supremacy::platform::MemFs::new(),
        }
    }

    fn ctx(&mut self) -> Ctx<'_> {
        Ctx {
            vm: &mut self.vm,
            st: &mut self.st,
            rng: &mut self.rng,
            host: &mut self.host,
            files: &mut self.files,
        }
    }

    /// `game::run` on this rig's context.
    fn run(&mut self, idx: u16) -> u16 {
        game::run(&mut self.ctx(), idx)
    }

    /// Byte write through the VM regions.
    fn set8(&mut self, a: u16, v: u8) {
        let c = self.ctx();
        c.vm.w8(c.st, a, v);
    }

    /// Word write through the VM regions.
    fn set16(&mut self, a: u16, v: u16) {
        let c = self.ctx();
        c.vm.w16(c.st, a, v);
    }

    /// Byte read through the VM regions.
    fn get8(&mut self, a: u16) -> u8 {
        let c = self.ctx();
        c.vm.r8(c.st, a)
    }

    /// Write op word `w` → handler `target` into the jump table.
    fn jt(&mut self, w: u16, target: u16) {
        let a = usize::from(JT_BASE) + usize::from(w);
        self.vm.low[a..a + 2].copy_from_slice(&target.to_le_bytes());
    }

    /// Point event slot `idx` at script `si`.
    fn event(&mut self, idx: u16, si: u16) {
        let a = usize::from(TAB_BASE) + usize::from(idx) * 4;
        self.vm.low[a..a + 2].copy_from_slice(&si.to_le_bytes());
    }

    /// Write a word into low memory (script bytes).
    fn w16(&mut self, a: u16, v: u16) {
        let i = usize::from(a);
        self.vm.low[i..i + 2].copy_from_slice(&v.to_le_bytes());
    }
}

#[test]
fn regions_split_at_the_save_block() {
    let mut r = Rig::new();
    r.set8(0x1234, 0xAB); // low
    r.set8(0x8150, 0xCD); // inside the save block
    r.set8(0x9DD0, 0xEF); // hi
    assert_eq!(r.get8(0x1234), 0xAB);
    assert_eq!(r.st.byte(0x8150), 0xCD);
    assert_eq!(r.get8(0x9DD0), 0xEF);
    r.set16(0x9CC3, 0x1122); // straddles block end → hi start
    assert_eq!(r.st.byte(0x9CC3), 0x22);
    let c = r.ctx();
    assert_eq!(c.vm.r16(c.st, 0x9CC3), 0x1122);
}

#[test]
fn set_flag_then_suspend() {
    let mut r = Rig::new();
    r.jt(0x40, 0x7782); // set flag 0xFF
    r.jt(0x44, 0x7768); // ret
    r.event(0, 0x6000);
    r.w16(0x6000, 0x40);
    r.w16(0x6002, 0x8250); // operand slot {ptr, tag}
    r.w16(0x6004, 0x0FAA);
    r.w16(0x6006, 0x44);
    assert_eq!(r.run(0), 2);
    assert_eq!(r.get8(0x8250), 0xFF);
}

#[test]
fn wait_gate_reruns_until_true() {
    let mut r = Rig::new();
    r.jt(0x40, 0x7796); // wait byte[p] == imm8
    r.jt(0x44, 0x7782); // set flag 0xFF
    r.jt(0x48, 0x7768); // ret
    r.event(0, 0x6000);
    r.w16(0x6000, 0x40);
    r.w16(0x6002, 0x8250); // {flag cell, tag}
    r.w16(0x6004, 0x0FAA);
    r.w16(0x6006, 1); // imm 1
    r.w16(0x6008, 0x44);
    r.w16(0x600A, 0x8252);
    r.w16(0x600C, 0x0FAA);
    r.w16(0x600E, 0x48);
    assert_eq!(r.run(0), 1); // suspends on the wait
    r.set8(0x8250, 1);
    assert_eq!(r.run(0), 3); // re-runs whole script
    assert_eq!(r.get8(0x8252), 0xFF);
}

#[test]
fn delay_counts_down_across_reruns() {
    let mut r = Rig::new();
    r.jt(0x40, 0x79AA); // countdown cell
    r.jt(0x44, 0x7768);
    r.event(0, 0x6000);
    r.w16(0x6000, 0x40);
    r.w16(0x6002, 0x8260);
    r.w16(0x6004, 0x0FAA);
    r.w16(0x6006, 0x44);
    r.set16(0x8260, 2);
    assert_eq!(r.run(0), 1); // 2 → 1
    assert_eq!(r.run(0), 1); // 1 → 0
    assert_eq!(r.run(0), 2); // 0 → continue, ret
}

#[test]
fn print_op_enqueues_ticker() {
    let mut r = Rig::new();
    r.jt(0x40, 0x7774); // print → 0x6B06 ticker enqueue
    r.jt(0x44, 0x7768);
    r.event(0, 0x6000);
    r.w16(0x6000, 0x40);
    r.w16(0x6002, 0x5B41); // "STATION HAS BEEN VAPORISED!"
    r.w16(0x6004, 0x0FAA);
    r.w16(0x6006, 0x44);
    assert_eq!(r.run(0), 2);
    assert_eq!(r.host.0[0], Call::Chan(0x8A5B, 0));
    assert_eq!(r.host.0[1], Call::Chan(0x8A3B, 0x1F));
    assert_eq!(r.host.0[2], Call::Sound(0xC));
    let c = r.ctx();
    assert_eq!(c.vm.r16(c.st, 0x91C2), 1); // record queued
    assert_eq!(c.vm.r16(c.st, 0), 0x5B41); // {si,0} at the write cursor
    assert_eq!(c.vm.r8(c.st, 0x91E8), 0xFF);
}

#[test]
fn rand_planet_stores_a_record() {
    let mut r = Rig::new();
    r.st = common::gsim::galaxy();
    r.jt(0x40, 0x77F0); // pick random planet → [0x81EE]
    r.jt(0x44, 0x7768);
    r.event(0, 0x6000);
    r.w16(0x6000, 0x40);
    r.w16(0x6002, 0x44);
    assert_eq!(r.run(0), 2);
    let c = r.ctx();
    let sel = c.vm.r16(c.st, 0x81EE);
    let (base, n) = (c.st.word(game::REC_BASE), c.st.word(game::REC_COUNT));
    assert!((sel - base).is_multiple_of(0x3A) && (sel - base) / 0x3A < n);
}

#[test]
fn tick_dispatches_script_events() {
    let mut st = common::gsim::galaxy();
    let n = u8::try_from(st.word(game::REC_TOTAL)).unwrap();
    st.set_byte(game::SEQ, 0x38 + 2 * n - 1);
    let tick = common::gsim::step(&mut st, &mut Rng::new(1), 's');
    assert!(matches!(tick, game::Tick::Script(_)));
    assert_eq!(NullHost.svc(Call::Menu), 0); // sink drops host calls
}
