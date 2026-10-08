/// Sound device — the index stored at `[0x155]` (`al` at `0x32076`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sound {
    /// PC speaker — index 1, timer variant `0` (also the default).
    PcSpeaker,
    /// Tandy 3-voice — index 2, timer variant `0xD5`.
    Tandy,
    /// `AdLib` — index 3, timer variant `0x1D0`.
    AdLib,
    /// Roland — index 4, timer variant `0x1D0`.
    Roland,
}
