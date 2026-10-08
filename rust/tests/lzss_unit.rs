//! Hand-built streams exercising the decoder's flag/match/terminator logic
//! and the strided/XOR draw paths the asm uses for VRAM.

mod common;

use supremacy::lzss::{decode_at, decode_planes, draw, Blit, Decoder, Kind};

/// `FF` + 8 literals + flag byte `00` + `00 00` token + `00` tail.
const LITS: &[u8] = b"\xFFABCDEFGH\x00\x00\x00\x00";

/// flag `0x03`: lit 'A', lit 'B', match (pos `0xFEE`, len 3 — overlaps the
/// write cursor, so it repeats "ABABA"), match flag `0` + `00 00` + `00`.
const WITH_MATCH: &[u8] = b"\x03AB\xEE\xF0\x00\x00\x00";

#[test]
fn literals_decode_and_mcg_leaves_tail() {
    let (out, i) = decode_at(LITS, 0);
    assert_eq!(&out, b"ABCDEFGH");
    assert_eq!(i, 12, "MCG decoder stops past the 00 00 token");
}

#[test]
fn ega_variant_consumes_trailing_byte() {
    let mut i = 0;
    let mut out = Vec::new();
    Decoder::new().run(LITS, &mut i, &mut out, true);
    assert_eq!(&out, b"ABCDEFGH");
    assert_eq!(i, 13, "EGA lodsb eats the trailing zero");
}

#[test]
fn match_token_copies_from_ring() {
    let (out, _) = decode_at(WITH_MATCH, 0);
    assert_eq!(&out, b"ABABA");
}

#[test]
fn draw_wraps_rows_by_stride() {
    let stream = b"\xFFABCDEFGH\x0FIJKL\x00\x00\x00";
    let mut dst = [0u8; 18];
    let mut i = 0;
    draw(
        stream,
        &mut i,
        &mut Blit {
            dst: &mut dst,
            di: 0,
            width: 4,
            stride: 6,
            kind: Kind::Opaque,
            eat_tail: false,
        },
    );
    assert_eq!(&dst[..4], b"ABCD");
    assert_eq!(&dst[6..10], b"EFGH");
    assert_eq!(&dst[12..16], b"IJKL");
}

#[test]
fn xor_draw_overlays_destination() {
    let mut dst = *b"________XX";
    let mut i = 0;
    draw(
        LITS,
        &mut i,
        &mut Blit {
            dst: &mut dst,
            di: 0,
            width: 8,
            stride: 8,
            kind: Kind::Xor,
            eat_tail: false,
        },
    );
    let want: Vec<u8> = b"ABCDEFGH"
        .iter()
        .zip(b"________")
        .map(|(a, b)| a ^ b)
        .collect();
    assert_eq!(&dst[..8], want.as_slice());
    assert_eq!(&dst[8..], b"XX");
}

#[test]
fn ega_planes_chain_through_one_buffer() {
    let two = [LITS, LITS].concat();
    let (out, i) = decode_planes(&two, 0, 2);
    assert_eq!(&out, b"ABCDEFGHABCDEFGH");
    assert_eq!(i, 26);
}
