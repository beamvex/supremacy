/// Saturating `usize → u16` for MZ header fields (sizes fit for valid
/// files; clamping keeps a malformed image from panicking).
pub(super) fn sat(v: usize) -> u16 {
    u16::try_from(v).unwrap_or(u16::MAX)
}
