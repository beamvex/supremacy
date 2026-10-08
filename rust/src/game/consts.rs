//! `ds:` offsets and sizes for the saveable state block — the real
//! addresses the asm uses, so `State` accessors take `ds:` values.

/// `ds:` offset where the save block starts (`dx` in the read/write calls,
/// file `0x383E1`/`0x38415`).
pub const STATE_OFS: usize = 0x7D39;
/// Save block length: `cx = 0x9CC4 − 0x7D39` (file `0x383DD`/`0x38411`).
/// Every savegame file is exactly this many bytes.
pub const STATE_LEN: usize = 0x1F8B;
/// `ds:` slot of the record-array base pointer (`[0x9158]`, written by the
/// galaxy preset at file `0x36809`).
pub const REC_BASE: usize = 0x9158;
/// `ds:` slot of the total record count (`[0x91B6]`, file `0x36811`).
pub const REC_TOTAL: usize = 0x91B6;
/// `ds:` slot of the planet count — also the index of the appended player
/// faction record (`[0x91B8]`, file `0x36815`).
pub const REC_COUNT: usize = 0x91B8;
/// `ds:` slot of the difficulty byte (`[0x91E4]`, file `0x3681B`).
pub const DIFFICULTY: usize = 0x91E4;
/// `ds:` slot the preset also mirrors the record base into (`[0x917C]`).
pub const REC_SEL: usize = 0x917C;
/// `ds:` slot the preset also mirrors the record base into (`[0x9184]`).
pub const REC_CUR: usize = 0x9184;
/// `ds:` slot the preset also mirrors the record base into (`[0x9178]`).
pub const REC_ALT: usize = 0x9178;
/// `ds:` slot of the selected/paced record index (`[0x9164]` —
/// initialised 6/14/30 by the presets; the `0x5D52` row-click updates
/// it and `0x5E03` derives `[0x9184]`/`[0x917C]` from it, files
/// `0x383AA`/`0x35E03`).
pub const SEL_IDX: usize = 0x9164;
/// Byte stride of a planet/faction record (`[0x9CB6] = 0x3A`, file
/// `0x35E41`).
pub const REC_STRIDE: usize = 0x3A;
/// `ds:` slot of the selected record pointer (`[0x917C]`, file `0x33D3D`).
pub const SEL_REC: usize = 0x917C;
/// `ds:` slot of the tick-in-day counter (`[0x91BA]`: `1..0x41`, wraps
/// into `DAY`, file `0x36A7B`–`0x36A8B`).
pub const TICK: usize = 0x91BA;
/// `ds:` slot of the day counter (`[0x91BC]`, file `0x36A87`).
pub const DAY: usize = 0x91BC;
/// `ds:` slot of the event-code sequencer (`[0x91CE]`, incremented by
/// `tick_step` at file `0x37403`).
pub const SEQ: usize = 0x91CE;
/// `ds:` slot of dirty-flag word 0 (`[0x9144]` — the stat-refresh
/// dispatcher reads it, file `0x33C6D`).
pub const DIRTY0: usize = 0x9144;
/// `ds:` slot of dirty-flag word 1 (`[0x9146]`).
pub const DIRTY1: usize = 0x9146;
/// `ds:` slot of the UI mode byte (`[0x91CD]`, file `0x33C02`).
pub const UIMODE: usize = 0x91CD;
/// `ds:` base of the machine/station array — `0x20` records of `0x28`
/// bytes (file `0x3741B`–`0x37421`).
pub const MACH_BASE: usize = 0x9491;
/// Machine records ticked per sequencer cycle (file `0x3740E`).
pub const MACH_COUNT: u16 = 0x20;
/// Machine record stride (file `0x37419`).
pub const MACH_STRIDE: usize = 0x28;
/// `ds:` base of the fleet/ship array — `0x17` records of `0xC` bytes
/// (file `0x345DA`/`0x34634`).
pub const FLEET_BASE: usize = 0x9991;
/// Ship records scanned per defence pass (file `0x345E0`).
pub const FLEET_COUNT: u16 = 0x17;
/// Ship record stride (file `0x34634`).
pub const FLEET_STRIDE: usize = 0xC;
/// Global: famine modifier — halves the growth score (file `0x33E08`).
pub const G_FAMINE: usize = 0x822B;
/// Global: horticultural research — boosts farm food output
/// (file `0x374E8`).
pub const G_FARMTECH: usize = 0x8229;
/// Global: mining research — boosts core-miner output (file `0x37476`).
pub const G_MINETECH: usize = 0x822C;
/// Global: deep-mining rate — mining pulls `0x32` instead of `0x19`
/// (file `0x3754E`).
pub const G_DEEPMINE: usize = 0x822D;
/// Global: delayed-depopulation flag (file `0x33EF9`).
pub const G_KILL_ON: usize = 0x821F;
/// Global: delayed-depopulation target record pointer (file `0x33F07`).
pub const G_KILL_REC: usize = 0x8220;
/// Global: cleared with the depopulation (file `0x33F12`).
pub const G_KILL_AUX: usize = 0x8224;
/// `ds:` slot of the colonise target record pointer (`[0x9170]`, file
/// `0x375C1`).
pub const QUEUED_REC: usize = 0x9170;
/// `ds:` slot of the selected machine pointer (`[0x9180]`, file
/// `0x37568`).
pub const SEL_MACH: usize = 0x9180;
/// `ds:` slot of the defence pass's current record (`[0x918C]`, file
/// `0x34458`).
pub const SCAN_REC: usize = 0x918C;
/// `ds:` slot of the captured-record message pointer (`[0x9140]`, file
/// `0x3467A`).
pub const MSG_REC: usize = 0x9140;
/// `ds:` slot of the fleet-scan abort cursor (`[0x914C]`, UI mode 1,
/// file `0x345EE`).
pub const LINK_REC: usize = 0x914C;
/// `ds:` slot of the reinforce-record pointer (`[0x9150]`, file
/// `0x348AB`).
pub const REINF_REC: usize = 0x9150;
/// `ds:` slot of the countdown message scratch (`[0x8314]`, file
/// `0x375B3`).
pub const MSG_BUILD: usize = 0x8314;
/// `ds:` slot of the mining message scratch (`[0x8318]`, file
/// `0x3757D`).
pub const MSG_MINE: usize = 0x8318;
/// Frame counter `[0x91D4]` — incremented once per main-loop pass
/// (file `0x339FE`).
pub const FRAME: usize = 0x91D4;
/// Redraw-request flag `[0x91EE]` — gates the refresh block (file
/// `0x33961`).
pub const REDRAW: usize = 0x91EE;
/// Info-panel request flag `[0x91EF]` (file `0x33997`).
pub const PANEL_REQ: usize = 0x91EF;
/// Selected machine-type index `[0x91ED]` — `ds:0x9B12` stride `0x30`
/// (file `0x33A5A`).
pub const TYPE_SEL: usize = 0x91ED;
/// Selected machine-type record pointer `[0x9154]` (file `0x33A7E`).
pub const TYPE_REC: usize = 0x9154;
/// Panel-visible flag `[0x91DB]` set from the type record (file
/// `0x33AA4`).
pub const PANEL_ON: usize = 0x91DB;
/// Sequencer delay countdown `[0x91A0]` (file `0x35C1E`).
pub const SEQ_DELAY: usize = 0x91A0;
/// Sequencer phase counter `[0x91D7]` (file `0x35C32`).
pub const SEQ_PHASE: usize = 0x91D7;
/// Sequencer period `[0x91D6]` — `0xFF` disables (file `0x35C2B`).
pub const SEQ_PERIOD: usize = 0x91D6;
/// Sequencer command-list cursor `[0x9198]` — a `cs:` pointer (file
/// `0x35C49`).
pub const SEQ_PTR: usize = 0x9198;
/// Sequencer aux pointer cleared on stream end `[0x919A]` (file
/// `0x35C90`).
pub const SEQ_AUX: usize = 0x919A;
/// Sound-enable sentinel `[0x91D3]` — `0xFF` mutes the driver (file
/// `0x38579`).
pub const SND_ON: usize = 0x91D3;
/// Mouse-buttons word `[0x9CDA]` — bit 0 left, bit 1 right (file
/// `0x2A119`).
pub const BUTTONS: usize = 0x9CDA;
/// Cursor x `[0x9CD6]` (file `0x2A128`).
pub const CUR_X: usize = 0x9CD6;
/// Cursor y `[0x9CD8]` (file `0x2A12C`).
pub const CUR_Y: usize = 0x9CD8;
/// Hotspot-list pointer `[0x9194]`, `0x14`-byte records (file
/// `0x2A130`).
pub const HOT_LIST: usize = 0x9194;
/// Hotspot entry count `[0x91A8]` (file `0x2A134`).
pub const HOT_COUNT: usize = 0x91A8;
/// Selected hotspot index `[0x9CC8]` (file `0x2A20E`).
pub const HOT_SEL: usize = 0x9CC8;
/// Button-press debounce latch `[0x91CA]` (file `0x2A121`).
pub const ARMED: usize = 0x91CA;
/// Pressed-state image index of the hit hotspot `[0x9188]` (file
/// `0x2A14E`).
pub const HOT_IMG: usize = 0x9188;
/// Pending-keystroke flag `[0x9CCB]` / code cell `[0x9CCC]` (file
/// `0x2A19C`).
pub const KEY_PEND: usize = 0x9CCB;
/// The latched scancode/synthetic code `[0x9CCC]` (file `0x2A1A1`).
pub const KEY_CODE: usize = 0x9CCC;
/// `K` keyboard-emulation flag `[0x9CCD]` (file `0x2A18E`).
pub const K_MODE: usize = 0x9CCD;
/// Tandy extra-nav flag `[0x9CC6]` (file `0x2A225`).
pub const TANDY: usize = 0x9CC6;
/// Menu suppress/drag flag `[0x91D5]` (file `0x2A112`).
pub const DRAG: usize = 0x91D5;
/// Selection stash on loop exit `[0xA81A]` (file `0x33A41`).
pub const SEL_SAVE: usize = 0xA81A;
/// Machine-type table base `ds:0x9B12`, stride `0x30` (file `0x33A78`).
pub const TYPE_BASE: usize = 0x9B12;
/// Machine-type record stride (file `0x33A5E`).
pub const TYPE_STRIDE: usize = 0x30;
/// Dirty-flag service snapshots `[0x9CBC]`/`[0x9CBE]` (file
/// `0x33C70`/`0x33C76`).
pub const SNAP0: usize = 0x9CBC;
/// Snapshot of `DIRTY1` (file `0x33C76`).
pub const SNAP1: usize = 0x9CBE;
