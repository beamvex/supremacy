//! Field offsets inside one `0x28`-byte machine/station record — the
//! array at `ds:0x9491` (`0x20` entries) ticked by `cs:0x73D9`
//! (file `0x37416`).

/// Build/repair countdown — decremented while nonzero, sets dirty bit
/// `0x800` (file `0x37433`).
pub const M_TIMER: usize = 0x0C;
/// Mining ops remaining — when `0` the mining flag clears and the
/// machine relinks to `[M_LINK]+0x28` (file `0x37537`–`0x3759A`).
pub const M_OPS: usize = 0x18;
/// Produce accumulator — counts toward the `0x4B0` harvest threshold
/// the `cs:0x7C30` event op waits for (file `0x37C42`).
pub const M_ACC: usize = 0x0A;
/// Linked record pointer — resolved via the machine-type table when
/// mining completes (file `0x37591`).
pub const M_LINK: usize = 0x1A;
/// Machine type — `3` = solar satellite (energy), `6` = core-mining
/// station (minerals+fuel), `7` = horticultural station (food, burns
/// energy) (file `0x37446`/`0x374B9`/`0x37506`).
pub const M_TYPE: usize = 0x1E;
/// Flag byte — `0x1` countdown active, `0x4` mining mode, `0x8` mining
/// running, `0x20` online/powered (file `0x3743D`–`0x374C3`).
pub const M_FLAGS: usize = 0x1F;
/// Host planet record index — `call 0x5085` resolves `si` to
/// `planet[M_HOST]` (file `0x35085`–`0x350A8`).
pub const M_HOST: usize = 0x21;
/// Construction countdown — at `0` fires the colonise completion
/// (file `0x375AB`–`0x375C1`).
pub const M_BUILD: usize = 0x22;
/// Mineral deposit this machine can still extract (file `0x37555`).
pub const M_DEPOSIT: usize = 0x26;

/// Solar satellite.
pub const TYPE_SOLAR: u8 = 3;
/// Core-mining station.
pub const TYPE_MINER: u8 = 6;
/// Horticultural station.
pub const TYPE_FARM: u8 = 7;

// Machine-type preset table — `ds:0x9B12`, stride `0x30`; `[0x9154]`
// holds the selected entry. Fields recovered from the machine-spawn
// copy (file `0x36590`/`0x38085`) and the purchase-cost deduction
// (file `0x365BE`–`0x365ED`).
/// Credits cost — deducted from the faction `+0x36`/`+0x38` dword.
pub const T_CREDITS: usize = 0x06;
/// Build/timer seed — copied to machine `M_TIMER` (`+0x0C`).
pub const T_BUILD: usize = 0x08;
/// Copied to machine `+0x14`.
pub const T_14: usize = 0x0A;
/// Deposit/capacity seed — scaled by `cs:0xE862` into machine
/// `M_DEPOSIT`/`+0x26`.
pub const T_DEPOSIT: usize = 0x0C;
/// Energy cost — deducted from faction `+0x34` (difficulty `≠0` only).
pub const T_ENERGY: usize = 0x0E;
/// Minerals cost — deducted from faction `+0x30` (difficulty `≥2`
/// only).
pub const T_MINERALS: usize = 0x10;
/// Type id — copied to machine `M_TYPE`; the value (`3`/`4`/`6`/`7`)
/// also drives the periodic-SFX selector (file `0x339A9`–`0x339EF`,
/// reads `[0x9154]+0x12`, not the planet record).
pub const T_TYPE: usize = 0x12;
/// Aux/rate byte — copied to machine `+0x20`; multiplied by the type id
/// for the display rate (file `0x33C3B`, again via `[0x9154]`).
pub const T_RATE: usize = 0x13;
