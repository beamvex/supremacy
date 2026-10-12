//! The `ds:0x50E3` op jump table — the `cs:0x7758` fetch loop reads a
//! script word `w` and runs `jmp word [ds:0x50E3 + w]` (file
//! `0x3775B`–`0x37766`).
//!
//! The table ships as `0x28` far pointers `{ofs, seg}`: `ofs` is a
//! near offset in the `0x292B0`-based code segment (`file − 0x292B0`,
//! observed `0xE4B9..0xE980`), `seg` the unrelocated `0x286B` marker
//! the loader rewrites to the real code segment (observed `0x32E1`
//! live). Script op words step by 4 — `0x00`→`cs:0x7769`,
//! `0x04`→`cs:0x7796`, …, `0x9C`→`cs:0x7C30`.
//!
//! [`fix`] rewrites each `ofs` into the port's `cs:` label space
//! (`file − 0x30000`, i.e. `−0x6D50`) so [`super::vops::op`] can match
//! the documented handler labels. It keys on the offset itself — a
//! near offset is `≥ 0x8000`, a `cs:` label `< 0x8000` — so it works
//! on both the shipped image and relocated live `ds` dumps, and leaves
//! tables tests poked with labels untouched.

/// Entry count — `0xA0` bytes of 4-byte far pointers.
pub const JT_N: usize = 0x28;
/// `0x30000 − 0x292B0` — the shipped near offset minus the port `cs:` label.
const DELTA: u16 = 0x6D50;

/// Rewrite the near-offset entries at `base` in `low` so word 0 holds
/// the port `cs:` label. Entries already holding a `< 0x8000` label
/// are left alone.
pub fn fix(low: &mut [u8], base: usize) {
    for i in 0..JT_N {
        let p = base + i * 4;
        let ofs = u16::from_le_bytes([low[p], low[p + 1]]);
        if ofs >= 0x8000 {
            low[p..p + 2].copy_from_slice(&ofs.wrapping_sub(DELTA).to_le_bytes());
        }
    }
}
