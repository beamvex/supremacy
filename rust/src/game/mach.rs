//! Field offsets inside one `0x28`-byte machine/station record — the
//! array at `ds:0x9491` (`0x20` entries) ticked by `cs:0x73D9`
//! (file `0x37416`).

/// Build/repair countdown — decremented while nonzero, sets dirty bit
/// `0x800` (file `0x37433`).
pub const M_TIMER: usize = 0x0C;
/// Mining ops remaining — when `0` the mining flag clears and the
/// machine relinks to `[M_LINK]+0x28` (file `0x37537`–`0x3759A`).
pub const M_OPS: usize = 0x18;
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
