/// The `add al,[0x9ca1]` / `cmp al,0x40` staging rule shared by the
/// `0x2ED30`/`0x2F1CF` DAC loaders: a 6-bit component takes the shade
/// offset (the `al` add wraps), clamps at `0x3F`, then scales to
/// 8-bit like [`super::from_dac`]. `gate` is the row/`pal_adj` loops'
/// `cmp al,0` — zero components stay zero; the `dx == 6` master loop
/// adds unconditionally.
pub(super) fn shade(src: u8, off: u8, gate: bool) -> u8 {
    let v = if gate && src == 0 {
        0
    } else {
        src.wrapping_add(off).min(0x3F)
    };
    u8::try_from(u16::from(v) * 255 / 63).unwrap_or_default()
}
