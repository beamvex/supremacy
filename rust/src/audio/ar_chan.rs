/// One `0x20`-byte channel record (`cs:0x1A00 + i·0x20`).
#[derive(Clone, Copy, Default)]
pub(super) struct AChan {
    /// `+0x00` — up to four simultaneous MIDI/OPL note numbers.
    pub notes: [u8; 4],
    /// `+0x04` — program (`timbre` slot) requested by the stream.
    pub prog: u8,
    /// `+0x05` — pending-event class (the `kind` dispatch key).
    pub kind: u8,
    /// `+0x06` — the event's parameter byte.
    pub param: u8,
    /// `+0x07` — channel volume as last computed (`0..0x40`).
    pub cvol: u8,
    /// `+0x08` — Roland pitch-bend register (`0x2000` centred).
    pub bend: u16,
    /// `+0x0A` — last note keyed on (the slide's "from").
    pub last_note: u8,
    /// `+0x0B` — last program sent (the `C0`-change dedup).
    pub last_prog: u8,
    /// `+0x0C` — slide target pitch (`0` = not sliding).
    pub slide_tgt: u16,
    /// `+0x0E` — slide direction (`0` up / `1` down — Roland;
    /// `AdLib`'s `slide_set` leaves `0` = down, `1` = up instead).
    pub slide_dir: u8,
    /// `+0x0F` — slide rate (the kind-3 param, latched once).
    pub slide_rate: u8,
    /// `+0x10` — current pitch value (Roland table `idx` / `AdLib` `fnum`).
    pub pitch: u16,
    /// `+0x12` — the slide's destination note.
    pub slide_note: u8,
    /// `+0x14` — per-channel 4-op/paired-voice flag from the `timbre`.
    pub op4: u8,
    /// `+0x15` — `AdLib` block (octave) for the current note.
    pub block: u8,
    /// `+0x16` — `AdLib` f-number (`pitch & 0x3FF`).
    pub fnum: u16,
}
