//! Field offsets inside one `0x3A`-byte planet/faction record, recovered
//! from the new-game init (file `0x330A0`–`0x331C2`) and the record
//! accessor `cs:0x5E3A` (`idx·0x3A + [0x9158]`).

/// Word zeroed for every record at init (file `0x331BA`).
pub const F_WORD0: usize = 0x00;
/// 8-byte display name + `0x21` terminator at `+0x16` — init writes
/// `LIFELESS!` (file `0x3315C`–`0x33170`).
pub const F_NAME: usize = 0x0E;
/// Word cleared in the player record during init (file `0x330F6`).
pub const F_22: usize = 0x22;
/// Owner byte — `2` = the player (file `0x330FB`/`0x3314B`).
pub const F_OWNER: usize = 0x24;
/// Difficulty-scaled parameter — preset picks `0x1F77`/`0x3A2F`/`0x5811`
/// for `[0x91E4]` = 0/1/2 (file `0x330A4`–`0x330CE`).
pub const F_DIFF: usize = 0x26;
/// Planet serial index stamped at init (file `0x33185`).
pub const F_SERIAL: usize = 0x28;
/// Byte cleared with the serial (file `0x33188`).
pub const F_29: usize = 0x29;
/// Stock field — one of population/minerals/fuel/energy/food; exact
/// mapping pending the SUPCHT field offsets (init: `0x5DC + rand(0x1F4)`
/// for the faction record, file `0x3313F`).
pub const F_STOCK0: usize = 0x2A;
/// Stock field (file `0x33127`).
pub const F_STOCK1: usize = 0x2C;
/// Stock field (file `0x3312C`).
pub const F_STOCK2: usize = 0x30;
/// Stock field (file `0x33131`).
pub const F_STOCK3: usize = 0x32;
/// Stock field (file `0x33136`).
pub const F_STOCK4: usize = 0x34;
/// Credits — 32-bit `dword` at `+0x36`/`+0x38` (file `0x3310A`–`0x33117`:
/// `0xC350 + rand(0x4E20)` ≈ 50,000–70,000 for a new faction).
pub const F_CREDITS: usize = 0x36;
