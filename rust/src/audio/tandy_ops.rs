//! Shared Tandy op bodies that survive GAM-80 — stream events are
//! dispatched in `tandy_step`; `op_end` is also used by `tandy_tick`.

use super::{Ports, Tandy};

impl<P: Ports> Tandy<P> {
    /// `0x1CA7` — `0x86` end: `stop`, then one render + gate pass
    /// (the asm pops the step frame and pushes the `cs:0x243` =
    /// `0x1B93` tick tail).
    pub(super) fn op_end(&mut self) {
        self.stop();
        self.render();
        self.gate();
    }
}
