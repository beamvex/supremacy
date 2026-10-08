use crate::lzss::{Decoder, Frame, Kind};

/// EGA plane width in bytes (`0x28` = 40) and plane length (`0x1F40` =
/// 200 rows × 0x28).
const STRIDE: usize = 0x28;
const PLANE: usize = 0x1F40;

/// The EGA draw path (file `0x2FB36`): four passes over the record's
/// consecutive plane streams. The asm writes the same `di` into VRAM each
/// time with map masks 1/2/4/8 (`[cs:0x538D]`, programmed via `call
/// 0x8498`); here each pass targets that plane's slice instead. The EGA
/// decoder variants consume the terminator's trailing byte, leaving `si`
/// on the next stream.
pub(super) fn draw(buf: &mut [u8], src: &[u8], col: usize, row: usize, width: usize, kind: Kind) {
    let xor = matches!(kind, Kind::Xor);
    let di = row * STRIDE + col;
    let mut pos = 0;
    for plane in 0..4 {
        let slice = &mut buf[plane * PLANE..(plane + 1) * PLANE];
        let mut frame = Frame::new(slice, di, width, STRIDE, xor);
        Decoder::new().run(src, &mut pos, &mut frame, true);
    }
}
