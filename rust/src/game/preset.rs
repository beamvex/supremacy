use super::consts::{DIFFICULTY, REC_ALT, REC_BASE, REC_COUNT, REC_CUR, REC_SEL, REC_TOTAL};
use super::State;

/// One galaxy-size/difficulty preset — the four pickers at file `0x36792`,
/// `0x367B2`, `0x367CD`, `0x367E8` (shared tail `0x36801`).
pub struct Preset {
    /// Record-array base `si` written to `[0x9158]`/`[0x917C]`/`[0x9184]`/
    /// `[0x9178]`.
    pub base: u16,
    /// Total records `ax` → `[0x91B6]`; planets = `ax − 1` → `[0x91B8]`.
    pub total: u16,
    /// Difficulty byte `bl` → `[0x91E4]` (drives the `F_TROOPS` write).
    pub difficulty: u8,
    /// The 5-byte banner written to the `ds:0x582B` scratch string
    /// (preset name in the game's charset — `RORN `/`KRART`/`SMINE`/
    /// `WOTOK` as written, little-endian word stores).
    pub banner: [u8; 5],
}

/// The four presets in menu order — word stores are little-endian, so the
/// asm's `mov word` immediates read back to front.
pub const PRESETS: [Preset; 4] = [
    Preset {
        base: 0x8486,
        total: 0x20,
        difficulty: 2,
        banner: *b"RORN ",
    },
    Preset {
        base: 0x8486,
        total: 0x20,
        difficulty: 2,
        banner: *b"KRART",
    },
    Preset {
        base: 0x8BC6,
        total: 0x10,
        difficulty: 1,
        banner: *b"SMINE",
    },
    Preset {
        base: 0x8F66,
        total: 0x08,
        difficulty: 0,
        banner: *b"WOTOK",
    },
];

/// Preset application — file `0x36801`–`0x3681F`: anchors the record array
/// (`si`), stores total/count (`ax`, `ax−1`) and difficulty (`bl`), plus
/// the `[0x91C6]` count byte. Returns the preset used.
pub fn select_galaxy(st: &mut State, idx: usize) -> &'static Preset {
    let p = &PRESETS[idx.min(PRESETS.len() - 1)];
    for slot in [REC_SEL, REC_CUR, REC_BASE, REC_ALT] {
        st.set_word(slot, p.base);
    }
    st.set_word(REC_TOTAL, p.total);
    st.set_word(REC_COUNT, p.total - 1);
    st.set_byte(0x91C6, u8::try_from(p.total - 1).unwrap_or(0));
    st.set_byte(DIFFICULTY, p.difficulty);
    p
}
