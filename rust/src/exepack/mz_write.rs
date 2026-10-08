use super::sat::sat;
use super::Unpacked;

/// 28-byte MZ header for a repackaged image: signature, page geometry,
/// fixup count/table offset, and the real `ss:sp`/`cs:ip` from the EXEPACK
/// block. `minalloc`/`maxalloc`/`checksum` follow the unpacked reference.
pub(super) fn mz_write(u: &Unpacked, hdr_paras: usize) -> Vec<u8> {
    let total = hdr_paras * 16 + u.image.len();
    let mut b = Vec::with_capacity(28);
    b.extend_from_slice(b"MZ");
    for v in [
        sat(total % 512),
        sat(total.div_ceil(512)),
        sat(u.relocs.len()),
        sat(hdr_paras),
        0x74,
        0xFFFF,
        u.ss,
        u.sp,
        0,
        u.ip,
        u.cs,
        0x1C,
        0,
    ] {
        b.extend_from_slice(&v.to_le_bytes());
    }
    b
}
