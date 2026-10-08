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
/// `ds:` slot of the per-difficulty countdown (`[0x9164]`: 6/14/30).
pub const DIFF_PACE: usize = 0x9164;
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
