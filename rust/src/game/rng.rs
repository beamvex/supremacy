/// The game PRNG — `cs:0x55B2` (file `0x2E862`).
///
/// A 32-bit LCG over a two-word state (`[cs:0x55A2]`/`[cs:0x55A4]`):
/// each call advances `s = 13·s + 7` (built from two doublings, a `+7`
/// pair and two recombination adds — all `add`/`adc` on 16-bit halves),
/// folds the halves `r = lo(s) ^ hi(s)` and returns
/// `hi16((b+1)·r)` for `range(b)`.
///
/// The second entry point (file `0x2E93C`) is the same scramble with
/// `bx = 0xFFFF`, i.e. the full folded word — [`Rng::next16`].
pub struct Rng {
    s: u32,
}

impl Rng {
    /// Seed with the 32-bit `[cs:0x55A4]:[cs:0x55A2]` state (zero in the
    /// image; the driver reseeds from the timer at runtime).
    #[must_use]
    pub fn new(seed: u32) -> Self {
        Self { s: seed }
    }

    /// `cs:0x55B2` — `rand(bx)` → `bx` ∈ `0..=b`. The asm does `inc bx`
    /// then `mul` and keeps `dx`, so the result is `hi16((b+1)·r)`.
    pub fn range(&mut self, b: u16) -> u16 {
        let r = u32::from(self.step());
        let hi = (u32::from(b) + 1).wrapping_mul(r) >> 16;
        u16::try_from(hi).unwrap_or(0)
    }

    /// Second entry (`bx = 0xFFFF`, file `0x2E93C`) — the folded state word.
    pub fn next16(&mut self) -> u16 {
        self.step()
    }

    /// Current LCG state — the `cs:0x55A4:0x55A2` pair the difftest
    /// harness reads back out of a DOS memory dump.
    #[must_use]
    pub fn state(&self) -> u32 {
        self.s
    }

    /// Replace the LCG state — difftest checkpoint boot from a
    /// `drv:0x55A2` dump.
    pub fn set_state(&mut self, s: u32) {
        self.s = s;
    }

    /// The `s = 13·s + 7` scramble + `lo^hi` fold (file `0x2E863`–`0x2E92F`).
    fn step(&mut self) -> u16 {
        let a = self.s;
        let p = a.wrapping_add(7);
        let m = a.wrapping_mul(4);
        let a = m.wrapping_mul(2).wrapping_add(p).wrapping_add(m);
        self.s = a;
        u16::try_from(a & 0xFFFF).unwrap_or(0) ^ u16::try_from(a >> 16).unwrap_or(0)
    }
}
