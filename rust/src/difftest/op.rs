//! Probe ops — one per fixed 12-byte `SCRIPT.BIN` record:
//! `[u16 tick][u8 op][u8 a][u16 b][u16 c][u16 d][u16 pad]` (LE).

use super::seg::Seg;

/// Record size on the wire.
pub const REC_LEN: usize = 12;

/// A single scheduled action the probe (and the port replay) performs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Op {
    /// `1` — scancode into the `[0x9CCB]` latch + `drv:0x8B72` LIFO
    /// (`Pump::key`).
    Key {
        /// The code, bit 7 = break.
        scan: u8,
    },
    /// `2` — far-call the game's int-33h event handler like the real
    /// driver.
    Mouse {
        /// Driver event mask (`a`).
        mask: u8,
        /// Cursor x in pixels (`b`; the handler receives it doubled).
        x: u16,
        /// Cursor y in pixels (`c`).
        y: u16,
        /// Button state (`d`) — bit 0 left, bit 1 right.
        buttons: u16,
    },
    /// `3` — write `len` bytes at `seg:ofs` to `D<id>.BIN`.
    Dump {
        /// Artefact id (`D<id>.BIN`).
        id: u8,
        /// Segment selector (`ds`, `drv`, absolute).
        seg: Seg,
        /// Offset within the segment.
        ofs: u16,
        /// Byte count.
        len: u16,
    },
    /// `4` — word write at `seg:ofs`.
    Poke {
        /// Segment selector.
        seg: Seg,
        /// Offset within the segment.
        ofs: u16,
        /// The word value.
        val: u16,
    },
    /// `5` — write `DONE.BIN`, the host's end-of-run marker.
    Done,
    /// `6` — dump the 768-byte VGA DAC to `P<id>.BIN`.
    Pal {
        /// Artefact id (`P<id>.BIN`).
        id: u8,
    },
    /// `7` — write the 24-byte probe debug block to `S<id>.BIN`.
    Stat {
        /// Artefact id (`S<id>.BIN`).
        id: u8,
    },
    /// `8` — block until `ds:[ofs] == val` (game-state sync point).
    WaitCell {
        /// `ds:` offset to poll.
        ofs: u16,
        /// Byte value to wait for.
        val: u8,
    },
}

impl Op {
    fn code(&self) -> u8 {
        match *self {
            Self::Key { .. } => 1,
            Self::Mouse { .. } => 2,
            Self::Dump { .. } => 3,
            Self::Poke { .. } => 4,
            Self::Done => 5,
            Self::Pal { .. } => 6,
            Self::Stat { .. } => 7,
            Self::WaitCell { .. } => 8,
        }
    }
}

/// An op plus the probe tick it applies at/after.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rec {
    /// Probe tick count (int-8 chain calls) that schedules the op.
    pub tick: u16,
    /// The op.
    pub op: Op,
}

impl Rec {
    /// The 12-byte `SCRIPT.BIN` record.
    #[must_use]
    #[allow(clippy::many_single_char_names)] // a/b/c/d are the record's field names
    pub fn encode(&self) -> [u8; REC_LEN] {
        let (a, b, c, d) = match self.op {
            Op::Key { scan } => (scan, 0, 0, 0),
            Op::Mouse {
                mask,
                x,
                y,
                buttons,
            } => (mask, x, y, buttons),
            Op::Dump { id, seg, ofs, len } => (id, ofs, len, seg.word()),
            Op::Poke { seg, ofs, val } => (0, ofs, val, seg.word()),
            Op::Done => (0, 0, 0, 0),
            Op::Pal { id } | Op::Stat { id } => (id, 0, 0, 0),
            Op::WaitCell { ofs, val } => (0, ofs, u16::from(val), 0),
        };
        let mut r = [0u8; REC_LEN];
        r[0..2].copy_from_slice(&self.tick.to_le_bytes());
        r[2] = self.op.code();
        r[3] = a;
        r[4..6].copy_from_slice(&b.to_le_bytes());
        r[6..8].copy_from_slice(&c.to_le_bytes());
        r[8..10].copy_from_slice(&d.to_le_bytes());
        r
    }
}
