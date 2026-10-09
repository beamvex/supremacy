//! The human script format the harness turns into `SCRIPT.BIN` — one
//! op per line, all fields hex (a leading `0x` is tolerated):
//!
//! ```text
//! # comment
//! 0A  key 1C            # Enter make at tick 0x0A
//! 0E  key 9C            # break
//! 14  mouse 1E 160 96 1 # event mask, x, y, buttons
//! 1E  dump ds 7D39 1F8B 01
//! 1E  dump drv 55A2 4 03
//! 1E  dump A000 0 FA00 04
//! 1E  pal 05
//! 1E  stat 06
//! 28  waitcell 91CD 5   # hold until uimode == galaxy
//! 32  pokew drv 55A2 1234
//! 3C  done
//! ```
//!
//! Ops apply in file order once their tick has passed; scripts are kept
//! sorted by tick at encode time (stable).

use super::op::{Op, Rec};
use super::seg::Seg;

/// A parsed scenario — records in script order.
pub struct Script(pub Vec<Rec>);

impl Script {
    /// The `SCRIPT.BIN` payload — records sorted by tick (stable).
    #[must_use]
    pub fn to_bin(&self) -> Vec<u8> {
        let mut rs = self.0.clone();
        rs.sort_by_key(|r| r.tick);
        rs.iter().flat_map(Rec::encode).collect()
    }
}

fn hx(t: &str) -> Option<u16> {
    u16::from_str_radix(t.trim_start_matches("0x"), 16).ok()
}

fn hx8(t: &str) -> Option<u8> {
    u8::from_str_radix(t.trim_start_matches("0x"), 16).ok()
}

/// Parse one script line (already stripped); `None` for blank/comment.
fn op_at(f: &[&str]) -> Option<Op> {
    match *f.first()? {
        "key" => Some(Op::Key {
            scan: hx8(f.get(1)?)?,
        }),
        "mouse" => Some(Op::Mouse {
            mask: hx8(f.get(1)?)?,
            x: hx(f.get(2)?)?,
            y: hx(f.get(3)?)?,
            buttons: hx(f.get(4)?)?,
        }),
        "dump" => Some(Op::Dump {
            id: hx8(f.get(4)?)?,
            seg: Seg::parse(f.get(1)?)?,
            ofs: hx(f.get(2)?)?,
            len: hx(f.get(3)?)?,
        }),
        "pokew" => Some(Op::Poke {
            seg: Seg::parse(f.get(1)?)?,
            ofs: hx(f.get(2)?)?,
            val: hx(f.get(3)?)?,
        }),
        "pal" => Some(Op::Pal {
            id: hx8(f.get(1)?)?,
        }),
        "stat" => Some(Op::Stat {
            id: hx8(f.get(1)?)?,
        }),
        "waitcell" => Some(Op::WaitCell {
            ofs: hx(f.get(1)?)?,
            val: hx8(f.get(2)?)?,
        }),
        "done" => Some(Op::Done),
        _ => None,
    }
}

/// Parse a whole script — silently skips blank lines, `#` comments and
/// malformed lines (the CLI reports the kept op count).
#[must_use]
pub fn parse(text: &str) -> Script {
    let mut rs = Vec::new();
    for line in text.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        let f: Vec<&str> = line.split_whitespace().collect();
        let (Some(tick), Some(tail)) = (f.first().and_then(|t| hx(t)), f.get(1..)) else {
            continue;
        };
        let Some(op) = op_at(tail) else { continue };
        rs.push(Rec { tick, op });
    }
    Script(rs)
}
