use super::{Config, Sound, Video};

/// Parse the PSP command tail (bytes starting at `es:0x80`: count byte,
/// space, then single-letter arguments every other byte).
///
/// Absent cells read as `0`, which lands the parser on the same defaults the
/// asm falls through to: MCGA video, real mouse, PC speaker.
#[must_use]
pub fn parse(tail: &[u8]) -> Config {
    let at = |psp_off: usize| tail.get(psp_off - 0x80).copied().unwrap_or(0);
    let (sound, timer_variant) = Sound::from_arg(at(0x86));
    Config {
        video: Video::from_arg(at(0x82)),
        keyboard_mouse: at(0x84) == b'K',
        sound,
        timer_variant,
    }
}
