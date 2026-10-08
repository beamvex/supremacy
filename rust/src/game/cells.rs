//! `ds:` cell offsets for the shell/UI layer — the menu-screen
//! hotspot install, status ticker, blink timer, oval hover table,
//! and the `cs:0x2CC5` shell flags (`decompiled/FUNCTIONS.md` §
//! "Main loop"/"UI shell").

/// `ds:` slot of the shell-loop record-list bound — a click row must
/// be `< [0x91C6] + 1` (file `0x35D87`).
pub const LIST_MAX: usize = 0x91C6;
/// `ds:` slot of the status-ticker x cursor `[0x91C0]` (file
/// `0x36BFC`).
pub const TICK_X: usize = 0x91C0;
/// `ds:` slot of the ticker record cursor `[0x915C]` — points into a
/// stream of 4-byte `{msg-ptr, unused}` records (file `0x36BA3`).
pub const TICK_PTR: usize = 0x915C;
/// `ds:` slot of the ticker write cursor `[0x9160]` — the `0x6B06`
/// enqueue appends `{msg-ptr, 0}` records here (file `0x36B2F`).
pub const TICK_PUT: usize = 0x9160;
/// `ds:` slot of the decimal-digit scratch `[0x9CC0]` — the `0x68BD`/
/// `0x69C6` divisor loops accumulate `'0' + count` here (file
/// `0x368BD`/`0x369C6`).
pub const DIGI: usize = 0x9CC0;
/// `ds:` slot of the ticker record countdown `[0x91C2]` — `0xFF` ops
/// decrement it; `0` disables the ticker (file `0x36BD4`).
pub const TICK_LEFT: usize = 0x91C2;
/// `ds:` slot of the ticker per-step delay `[0x91D2]` — `0xFC` op
/// reloads it, otherwise it decrements (file `0x36B97`).
pub const TICK_WAIT: usize = 0x91D2;
/// `ds:` slot of the blink-timer countdown `[0x91D1]` — armed `1` by
/// the ticker's `0xFD` op; nonzero suppresses the ticker (files
/// `0x36BDD`/`0x33D7A`).
pub const BLINK_T: usize = 0x91D1;
/// `ds:` slot of the blink hold flag `[0x91F2]` — skips the phase
/// machinery while set (file `0x36B51`).
pub const BLINK_HOLD: usize = 0x91F2;
/// `ds:` slot of the blink sub-phase `[0x91F3]` — `0x11` starts the
/// two-string flash (file `0x36B58`).
pub const BLINK_PH: usize = 0x91F3;
/// `ds:` slot cleared on shell entry `[0x91E8]` — also cleared by the
/// ticker's `0xFF` end-of-record op (file `0x32CE6`/`0x36BC4`).
pub const TICK_FLAG: usize = 0x91E8;
/// `ds:` slot cleared on shell entry and on the galaxy loop's exit
/// `[0x91EA]` (file `0x32CE1`/`0x33A1C`).
pub const OVERLAY_F: usize = 0x91EA;
/// `ds:` slot receiving a copy of `[0x91A6]` on shell entry (file
/// `0x32D2A`).
pub const SHELL_A6: usize = 0x928B;
/// `ds:` slot holding the shell's per-frame source `[0x91A6]` (file
/// `0x32D27`).
pub const SHELL_SRC: usize = 0x91A6;
/// `ds:` offset of the wrap-to record base the ticker's `0xFE`
/// sentinel jumps to (`0x928D`, file `0x36BD1`).
pub const TICK_BASE: usize = 0x928D;
/// `ds:` offset of the menu-screen hotspot selection word — the
/// `0x14`-stride list base minus 2 (`0xA0AA`, file `0x32D31`).
pub const MENU_SEL: usize = 0xA0AA;
/// `ds:` offset of the menu-screen hotspot list (`0xA0AC`, file
/// `0x32D37`).
pub const MENU_LIST: usize = 0xA0AC;
/// Menu-screen hotspot count installed by the shell (`0x31`, file
/// `0x32D3D`).
pub const MENU_COUNT: u16 = 0x31;
/// `ds:` base of the per-row half-width byte table the `0x8DC6`
/// oval/scrollbar hit test indexes (`[bx+0xA8]`, file `0x38DEE`).
pub const OVAL_TAB: usize = 0xA8;
/// `ds:` offset of the "alien name" string `0x6AD2` prints when the
/// selected record's `+0xC` is `1` (`0x71B9`, file `0x36AE9`).
pub const ALIEN_NAME: usize = 0x71B9;
/// `ds:` offsets of the two blink strings `0x6B4A` alternates at
/// `(0x4D, 0x67)` (`0x77AA`/`0x77E3`, file `0x36B71`/`0x36B76`).
pub const BLINK_A: usize = 0x77AA;
/// Second blink string (file `0x36B76`).
pub const BLINK_B: usize = 0x77E3;

/// `ds:` slot of the modal-dialog result flag `[0x91DA]` — the
/// `0x2F9B` confirm and `0x2E21` dialogs wait for the menu actions
/// `0x2F01`/`0x2F08`/`0x2F95` to write `1`/`2`/`0xFF` here (file
/// `0x32F01`/`0x32F08`/`0x32F95`).
pub const DLG_FLAG: usize = 0x91DA;
/// `ds:` slot of the post-load screen selector `[0x91E5]` — the load
/// action `jmp`s to `0x685A`/`0x6847`/`0x6834`/`0x6821` on 0/1/2/other
/// (file `0x32EB1`–`0x32ECF`).
pub const LOAD_TO: usize = 0x91E5;
/// `ds:` slot of the line-input mode flag `[0x9CC5]` — `1` while the
/// `0xE66E` filename editor owns the keyboard (file `0x32F4F`).
pub const LINE_MODE: usize = 0x9CC5;
/// `ds:` slot of the `[0x1278]`-slot mode byte `[0x9CCA]` — `1`/`2`
/// around the load's pre/post image loads (file `0x38E66`/`0x38E75`).
pub const SLOT_MODE: usize = 0x9CCA;
/// `ds:` slot of the filename length byte `[0x4FCB]` (file `0x32F2D`).
pub const NAME_LEN: usize = 0x4FCB;
/// `ds:` slot of the filename limit byte `[0x4FCC]` — `0x1E` during
/// input, `9` after (file `0x32F32`/`0x32F5F`).
pub const NAME_MAX: usize = 0x4FCC;
/// `ds:` slot of the line-editor dirty flag `[0x4FCA]` — set when an
/// edit changed the buffer (file `0x2E6F3`).
pub const IN_DIRTY: usize = 0x4FCA;
/// `ds:` slot of the line-editor done flag `[0x4FCF]` (file `0x2E6F8`).
pub const IN_DONE: usize = 0x4FCF;
/// `ds:` offset of the save/load dialog's selection word (`0xB05A`,
/// file `0x32E47`).
pub const DLG_SEL: usize = 0xB05A;
/// `ds:` offset of the save/load dialog's 3-record hotspot list
/// (`0xB05C`, file `0x32E4D`).
pub const DLG_LIST: usize = 0xB05C;
/// `ds:` offset of the confirm dialog's selection word (`0xB098`,
/// file `0x32FA4`).
pub const CNF_SEL: usize = 0xB098;
/// `ds:` offset of the confirm dialog's 2-record hotspot list
/// (`0xB09A`, file `0x32FAA`).
pub const CNF_LIST: usize = 0xB09A;
/// `ds:` offset of the filename prompt script (`0x78E5`, file
/// `0x32E94`/`0x32ED2`).
pub const S_PROMPT: usize = 0x78E5;
/// `ds:` offset of the filename-input legend (`0x78C3`, file
/// `0x32F37`).
pub const S_LEGEND: usize = 0x78C3;
/// `ds:` offset of the save/load error message (`0x7929`, file
/// `0x32EE7`).
pub const S_ERROR: usize = 0x7929;
/// `ds:` offset of the save-ok message (`0x7C2E`, file `0x32EE0`).
pub const S_SAVED: usize = 0x7C2E;
/// `ds:` offset of the post-load message (`0x7C50`, file `0x32EAB`).
pub const S_LOADED: usize = 0x7C50;
/// `ds:` offset of the confirm title (`0x78B8`, file `0x32F21`).
pub const S_CNF_T: usize = 0x78B8;
/// `ds:` offset of the confirm body (`0x7C72`, file `0x32F9E`).
pub const S_CNF_B: usize = 0x7C72;
