//! Script ops — the `.scr` replay vocabulary [`run`][super::run]
//! applies to a [`Game`][crate::front::Game]: input injection
//! (`key`/`mouse`), state pokes and waits, plus the artefact ops
//! (`dump`/`pal`/`stat`) that parse but do nothing under the digest
//! runner — the hash channels are the snapshot.

use super::seg::Seg;

/// A single scheduled action the replay performs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Op {
    /// `key` — scancode into the `[0x9CCB]` latch + `drv:0x8B72` LIFO
    /// (`Pump::key`).
    Key {
        /// The code, bit 7 = break.
        scan: u8,
    },
    /// `mouse` — feed the driver event mask, cursor position and
    /// button state through `Pump` like the int-33h handler does.
    Mouse {
        /// Driver event mask.
        mask: u8,
        /// Cursor x in pixels (the handler receives it doubled).
        x: u16,
        /// Cursor y in pixels.
        y: u16,
        /// Button state — bit 0 left, bit 1 right.
        buttons: u16,
    },
    /// `dump` — a `seg:ofs`/`len` memory artefact; a no-op under
    /// `smoke` (snapshots here are the hash channels, not files).
    Dump {
        /// Artefact id.
        id: u8,
        /// Segment selector (`ds`, `drv`, absolute).
        seg: Seg,
        /// Offset within the segment.
        ofs: u16,
        /// Byte count.
        len: u16,
    },
    /// `pokew` — word write at `seg:ofs`.
    Poke {
        /// Segment selector.
        seg: Seg,
        /// Offset within the segment.
        ofs: u16,
        /// The word value.
        val: u16,
    },
    /// `done` — end the run at this tick.
    Done,
    /// `pal` — a 768-byte VGA DAC artefact; a no-op under `smoke`.
    Pal {
        /// Artefact id.
        id: u8,
    },
    /// `stat` — a 24-byte status artefact; a no-op under `smoke`.
    Stat {
        /// Artefact id.
        id: u8,
    },
    /// `waitcell` — block until `ds:[ofs] == val` (state sync point).
    WaitCell {
        /// `ds:` offset to poll.
        ofs: u16,
        /// Byte value to wait for.
        val: u8,
    },
}

/// An op plus the tick it applies at/after.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rec {
    /// Tick count (one tick = one `Game::step`) scheduling the op.
    pub tick: u16,
    /// The op.
    pub op: Op,
}
