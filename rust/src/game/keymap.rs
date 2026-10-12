//! The `ds:0x7CBD` keymap the `0xE66E` line editor and the `0x32BA8`
//! yes/no poll scan — 62 `{code, out}` byte pairs shipped at
//! `ds:0x7CBD` itself (file `0x1835D`), ending exactly at the save
//! block `0x7D39` (the earlier `ds:0x76BF` "source" was an artifact of
//! the stale `+0x10CA0` file mapping — runtime dumps confirm the map
//! lives at `0x7CBD` as shipped).
//!
//! `out` classes: `0` exits, `1` backspaces, `2` pads the field with
//! spaces and exits, anything else is the char appended while
//! `[0x4FCB] < [0x4FCC]`. Uppercase chars only — `'\'` arrives as
//! `0x5B` ('[') and the `ds:0` sanitiser remaps it to `0x5C`.

/// `ds:0x7CBD` — the 62-entry `{code, output}` keymap the `0xE66E`
/// line editor and the `0x32BA8` yes/no poll read (ends exactly at the
/// save block `0x7D39`).
pub const KEYMAP: usize = 0x7CBD;
/// Keymap entry count — 62 `{code, output}` byte pairs.
pub const KEYMAP_N: usize = 0x3E;

/// The 124 shipped bytes — `0x01`→exit (Esc) first, then erase keys
/// (`0x0E`/`0x70`/`0x53`), pad keys (`0x1C`/`0x74`), digits, keypad,
/// `-`/`=`, QWERTY rows (`';'`→`':'`, `'\'`→`'['`), `2C..35` row,
/// space, and the `0x47`/`0x56` `'['` entries (`0x47` is shadowed by
/// the keypad-`7` entry — first match wins).
pub const KEYMAP_BYTES: [u8; 124] = [
    0x01, 0x00, 0x0E, 0x01, 0x70, 0x01, 0x53, 0x01, 0x1C, 0x02, 0x74, 0x02, // esc + edits
    0x02, b'1', 0x03, b'2', 0x04, b'3', 0x05, b'4', 0x06, b'5', // row
    0x07, b'6', 0x08, b'7', 0x09, b'8', 0x0A, b'9', 0x0B, b'0', 0x4F, b'1', 0x50, b'2', 0x51, b'3',
    0x4B, b'4', 0x4C, b'5', // pad
    0x4D, b'6', 0x47, b'7', 0x48, b'8', 0x49, b'9', 0x52, b'0', 0x0C, b'-', 0x0D,
    b'=', // punct
    0x10, b'Q', 0x11, b'W', 0x12, b'E', 0x13, b'R', 0x14, b'T', // qwerty
    0x15, b'Y', 0x16, b'U', 0x17, b'I', 0x18, b'O', 0x19, b'P', 0x1E, b'A', 0x1F, b'S', 0x20, b'D',
    0x21, b'F', 0x22, b'G', // home
    0x23, b'H', 0x24, b'J', 0x25, b'K', 0x26, b'L', 0x27, b':', 0x2B, b'[', 0x2C, b'Z', 0x2D, b'X',
    0x2E, b'C', 0x2F, b'V', // bottom
    0x30, b'B', 0x31, b'N', 0x32, b'M', 0x33, b',', 0x34, b'.', 0x35, b'/', 0x39, b' ', 0x47, b'[',
    0x56, b'[',
];

/// Install the runtime keymap at `ds:0x7CBD` from the embedded
/// shipped bytes — for rigs with no image.
pub fn install(low: &mut [u8]) {
    low[KEYMAP..KEYMAP + KEYMAP_BYTES.len()].copy_from_slice(&KEYMAP_BYTES);
}
