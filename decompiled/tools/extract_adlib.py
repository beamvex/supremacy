#!/usr/bin/env python3
"""Extract the embedded AdLib/MT-32 song records from GAME.unpacked.exe (GAM-30).

Each song record starts with the tag B4 9A 01 and carries a 0x118 header,
a 0x200 paragraph-offset table, per-sound data blocks, a 0x100-stride
MT-32 timbre table and a 0x40-stride AdLib FM patch table.  See
decompiled/FUNCTIONS.md ("AdLib/Roland driver module") for the layout.
"""
import hashlib
import json
import struct
import sys
from pathlib import Path

TAG = b'\xb4\x9a\x01'
HDR_LEN, TBL_LEN = 0x118, 0x200
TIMBRE_STRIDE, FM_STRIDE = 0x100, 0x40
PRE_TAG_PAD = 8
SONG_DATA_END = 0x1074E  # first byte after the last song's usable area


def sha256(b: bytes) -> str:
    return hashlib.sha256(b).hexdigest()


def song_tags(data: bytes) -> list[int]:
    """All B4 9A 01 song-record offsets inside the driver module image."""
    out, i = [], data.find(TAG, 0x2900)
    while i != -1:
        out.append(i)
        i = data.find(TAG, i + 1)
    return out


def name_field(raw: bytes) -> str:
    """Instrument name from a NUL/space-padded field."""
    return raw.split(b'\x00')[0].rstrip(b' ').decode('ascii', 'replace')


def is_named(raw: bytes) -> bool:
    """True when raw looks like a printable >=2-char instrument name."""
    txt = name_field(raw)
    return len(txt) >= 2 and all(b == 0 or 0x20 <= b < 0x7F for b in raw)


def find_fm_start(data: bytes, timbre_start: int, end: int) -> int:
    """FM table starts on the 0x100 grid at the first +0-named slot."""
    off = timbre_start
    while off + TIMBRE_STRIDE <= end:
        if data[off] != 0x20 and is_named(data[off:off + 0x0A]):
            return off
        off += TIMBRE_STRIDE
    return end


def fm_end(data: bytes, start: int, limit: int) -> int:
    """Last 0x40 slot fully inside [start, limit)."""
    off = start
    while off + FM_STRIDE <= limit:
        off += FM_STRIDE
    return off


def timbre_recs(data: bytes, start: int, end: int) -> list[dict]:
    recs = []
    for i, off in enumerate(range(start, end, TIMBRE_STRIDE)):
        raw = data[off + 8:off + 0x12]
        recs.append(dict(idx=i, off=off, kind='timbre', type=data[off],
                         name=name_field(raw) if is_named(raw) else None))
    return recs


def fm_recs(data: bytes, start: int, end: int) -> list[dict]:
    recs = []
    for i, off in enumerate(range(start, end, FM_STRIDE)):
        raw = data[off:off + 0x0A]
        recs.append(dict(idx=i, off=off, kind='fm',
                         name=name_field(raw) if is_named(raw) else None))
    return recs


def block_map(hdr: bytes) -> list[int]:
    """Sound-index -> block-index map, trimmed after the highest entry."""
    mx = max(hdr[0x18:0x98])
    return list(hdr[0x18:0x18 + mx + 1])


def song_end(data: bytes, tag: int, tags: list[int]) -> int:
    nxt = [t for t in tags if t > tag]
    return min(nxt) if nxt else SONG_DATA_END


def write_blob(data: bytes, out: Path, fn: str, lo: int, hi: int) -> dict:
    out.write_bytes(data[lo:hi])
    return dict(file=fn, off=lo, len=hi - lo, sha256=sha256(data[lo:hi]))


def write_patches(data: bytes, root: Path, recs: list[dict]) -> None:
    for r in recs:
        if not r['name']:
            continue
        size = TIMBRE_STRIDE if r['kind'] == 'timbre' else FM_STRIDE
        fn = f"{r['idx']:02}_{r['name'].replace(' ', '_')}_{r['kind']}.bin"
        (root / fn).write_bytes(data[r['off']:r['off'] + size])


def song_info(data: bytes, tag: int, end: int, fallback: str) -> dict:
    name = name_field(data[tag + 3:tag + 0x18]) or fallback
    bmap = block_map(data[tag:tag + HDR_LEN])
    table = struct.unpack_from(f'<{TBL_LEN // 2}H', data, tag + HDR_LEN)
    ibase = tag + table[len(bmap)] * 16
    tstart, fstart = ibase + HDR_LEN, 0
    fstart = find_fm_start(data, tstart, end - PRE_TAG_PAD)
    fend = fm_end(data, fstart, end - PRE_TAG_PAD)
    return dict(name=name, tag=tag, end=end, bmap=bmap, table=table,
                ibase=ibase, tstart=tstart, fstart=fstart, fend=fend)


def extract_song(data: bytes, info: dict, root: Path) -> dict:
    n = info['name']
    files = [write_blob(data, root / f'{n}.bin', f'{n}.bin', info['tag'], info['end']),
             write_blob(data, root / f'{n}.hdr.bin', f'{n}.hdr.bin', info['tag'], info['tag'] + HDR_LEN),
             write_blob(data, root / f'{n}.table.bin', f'{n}.table.bin', info['tag'] + HDR_LEN, info['tag'] + 0x318),
             write_blob(data, root / f'{n}.blocks.bin', f'{n}.blocks.bin', info['tag'] + 0x318, info['ibase']),
             write_blob(data, root / f'{n}.timbres.bin', f'{n}.timbres.bin', info['tstart'], info['fstart']),
             write_blob(data, root / f'{n}.fm.bin', f'{n}.fm.bin', info['fstart'], info['fend'])]
    pdir = root / 'patches' / n
    pdir.mkdir(parents=True, exist_ok=True)
    timbres = timbre_recs(data, info['tstart'], info['fstart'])
    fm = fm_recs(data, info['fstart'], info['fend'])
    write_patches(data, pdir, timbres + fm)
    return dict(name=n, tag=info['tag'], end=info['end'], files=files,
                block_map=info['bmap'],
                para_table=list(info['table'][:len(info['bmap']) + 1]),
                blocks=[info['tag'] + info['table'][i] * 16 for i in range(len(info['bmap']))],
                instrument_base=info['ibase'], timbre_start=info['tstart'],
                fm_start=info['fstart'], fm_end=info['fend'],
                timbres=timbres, fm=fm)


def main() -> None:
    src = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).parents[1] / 'GAME.unpacked.exe'
    outdir = Path(sys.argv[2]) if len(sys.argv) > 2 else Path(__file__).parents[1] / 'fixtures' / 'adlib'
    data = src.read_bytes()
    outdir.mkdir(parents=True, exist_ok=True)
    tags = song_tags(data)
    assert len(tags) == 4, f'expected 4 song tags, got {tags!r}'
    songs = [extract_song(data, song_info(data, t, song_end(data, t, tags), f'song{i}'), outdir)
             for i, t in enumerate(tags)]
    manifest = dict(source=src.name, source_sha256=sha256(data),
                    module_base=0x2900, tag=list(TAG), songs=songs)
    (outdir / 'manifest.json').write_text(json.dumps(manifest, indent=1))
    named = sum(sum(1 for r in s['timbres'] + s['fm'] if r['name']) for s in songs)
    print(f'wrote {len(songs)} songs, {named} named instruments -> {outdir}')


if __name__ == '__main__':
    main()
