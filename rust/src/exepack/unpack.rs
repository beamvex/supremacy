use super::commands::run;
use super::mz::Mz;
use super::reloc_list::reloc_list;
use super::scan::stream_end;
use super::{Error, Header, Unpacked};

/// Unpack an EXEPACK-compressed MZ executable.
///
/// Reproduces the stub's memory model in a flat buffer: `dest` is
/// `dest_len` paragraphs seeded with the packed load image (the program's
/// uncompressed head plus the compressed tail), then command blocks expand
/// the tail backwards from the top. The returned fixup list and real
/// `cs:ip`/`ss:sp` come from the block header — the values the stub passes
/// to the far jump at `0x23B76`.
///
/// # Errors
/// `BadMz` (no `MZ`/`ZM` or truncated header), `MissingPack` (no `RB`
/// header at the entry `CS`), `Corrupt` (bad command byte or a read/write
/// outside the image).
pub fn unpack(file: &[u8]) -> Result<Unpacked, Error> {
    let mz = Mz::parse(file)?;
    let img = file.get(mz.image.clone()).ok_or(Error::BadMz)?;
    let blk_ofs = usize::from(mz.entry_cs) * 16;
    let block = img.get(blk_ofs..).ok_or(Error::MissingPack)?;
    let hdr = Header::parse(block)?;
    let mut dest = vec![0u8; usize::from(hdr.dest_len) * 16];
    let n = dest.len().min(img.len());
    dest[..n].copy_from_slice(&img[..n]);
    let si = stream_end(img, blk_ofs)?;
    let di = dest.len() - 1;
    run(&mut dest, si, di)?;
    Ok(Unpacked {
        relocs: reloc_list(block),
        image: dest,
        cs: hdr.real_cs,
        ip: hdr.real_ip,
        ss: hdr.real_ss,
        sp: hdr.real_sp,
    })
}
