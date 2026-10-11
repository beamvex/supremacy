//! The resident `AdLib`/Roland int-80 driver — the module at file
//! `0x2900` (image `0x1D00`, installed at `[0x156] = 0x1D0`).
//!
//! Unlike the P/T trackers this module is C-compiled: its exports are
//! the eight-entry `jmp` table at `cs:0` (the game's far-call table at
//! `cs:0x126C` lists six of them). Music/SFX records live in the song
//! header `ds`-space copied in by export 3; the event queue feeds the
//! eight channel records at `cs:0x1A00` (`AChan`, `0x20` bytes each),
//! which the apply/dispatch pair turns into MPU-401 MIDI writes or
//! OPL2 register writes.
use super::ar_chan::AChan;
use super::Ports;

/// The `AdLib`/Roland driver state — see module docs.
pub struct Ar<P> {
    /// Module bytes — `cs:` reads `map` to file `0x2900 + ofs`; the
    /// slice runs to `0x4970` where the embedded song records begin.
    pub(super) img: Box<[u8]>,
    /// Port bus — OPL `0x388`/`0x389`, MPU `0x330`/`0x331`, PIT
    /// `0x43`/`0x40` writes.
    pub ports: P,
    /// `[0x19AC]` — `0xFF` uninstalled, `0` `AdLib`, `1` Roland.
    pub(super) mode: u8,
    /// `[0x1C05]` — the music enable flag (`en`).
    pub(super) en: u8,
    /// `[0x1C02]` — the MPU's last ack byte.
    pub(super) ack: u8,
    /// `[0x1C85]` — the installed-timer flag.
    pub(super) timer: bool,
    /// `[0x19F9]` — `fetch` cadence in ticks (tempo, default 6).
    pub(super) div: u8,
    /// `[0x19FD]` — the cadence counter.
    pub(super) cnt: u8,
    /// `[0x19FF]` — queue fetches this sound (`0x40` wraps).
    pub(super) pass: u8,
    /// `[0x19FA]` — the sound index into `hdr`'s `map`.
    pub(super) idx: u8,
    /// `[0x1FD2]` — highest nonzero `map` index + 1 (`nmap`).
    pub(super) nmap: u16,
    /// `[0x19FB]` — the event-queue cursor (record-relative offset).
    pub(super) cur: u16,
    /// `[0x1C81]` — the queue region's record-relative paragraph —
    /// `tbl[maxmap + 1]` where `maxmap` is the highest `map` byte
    /// (`0x236A`'s `dl` scan; the asm adds the `[0x1C03]` record
    /// segment, which is the `song` slice's base here).
    pub(super) qbase: u16,
    /// `[0x1FCE]` — queue-region offset of the `timbre` records
    /// (`0x122 + 0x100·nprogs`).
    pub(super) qend: u16,
    /// `[0x19FE]` — the force-`advance` flag (kind `0xB`/`0xD`).
    pub(super) rst: u8,
    /// `[0x2060]` — fade armed flag.
    pub(super) fade: u8,
    /// `[0x205C]` — fade rate (doubled on `AdLib`).
    pub(super) frate: u8,
    /// `[0x205E]` — fade tick divider (2).
    pub(super) fcnt: u8,
    /// `[0x2058]` — master music volume (`0x64`/`0x7F` `init`).
    pub(super) mvol: u8,
    /// `[0x2062]` — the `init`-time default `mvol`.
    pub(super) dvol: u8,
    /// `[0x2056]` — `mvol` cached for the `AdLib` `vol_apply` clamp.
    pub(super) wvol: u8,
    /// `[0x1BDB]` — 16-byte `[0x40]`-domain volume cache (dedup).
    pub(super) volc: [u8; 16],
    /// The `0x118`-byte song header (`cs:0x1673`) loaded by export 3:
    /// `[0x18..]` `map`, `[0xD9]` loop point, `[0xD8]` channel count,
    /// `[0x98..]` 0x20 `progmap` `{prog, vol}` pairs.
    pub(super) hdr: [u8; 0x118],
    /// The `0x200`-byte sound table (`cs:0x178C`): per-sound
    /// record-relative paragraphs (word each).
    pub(super) tbl: [u8; 0x200],
    /// The song record image E3 copied `ds:si` from (kept whole —
    /// streams + timbres are read through it on demand).
    pub(super) song: Vec<u8>,
    /// The `0x8E`-byte `cs:` staging area `[0x1C1D..0x1CAB]` — Roland
    /// setup/`timbre` buffers plus the `AdLib` scratch cells (`[0x1C86..]`
    /// rhythm flags, `[0x1C90..]` patch-`select` params, `[0x1C98]`
    /// rhythm mask, `[0x1C99..]` perc masks + `init` patch data).
    pub(super) stage: [u8; 0x8E],
    /// `cs:0x1BED` — the percussion `SysEx` block (kinds 7/8 patch the
    /// `+0x0A..+0x0C` body bytes then `0x28C8` it out).
    pub(super) sfxbuf: [u8; 0x16],
    /// `cs:0x1FF8` — the all-notes-off `SysEx` template; `0x1FE0` copies
    /// the `0x200B + (nchans-1)·8` `map` block into `+0x0A` first.
    pub(super) anobuf: [u8; 0x16],
    /// `cs:0x204B` — the master-volume `SysEx` block (`+0x0A` = level).
    pub(super) volbuf: [u8; 0x16],
    /// `cs:0x1E20` — the 30-byte patch staging record (two 13-byte
    /// operators + levels/flag tail).
    pub(super) patch: [u8; 30],
    /// `cs:0x1E60` — `freq_write`'s per-channel note cache (18).
    pub(super) notec: [u8; 18],
    /// `cs:0x1E72` — the per-channel key-bit cache (18).
    pub(super) keyc: [u8; 18],
    /// `cs:0x1E84` — per-slot volume scale (the `vol_apply` store).
    pub(super) stot: [u8; 18],
    /// `cs:0x1E96` — 18 16-byte operator slot records (`0x301B`
    /// staging: 13 patch bytes + wave byte at `+0x0D`).
    pub(super) slot: [[u8; 16]; 18],
    /// `[0x1FD0]` — the record index `apply`/`service` last set
    /// (`freq_write` writes `pitch`/`block`/`fnum` through it).
    pub(super) chan: usize,
    /// The eight `0x20`-byte records, plus a scratch ninth absorbing
    /// the assembly's stale `[0x1FD0]` writes after out-of-range
    /// lookups (the module writes through whatever `si` it kept).
    pub(super) chans: [AChan; 9],
}

impl<P: Ports> Ar<P> {
    /// Cold state — `mode = 0xFF` (uninstalled), `div = 6`, the
    /// `cs:` staging/buffer blocks copied out of the shipped module
    /// (`img` is the unpacked exe; `file 0x2900..0x4970` is kept).
    #[must_use]
    pub fn new(ports: P, img: &[u8]) -> Self {
        let img: Box<[u8]> = img.get(0x2900..0x4970).unwrap_or(&[]).into();
        let mut s = Self {
            img,
            ports,
            mode: 0xFF,
            en: 0,
            ack: 0,
            timer: false,
            div: 6,
            cnt: 0,
            pass: 0,
            idx: 0,
            nmap: 0,
            cur: 0x118,
            qbase: 0,
            qend: 0,
            rst: 0,
            fade: 0,
            frate: 0,
            fcnt: 0,
            mvol: 0,
            dvol: 0,
            wvol: 0,
            volc: [0; 16],
            hdr: [0; 0x118],
            tbl: [0; 0x200],
            song: Vec::new(),
            stage: [0; 0x8E],
            sfxbuf: [0; 0x16],
            anobuf: [0; 0x16],
            volbuf: [0; 0x16],
            patch: [0; 30],
            notec: [0; 18],
            keyc: [0; 18],
            stot: [0; 18],
            slot: [[0; 16]; 18],
            chan: 8,
            chans: [AChan::default(); 9],
        };
        if let Some(src) = s.img.get(0x1C1D..0x1C1D + 0x8E) {
            s.stage.copy_from_slice(src);
        }
        if let Some(src) = s.img.get(0x1BED..0x1BED + 0x16) {
            s.sfxbuf.copy_from_slice(src);
        }
        if let Some(src) = s.img.get(0x1FF8..0x1FF8 + 0x16) {
            s.anobuf.copy_from_slice(src);
        }
        if let Some(src) = s.img.get(0x204B..0x204B + 0x16) {
            s.volbuf.copy_from_slice(src);
        }
        s
    }
}
