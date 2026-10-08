//! Field offsets inside one `0xC`-byte fleet/ship record — the array at
//! `ds:0x9991` (`0x17` entries) scanned by the defence pass
//! (file `0x345DA`).

/// Attack strength — summed into `F_ATTACK`, weighted into `F_DEFENCE`.
pub const S_POW: usize = 0x00;
/// Docked-at record pointer — a planet/machine `ds:` offset, `0` when
/// the slot is free (file `0x3447B`).
pub const S_LINK: usize = 0x02;
/// Second status word, cleared with the link (file `0x34623`).
pub const S_UNK4: usize = 0x04;
/// Flag byte — `0x2` = armed/present in the defence scan, `0x8` exempts
/// a ship from the reinforcement crew bonus (file `0x3460C`/`0x348C0`).
pub const S_FLAGS: usize = 0x06;
/// Crew/experience byte — weights defence as `pow·crew/0x48` and gains
/// `+7` per reinforcement under `0x5D` (file `0x344B1`/`0x348F7`).
pub const S_CREW: usize = 0x07;
/// Weapons load — feeds the defence weighting (file `0x344E9`).
pub const S_GUNS: usize = 0x08;
/// Fuel/load — feeds the defence weighting (file `0x344EC`).
pub const S_LOAD: usize = 0x0A;
