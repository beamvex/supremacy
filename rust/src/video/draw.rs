use super::{ega, Banked, Screen};
use crate::args::Video;
use crate::assets::AssetSet;
use crate::gph::Record;
use crate::lzss::{Decoder, Frame, Kind};

const MCG_STRIDE: usize = 0x140;

fn draw_kind(rec: &Record) -> Option<Kind> {
    match rec.mode_flag {
        0 => Some(Kind::Opaque),
        1 => Some(Kind::Xor),
        _ => None,
    }
}

impl Screen {
    /// `ds:[0x125A]` — draw record `idx` of `set` at its (x,y).
    ///
    /// Port of the four per-mode routines (`decompiled/FUNCTIONS.md`): the
    /// `ds:[0x1298]` dirty-region call is region bookkeeping, not clipping,
    /// so it's omitted; `ofs_para ≥ 0xA000` can't occur post-fixup and is
    /// dropped the same way MCG drops it. An out-of-range `idx` is a
    /// no-op — the asm's `idx * 0xC` wraps to a wild `ds:` record read
    /// rather than faulting.
    pub fn draw_image(&mut self, set: &AssetSet, idx: usize) {
        let Some(&rec) = set.records.get(idx) else {
            return;
        };
        let (Some(kind), src) = (draw_kind(&rec), set.stream(idx)) else {
            return;
        };
        let (col, row, w) = (usize::from(rec.x), usize::from(rec.y), usize::from(rec.w));
        match self.mode {
            Video::Ega => ega::draw(&mut self.buf, src, col, row, w, kind),
            Video::Cga | Video::Tga => self.banked(src, col, row, w, kind),
            Video::Mcga => {
                let mut pos = 0;
                let xor = matches!(kind, Kind::Xor);
                let di = self.mode.di_at(col, row);
                let mut f = Frame::new(&mut self.buf, di, w, MCG_STRIDE, xor);
                Decoder::new().run(src, &mut pos, &mut f, false);
            }
        }
    }

    fn banked(&mut self, src: &[u8], col: usize, row: usize, width: usize, kind: Kind) {
        let (span, pitch) = match self.mode {
            Video::Cga => (0x4000, 0x50),
            _ => (0x8000, 0xA0),
        };
        let di = self.mode.di_at(col, row);
        let xor = matches!(kind, Kind::Xor);
        let mut f = Banked::new(&mut self.buf, di, width, 0x2000, span, pitch, xor);
        let mut pos = 0;
        Decoder::new().run(src, &mut pos, &mut f, false);
    }
}
