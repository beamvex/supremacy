//! A recording `Ports` bus — captures every `in`/`out` so driver tests
//! can assert the exact port-op sequences; `in` returns queued replies
//! (or `last`).

use std::collections::VecDeque;

use supremacy::audio::Ports;

/// The recorded ops: `(is_in, port, val)` in issue order.
pub struct Rec {
    /// Every port access, in order.
    pub ops: Vec<(bool, u16, u8)>,
    /// Values `inb` returns before falling back to `last`.
    pub replies: VecDeque<u8>,
    /// `inb` fallback once `replies` drains.
    pub last: u8,
}

impl Rec {
    /// A fresh recorder — `in` reads `0`.
    pub fn new() -> Self {
        Self {
            ops: Vec::new(),
            replies: VecDeque::new(),
            last: 0,
        }
    }

    /// Just the `out` ops, `(port, val)` pairs.
    pub fn outs(&self) -> Vec<(u16, u8)> {
        self.ops
            .iter()
            .filter(|op| !op.0)
            .map(|op| (op.1, op.2))
            .collect()
    }
}

impl Ports for Rec {
    fn outb(&mut self, port: u16, val: u8) {
        self.ops.push((false, port, val));
    }
    fn inb(&mut self, port: u16) -> u8 {
        let v = self.replies.pop_front().unwrap_or(self.last);
        self.ops.push((true, port, v));
        v
    }
}

impl Default for Rec {
    fn default() -> Self {
        Self::new()
    }
}
