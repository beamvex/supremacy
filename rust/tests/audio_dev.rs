//! Audio device drivers: exact port-op sequences for the OPL2 write and
//! detect paths, the MPU-401 UART handshake, and the speaker gate —
//! checked against the asm byte streams in `decompiled/FUNCTIONS.md`.

use supremacy::audio::{Mpu, Opl, Ports, Speaker};

/// Records every `in`/`out`; `in` returns queued replies (or `last`).
struct Rec {
    ops: Vec<(bool, u16, u8)>,
    replies: std::collections::VecDeque<u8>,
    last: u8,
}

impl Rec {
    fn new() -> Self {
        Self {
            ops: Vec::new(),
            replies: std::collections::VecDeque::new(),
            last: 0,
        }
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

#[test]
fn opl_write_sequence_matches_asm() {
    let mut o = Opl::new(Rec::new());
    o.write(0xBD, 0xC0);
    let ops = &o.ports.ops;
    assert_eq!(ops[0], (false, 0x388, 0xBD));
    assert_eq!(ops[1..7], [(true, 0x388, 0); 6]);
    assert_eq!(ops[7], (false, 0x389, 0xC0));
    assert_eq!(ops[8..45], [(true, 0x388, 0); 37]);
    assert_eq!(ops.len(), 45);
}

#[test]
fn opl_detect_is_canonical_timer_test() {
    let mut o = Opl::new(Rec::new());
    // Status reads in order: 2 writes' delays (86), st0, 2 more writes'
    // delays (86), 4 delay loops (152), a discard, then st1.
    let mut replies = vec![0u8; 327];
    replies[326] = 0xC0;
    o.ports.replies = replies.into();
    assert!(o.detect());
    let outs: Vec<u8> = o
        .ports
        .ops
        .iter()
        .filter(|op| !op.0)
        .map(|op| op.2)
        .collect();
    assert_eq!(
        outs,
        [0x04, 0x60, 0x04, 0x80, 0x02, 0xFF, 0x04, 0x21, 0x04, 0x60, 0x04, 0x80]
    );
}

#[test]
fn opl_detect_rejects_silent_or_stuck() {
    let mut o = Opl::new(Rec::new());
    // st0 already has flags set → absent.
    o.ports.last = 0xE0;
    assert!(!o.detect());
    // st0 quiet but timers never expire → absent.
    let mut o2 = Opl::new(Rec::new());
    o2.ports.last = 0;
    assert!(!o2.detect());
}

#[test]
fn mpu_reset_and_command_handshake() {
    let mut m = Mpu::new(Rec::new());
    // DSR clear immediately; DRR clear then data read returns 0xFE.
    m.ports.replies.extend([0x00, 0x00, 0xFE]);
    assert!(m.reset());
    assert_eq!(m.ack, 0xFE);
    assert_eq!(m.ports.ops[1], (false, 0x331, 0xFF));
    // command: status polls (DSR, DRR) then a data read.
    m.ports.replies.extend([0x00, 0x00, 0xFE]);
    assert_eq!(m.command(0x3F), 0xFE);
    assert!(m.ports.ops.contains(&(false, 0x331, 0x3F)));
}

#[test]
fn mpu_sysex_checksum_negated() {
    let mut m = Mpu::new(Rec::new());
    m.ports.last = 0;
    m.sysex(&[0x41, 0x10], &[0x16, 0x12]);
    let outs: Vec<u8> = m
        .ports
        .ops
        .iter()
        .filter(|op| !op.0 && op.1 == 0x330)
        .map(|op| op.2)
        .collect();
    assert_eq!(outs, [0x41, 0x10, 0x16, 0x12, 0x58, 0xF7]);
}

#[test]
fn speaker_note_programs_pit_once() {
    let mut s = Speaker::new(Rec::new());
    s.note(0x1234);
    let outs: Vec<(u16, u8)> = s
        .ports
        .ops
        .iter()
        .filter(|op| !op.0)
        .map(|op| (op.1, op.2))
        .collect();
    assert_eq!(outs[..3], [(0x43, 0xB6), (0x42, 0x34), (0x42, 0x12)]);
    assert_eq!(outs[3], (0x61, 3));
    // Same divisor skips the PIT program (asm `cmp [0x2E],cx / jz`).
    s.ports.ops.clear();
    s.note(0x1234);
    let outs: Vec<(u16, u8)> = s
        .ports
        .ops
        .iter()
        .filter(|op| !op.0)
        .map(|op| (op.1, op.2))
        .collect();
    assert_eq!(outs, [(0x61, 3)]);
}

#[test]
fn opl_key_and_freq_writes() {
    let mut o = Opl::new(Rec::new());
    o.key_off(2);
    o.freq(2, 0x169, 4, 0x20);
    let outs: Vec<u8> = o
        .ports
        .ops
        .iter()
        .filter(|op| !op.0)
        .map(|op| op.2)
        .collect();
    assert_eq!(outs, [0xA2, 0x00, 0xB2, 0x00, 0xA2, 0x69, 0xB2, 0x31]);
}
