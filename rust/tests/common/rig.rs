//! A `Vm`+`State`+`Rng` rig with a recording host that also serves
//! `cs:` words for the draw sequencer — shared by the main-loop tests.

use std::collections::VecDeque;

use super::gsim::galaxy;
use supremacy::game::{Call, Ctx, Rng, State, Vm, VmHost};
use supremacy::platform::MemFs;

/// Records every [`Call`]; `cs` backs `cs_word` fetches; `keys` is the
/// channel-A stack [`Call::KeyPop`] drains (LIFO — push in read order).
pub struct Rec(pub Vec<Call>, pub Vec<u8>, pub VecDeque<u8>);

impl VmHost for Rec {
    fn svc(&mut self, call: Call) -> u16 {
        self.0.push(call);
        if call == Call::KeyPop {
            return u16::from(self.2.pop_back().unwrap_or(0));
        }
        0
    }
    fn cs_word(&mut self, ofs: u16) -> u16 {
        let a = usize::from(ofs);
        u16::from(self.1[a]) | u16::from(self.1[a + 1]) << 8
    }
}

/// `Vm`/`State`/`Rng`/host/files bundle with `ds:` accessors.
pub struct Rig {
    /// Low/high `ds` memory.
    pub vm: Vm,
    /// The saveable state block (preset galaxy 3).
    pub st: State,
    /// PRNG.
    pub rng: Rng,
    /// The recording host.
    pub host: Rec,
    /// In-memory save filesystem.
    pub files: MemFs,
}

impl Rig {
    /// Fresh rig — zeroed `Vm`, `galaxy()` state, seed `0x42`.
    pub fn new() -> Self {
        Self {
            vm: Vm::new(),
            st: galaxy(),
            rng: Rng::new(0x42),
            host: Rec(Vec::new(), vec![0; 0x10000], VecDeque::new()),
            files: MemFs::new(),
        }
    }
    /// Borrow the five contexts.
    pub fn ctx(&mut self) -> Ctx<'_> {
        Ctx {
            vm: &mut self.vm,
            st: &mut self.st,
            rng: &mut self.rng,
            host: &mut self.host,
            files: &mut self.files,
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
    /// Script channel-A keys — each make scancode plus its break
    /// (`0x80 | scan`); the break's table miss is what clears the
    /// `[0x4FCA]` debounce so the next make dispatches.
    pub fn keys(&mut self, makes: &[u8]) {
        for &m in makes.iter().rev() {
            self.host.2.push_back(m);
            self.host.2.push_back(m | 0x80);
        }
    }

    /// Write a sequencer `{op, pad}` record into the cs image.
    pub fn rec(&mut self, ofs: u16, op: u16) {
        let a = usize::from(ofs);
        self.host.1[a] = u8::try_from(op & 0xFF).unwrap_or(0);
        self.host.1[a + 1] = u8::try_from(op >> 8).unwrap_or(0);
    }
}
