use super::cmd::Cmd;

impl Cmd {
    /// `test al,1` at `0x23B12` — the low bit marks the final block.
    pub(super) fn last(&self) -> bool {
        match self {
            Self::Fill { last, .. } | Self::Copy { last, .. } => *last,
        }
    }
}
