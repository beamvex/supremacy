//! FNV-1a 64 — the same parameters as `tests/common::fnv` and the
//! Python fixture generator. Streaming so the audio digest can fold
//! each rendered chunk without holding the whole PCM stream.

/// Streaming FNV-1a 64 state.
pub struct Fnv(u64);

impl Fnv {
    /// The offset basis.
    #[must_use]
    pub fn new() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }

    /// Fold bytes in.
    pub fn write(&mut self, b: &[u8]) {
        for &x in b {
            self.0 ^= u64::from(x);
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }

    /// Fold one `u32` little-endian — used for `f32` sample bits.
    pub fn write_u32(&mut self, v: u32) {
        self.write(&v.to_le_bytes());
    }

    /// The digest so far.
    #[must_use]
    pub fn sum(&self) -> u64 {
        self.0
    }
}

impl Default for Fnv {
    fn default() -> Self {
        Self::new()
    }
}

/// One-shot FNV-1a 64 over `b`.
#[must_use]
pub fn fnv1a(b: &[u8]) -> u64 {
    let mut h = Fnv::new();
    h.write(b);
    h.sum()
}
