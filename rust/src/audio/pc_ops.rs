//! The shared PC op bodies that survive GAM-80 — stream events are
//! dispatched in [`pc_step`][super::pc_step]; `end_song` is also used
//! by the song/sfx loaders.

use super::{Pc, Ports};

impl<P: Ports> Pc<P> {
    /// `0xFA1` — the end marker clears `loaded`, `sfx_on`, `mus`.
    pub(super) fn end_song(&mut self) {
        self.loaded = 0;
        self.sfx_on = 0;
        self.mus = 0;
    }
}
