use super::consts::{N, R_INIT};
use super::Decoder;

impl Decoder {
    /// Decoder prologue: zero the ring (`mov cx,0x7F7` / `mov word [bp],0`
    /// loop) and set the write cursor `bp = 0xFEE`. `dx` starts at zero so the
    /// first `shr` triggers a flag reload.
    #[must_use]
    pub fn new() -> Self {
        Self {
            ring: [0; N],
            r: R_INIT,
            flags: 0,
        }
    }
}
