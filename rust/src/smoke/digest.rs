//! The run digest — five FNV-1a channels so a regression names its
//! subsystem: `video` (the mode-laid-out VRAM), `pal` (the 256-colour
//! table), `audio` (every PCM sample rendered), `state` (the
//! reassembled `ds` image) and `rng` (the final LCG state).

use std::fmt;

/// What a [`run`][crate::smoke::run] measured.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Digest {
    /// `Game::step`s run (including wait-loop steps the ops consumed).
    pub steps: u16,
    /// FNV-1a of `Host::scr.buf`.
    pub video: u64,
    /// FNV-1a of the 768 palette bytes (`rgb(0..=255)`).
    pub pal: u64,
    /// FNV-1a of the rendered PCM `f32` bits.
    pub audio: u64,
    /// FNV-1a of [`ds_image`][crate::difftest::ds_image].
    pub state: u64,
    /// Final `Rng` state.
    pub rng: u32,
}

impl Digest {
    /// The value `key` names — `steps`/`video`/`pal`/`audio`/`state`/
    /// `rng`; `None` for anything else.
    #[must_use]
    pub fn field(&self, key: &str) -> Option<u64> {
        Some(match key {
            "steps" => u64::from(self.steps),
            "video" => self.video,
            "pal" => self.pal,
            "audio" => self.audio,
            "state" => self.state,
            "rng" => u64::from(self.rng),
            _ => return None,
        })
    }

    /// Compare against a spec — `key=value` pairs separated by commas
    /// or whitespace (`steps` decimal, the rest hex, `0x` tolerated).
    /// Returns one description per mismatch or malformed pair, so
    /// `check(spec).is_empty()` is the pass test.
    #[must_use]
    pub fn check(&self, spec: &str) -> Vec<String> {
        spec.split([',', ' ', '\t'])
            .filter(|s| !s.is_empty())
            .filter_map(|p| self.pair(p))
            .collect()
    }

    /// `Some(problem)` when the pair names an unknown key, a bad value
    /// or a mismatching field.
    fn pair(&self, p: &str) -> Option<String> {
        let Some((k, v)) = p.split_once('=') else {
            return Some(format!("malformed expectation {p:?}"));
        };
        let Some(want) = expect(k, v) else {
            return Some(format!("bad value in expectation {p:?}"));
        };
        let Some(got) = self.field(k) else {
            return Some(format!("unknown field {k:?} in {p:?}"));
        };
        (got != want).then(|| format!("{k}: got {got:X}, want {want:X}"))
    }
}

/// Parse one expected value — `steps` decimal, the rest hex.
fn expect(k: &str, v: &str) -> Option<u64> {
    if k == "steps" {
        v.parse().ok()
    } else {
        u64::from_str_radix(v.trim_start_matches("0x"), 16).ok()
    }
}

impl fmt::Display for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "steps={} video={:016X} pal={:016X} audio={:016X} state={:016X} rng={:08X}",
            self.steps, self.video, self.pal, self.audio, self.state, self.rng
        )
    }
}
