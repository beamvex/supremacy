//! A `Vm`+`State`+`Rng` rig with a recording host that also serves
//! `cs:` words for the draw sequencer — shared by the main-loop tests.

use super::gsim::galaxy;
use supremacy::game::{Call, Ctx, Rng, State, Vm, VmHost};

/// Records every [`Call`]; `cs` backs `cs_word` fetches.
pub struct Rec(pub Vec<Call>, pub Vec<u8>);

impl VmHost for Rec {
    fn svc(&mut self, call: Call) -> u16 {
        self.0.push(call);
        0
    }
    fn cs_word(&mut self, ofs: u16) -> u16 {
        let a = usize::from(ofs);
        u16::from(self.1[a]) | u16::from(self.1[a + 1]) << 8
    }
}

/// `Vm`/`State`/`Rng`/host bundle with `ds:` accessors.
pub struct Rig {
    /// Low/high `ds` memory.
    pub vm: Vm,
    /// The saveable state block (preset galaxy 3).
    pub st: State,
    /// PRNG.
    pub rng: Rng,
    /// The recording host.
    pub host: Rec,
}

impl Rig {
    /// Fresh rig — zeroed `Vm`, `galaxy()` state, seed `0x42`.
    pub fn new() -> Self {
        Self {
            vm: Vm::new(),
            st: galaxy(),
            rng: Rng::new(0x42),
            host: Rec(Vec::new(), vec![0; 0x10000]),
        }
    }
    /// Borrow the four contexts.
    pub fn ctx(&mut self) -> Ctx<'_> {
        Ctx {
            vm: &mut self.vm,
            st: &mut self.st,
            rng: &mut self.rng,
            host: &mut self.host,
        }
    }
    /// `ds:` word read.
    pub fn r16(&mut self, a: usize) -> u16 {
        self.vm.r16(&self.st, u16::try_from(a).unwrap_or(0))
    }
    /// `ds:` word write.
    pub fn w16(&mut self, a: usize, v: u16) {
        self.vm.w16(&mut self.st, u16::try_from(a).unwrap_or(0), v);
    }
    /// `ds:` byte write.
    pub fn w8(&mut self, a: usize, v: u8) {
        self.vm.w8(&mut self.st, u16::try_from(a).unwrap_or(0), v);
    }
    /// `ds:` byte read.
    pub fn r8(&mut self, a: usize) -> u8 {
        self.vm.r8(&self.st, u16::try_from(a).unwrap_or(0))
    }
    /// Write a sequencer `{op, pad}` record into the cs image.
    pub fn rec(&mut self, ofs: u16, op: u16) {
        let a = usize::from(ofs);
        self.host.1[a] = u8::try_from(op & 0xFF).unwrap_or(0);
        self.host.1[a + 1] = u8::try_from(op >> 8).unwrap_or(0);
    }
}
