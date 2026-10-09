//! Dump artefact naming and the probe's 24-byte debug block.

/// `D<id>.BIN` — a `DUMP` op's raw region.
#[must_use]
pub fn dump_name(id: u8) -> String {
    format!("D{id:02X}.BIN")
}

/// `P<id>.BIN` — a `PAL` op's 768-byte VGA DAC dump.
#[must_use]
pub fn pal_name(id: u8) -> String {
    format!("P{id:02X}.BIN")
}

/// `S<id>.BIN` — a `STAT` op's debug block.
#[must_use]
pub fn stat_name(id: u8) -> String {
    format!("S{id:02X}.BIN")
}

/// Written once when the probe first sees the game's `ds`.
pub const ARMED_NAME: &str = "ARMED.BIN";
/// Written by the `DONE` op — the host's end-of-run marker.
pub const DONE_NAME: &str = "DONE.BIN";
/// Debug-block size on the wire.
pub const DBG_LEN: usize = 24;

/// Decoded `ARMED.BIN`/`S<id>.BIN` content.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Dbg {
    /// Captured game data segment.
    pub game_ds: u16,
    /// Captured driver (int-8/9 ISR) segment.
    pub drv_seg: u16,
    /// Probe tick count when the block was written.
    pub tick: u16,
    /// Script bytes consumed so far.
    pub op_ptr: u16,
    /// Total script bytes.
    pub op_total: u16,
    /// The game's int-33h event handler (`0` in K mode).
    pub mhand: u32,
    /// DOS `InDOS` flag address (seg:ofs packed — seg in the high half).
    pub indos: u32,
}

/// Parse a debug block (short buffers give a zeroed block).
#[must_use]
pub fn dbg_parse(b: &[u8]) -> Dbg {
    let w = |o: usize| -> u16 {
        u16::from_le_bytes([
            b.get(o).copied().unwrap_or(0),
            b.get(o + 1).copied().unwrap_or(0),
        ])
    };
    Dbg {
        game_ds: w(0),
        drv_seg: w(2),
        tick: w(4),
        op_ptr: w(6),
        op_total: w(8),
        mhand: u32::from(w(10)) | u32::from(w(12)) << 16,
        indos: u32::from(w(16)) | u32::from(w(14)) << 16,
    }
}
