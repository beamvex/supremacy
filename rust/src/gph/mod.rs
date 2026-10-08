//! `.GPH` graphics index — fixed 12-byte records.
//!
//! ```c
//! struct gph_rec { uint16_t x, y, w, h, ofs_para, mode_flag; };
//! ```
//!
//! `x,y` = draw position, `w,h` = image size (`w` is bytes-per-row for the
//! mode), `ofs_para` = paragraph offset of the record's compressed stream
//! inside `.BIN`, and `mode_flag` selects opaque (`0`) vs XOR overlay (`1`)
//! draw — `dl` tested at `0x2FB80`. At load time the game adds the heap
//! segment to `ofs_para` in place (loop at `0x320D8`).

mod fixup;
mod parse;
mod record;
mod record_ofs;

pub use fixup::fix_up;
pub use parse::parse_index;
pub use record::{Record, RECORD_LEN};
