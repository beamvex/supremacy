//! Music streams — the PC-speaker (module `low:0x0000`) and Tandy
//! (`low:0x0D50`) channel byte streams → `pctab`/`ttab` matches mapping
//! every event address to `(Ev, next)` — the `cs:0xF3D`/`0x1C02`
//! decode loops compiled.
//!
//! Reachable streams: the `cs:0x6EB`/`cs:0x93E` song tables' channel
//! pointer lists (0-terminated word arrays) plus `op_list` targets,
//! followed transitively.

use std::collections::{BTreeMap, BTreeSet};

use super::blob::{b16, Blobs};
use super::out::Out;

const PC_SONG: u16 = 0x6EB;
const T_SONG: u16 = 0x93E;
const T_BASE: usize = 0xD50;
const SONGS: u16 = 11;
const CAP: usize = 0x800;

/// `al` → `(len, PcEv)`.
fn pc_ev(al: u8, rest: &[u8]) -> (u16, String) {
    if al < 0x80 {
        return (1, format!("Note({al:#04X})"));
    }
    if al < 0xA0 {
        let (l, s) = match al {
            0x80 => (1, "Mute".into()),
            0x81 => (1, "Flags(0x00)".into()),
            0x82 => (
                4,
                format!("Sweep {{ dt: {:#06X}, n: {:#04X} }}", w(rest, 0), rest[2]),
            ),
            0x83 => (1, "Flags(0xC0)".into()),
            0x84 => (1, "Flags(0x40)".into()),
            0x85 => (1, "Next".into()),
            0x86 => (
                4,
                format!("Trem {{ dep: {:#04X}, cur: {:#04X} }}", rest[0], rest[2]),
            ),
            0x87 => (1, "End".into()),
            0x88 => (2, format!("Trans({:#04X})", rest[0])),
            0x89 => (1, "Flags(0x02)".into()),
            0x8A => (2, format!("Mux({:#04X})", rest[0])),
            0x8B => (1, "Hold".into()),
            0x8C => (2, format!("Gtrans({:#04X})", rest[0])),
            0x8D => (3, format!("List({:#06X})", w(rest, 0))),
            0x8E => (2, format!("Tempo({:#04X})", rest[0])),
            0x8F => (1, "RotMux".into()),
            _ => (1, "Wild".into()),
        };
        return (l, s);
    }
    if al >= 0xE0 {
        return (1, format!("Reload({:#04X})", al.wrapping_add(0x21)));
    }
    (1, format!("Env({:#04X})", al.wrapping_add(0x20)))
}

/// `al` → `(len, Tev)`.
fn t_ev(al: u8, rest: &[u8]) -> (u16, String) {
    if al < 0x80 {
        return (1, format!("Note({al:#04X})"));
    }
    if al < 0xA0 {
        let (l, s) = match al {
            0x80 => (1, "Mute".into()),
            0x81 => (2, format!("Tsel({:#04X})", rest[0])),
            0x82 => (
                4,
                format!("Sweep {{ dt: {:#06X}, n: {:#04X} }}", w(rest, 0), rest[2]),
            ),
            0x83 => (1, "Flags(0xC0)".into()),
            0x84 => (1, "Flags(0x40)".into()),
            0x85 => (1, "Next".into()),
            0x86 => (1, "End".into()),
            0x87 => (2, format!("Gtrans({:#04X})", rest[0])),
            0x88 => (1, "Hold".into()),
            0x89 => (3, format!("Nadd({:#06X})", w(rest, 0))),
            0x8A => (2, format!("Trans({:#04X})", rest[0])),
            0x8B => (3, format!("List({:#06X})", w(rest, 0))),
            0x8C => (2, format!("Tempo({:#04X})", rest[0])),
            0x8D => (2, format!("Mvol({:#04X})", rest[0])),
            _ => (1, "Wild".into()),
        };
        return (l, s);
    }
    if al >= 0xE0 {
        return (1, format!("Reload({:#04X})", al.wrapping_add(0x21)));
    }
    if al >= 0xC0 {
        return (1, format!("Esel({:#04X})", al - 0xC0));
    }
    (1, format!("Inst({:#04X})", al - 0xA0))
}

/// `u16` operand at `rest[ofs]`.
fn w(rest: &[u8], ofs: usize) -> u16 {
    u16::from(rest[ofs]) | u16::from(rest[ofs + 1]) << 8
}

/// Decode `heads` streams inside `img` (module-relative offsets).
/// `end` names the end-marker variant. A head that runs past [`CAP`]
/// events without ending is a stale pointer — its arms are dropped.
fn decode(
    img: &[u8],
    heads: BTreeSet<u16>,
    end: &str,
    mut ev: impl FnMut(u8, &[u8]) -> (u16, String),
) -> (BTreeMap<u16, (String, u16)>, BTreeSet<u16>) {
    let mut out = BTreeMap::new();
    let mut clean = BTreeSet::new();
    for h in heads {
        let mut si = h;
        let mut mine = Vec::new();
        let mut ok = false;
        for _ in 0..CAP {
            if out.contains_key(&si) {
                ok = true;
                break;
            }
            let a = usize::from(si);
            let al = img.get(a).copied().unwrap_or(0);
            let (len, e) = ev(al, img.get(a + 1..).unwrap_or(&[]));
            out.insert(si, (e.clone(), si.wrapping_add(len)));
            mine.push(si);
            si = si.wrapping_add(len);
            if e == end {
                ok = true;
                break;
            }
        }
        if !ok {
            for m in &mine {
                out.remove(m);
            }
            eprintln!("songs: stream {h:#06X} overruns — dropped");
        } else {
            clean.insert(h);
        }
    }
    (out, clean)
}

/// Stream heads from a song table: `{tempo, list…}` records, the
/// 0-terminated word lists they point at, then every `List` operand
/// found in `tab` (transitively).
fn heads(img: &[u8], tab: u16, stride: u16, chans: u16) -> BTreeSet<u16> {
    let mut lists = BTreeSet::new();
    for i in 0..SONGS {
        for c in 0..chans {
            lists.insert(b16(img, tab + i * stride + 1 + c * 2));
        }
    }
    list_heads(img, &lists)
}

/// Every word in each 0-terminated list.
fn list_heads(img: &[u8], lists: &BTreeSet<u16>) -> BTreeSet<u16> {
    let mut out = BTreeSet::new();
    for &l in lists {
        for j in 0..0x80u16 {
            let v = b16(img, l + j * 2);
            if v == 0 {
                break;
            }
            out.insert(v);
        }
    }
    out
}

/// `List(a)` operand addrs in `arms` → their stream heads.
fn op_lists(img: &[u8], arms: &[String]) -> BTreeSet<u16> {
    let mut lists = BTreeSet::new();
    for e in arms {
        if let Some(a) = e.strip_prefix("List(").and_then(|s| s.strip_suffix(')')) {
            lists.insert(u16::from_str_radix(&a[2..], 16).unwrap_or(0));
        }
    }
    list_heads(img, &lists)
}

/// Decode with `op_list` reachability: decode `heads`, then any
/// still-undecoded stream the `List` operands point at, repeatedly.
fn all(
    img: &[u8],
    heads: BTreeSet<u16>,
    end: &str,
    ev: impl FnMut(u8, &[u8]) -> (u16, String) + Copy,
) -> BTreeMap<u16, (String, u16)> {
    let mut seen = BTreeSet::new();
    let mut work = heads;
    let mut tab = BTreeMap::new();
    loop {
        let fresh: BTreeSet<u16> = work.difference(&seen).copied().collect();
        if fresh.is_empty() {
            return tab;
        }
        seen.extend(&fresh);
        let (t, clean) = decode(img, fresh, end, ev);
        let arms: Vec<String> = clean
            .iter()
            .flat_map(|h| t.range(h..).map(|x| x.1 .0.clone()))
            .collect();
        work = op_lists(img, &arms);
        tab.extend(t);
    }
}

/// Emit a tab module: `mod.rs` (enum + `at`) + `a*.rs` chunks.
fn tab(o: &mut Out, dir: &str, ename: &str, doc: &str, t: &BTreeMap<u16, (String, u16)>) {
    let arms: Vec<(u16, String)> = t
        .iter()
        .map(|(a, (e, n))| {
            (
                *a,
                format!("        {a:#06X} => Some(({ename}::{e}, {n:#06X})),\n"),
            )
        })
        .collect();
    let mut mods = String::new();
    let mut calls = String::new();
    for (i, c) in arms.chunks(170).enumerate() {
        mods += &format!("mod a{i};\n");
        calls += &format!(
            "        {:#06X}..={:#06X} => a{i}::f(a),\n",
            c[0].0,
            arms.get((i + 1) * 170)
                .map_or(0xFFFF, |x| u32::from(x.0 - 1))
        );
        let body = c.iter().map(|x| x.1.as_str()).collect::<String>();
        o.file(&format!("{dir}/a{i}.rs"), &format!("//! @generated by `xgen` — {ename} chunk {i}.\n\nuse super::{ename};\n\npub(super) fn f(a: u16) -> Option<({ename}, u16)> {{\n    match a {{\n{body}        _ => None,\n    }}\n}}\n"));
    }
    // The range arms are contiguous to `0xFFFF`; when they also start
    // at `0x0000` the `_` arm is unreachable and needs the allow.
    let attr = if arms.first().is_some_and(|x| x.0 == 0) {
        "#[allow(unreachable_patterns)]\n"
    } else {
        ""
    };
    let m = format!("//! @generated by `xgen` — compiled channel stream events.\n\n/// One stream event (`(ev, next)` per address).\n#[derive(Clone, Copy, PartialEq, Eq, Debug)]\npub enum {ename} {{\n{doc}}}\n\n{mods}\n/// The event at stream address `a` and the next cursor.\n{attr}pub fn at(a: u16) -> Option<({ename}, u16)> {{\n    match a {{\n{calls}        _ => None,\n    }}\n}}\n");
    o.file(&format!("{dir}/mod.rs"), &m);
}

/// Emit `pctab` and `ttab`.
pub fn emit(b: &Blobs, o: &mut Out) {
    let pc = all(&b.low, heads(&b.low, PC_SONG, 5, 2), "End", pc_ev);
    tab(o, "audio/pctab", "PcEv", PC_DOC, &pc);
    let timg = &b.low[T_BASE..];
    let t = all(timg, heads(timg, T_SONG, 9, 4), "End", t_ev);
    tab(o, "audio/ttab", "Tev", T_DOC, &t);
}

const PC_DOC: &str = "    /// note byte `< 0x80` — ends the step\n    Note(u8),\n    /// `0x80` — fade 0 + hold tail\n    Mute,\n    /// `0x81`/`0x83`/`0x84`/`0x89` — flag set/or\n    Flags(u8),\n    /// `0x82` — sweep delta + count\n    Sweep { dt: u16, n: u8 },\n    /// `0x85` — advance the stream list\n    Next,\n    /// `0x86` — tremolo `{dep, pad, cur}`\n    Trem { dep: u8, cur: u8 },\n    /// `0x87` — song end\n    End,\n    /// `0x88` — channel transpose\n    Trans(u8),\n    /// `0x8A` — fixed mux byte\n    Mux(u8),\n    /// `0x8B` — wait reload + suspend\n    Hold,\n    /// `0x8C` — global transpose\n    Gtrans(u8),\n    /// `0x8D` — switch stream list\n    List(u16),\n    /// `0x8E` — tempo byte\n    Tempo(u8),\n    /// `0x8F` — rotate mux mode\n    RotMux,\n    /// `0x90..0x9F` — off-table; the port stops\n    Wild,\n    /// `0xE0..` — wait reload\n    Reload(u8),\n    /// `0xA0..0xDF` — envelope index `al + 0x20`\n    Env(u8),\n";

const T_DOC: &str = "    /// note byte `< 0x80` — ends the step\n    Note(u8),\n    /// `0x80` — envelope freeze + tail\n    Mute,\n    /// `0x81` — tremolo selector\n    Tsel(u8),\n    /// `0x82` — sweep delta + count\n    Sweep { dt: u16, n: u8 },\n    /// `0x83`/`0x84` — slide flags\n    Flags(u8),\n    /// `0x85` — advance the stream list\n    Next,\n    /// `0x86` — song end (ends the tick)\n    End,\n    /// `0x87` — global transpose\n    Gtrans(u8),\n    /// `0x88` — tail + suspend\n    Hold,\n    /// `0x89` — `run` reload word\n    Nadd(u16),\n    /// `0x8A` — channel transpose\n    Trans(u8),\n    /// `0x8B` — switch stream list\n    List(u16),\n    /// `0x8C` — tempo byte\n    Tempo(u8),\n    /// `0x8D` — master volume\n    Mvol(u8),\n    /// `0x8E..0x9F` — off-table; the port stops\n    Wild,\n    /// `0xE0..` — wait reload\n    Reload(u8),\n    /// `0xC0..0xDF` — envelope selector\n    Esel(u8),\n    /// `0xA0..0xBF` — vibrato index\n    Inst(u8),\n";
