use super::cmd::Cmd;
use super::word::word;
use super::Error;

/// Read one command block at `si` going backwards (`DF=1`): byte `[si]` is
/// `cmd`, `[si-2]` is `len`, and a `0xB0` fill block carries its byte at
/// `[si-3]`; a `0xB2` copy's sources sit below that. Returns the command and
/// the next block's `si`.
pub(super) fn read_cmd(buf: &[u8], si: usize) -> Result<(Cmd, usize), Error> {
    let cmd = *buf.get(si).ok_or(Error::Corrupt(0))?;
    let at = |k: usize| si.checked_sub(k).ok_or(Error::Corrupt(cmd));
    let len = usize::from(word(buf, at(2)?)?);
    let last = cmd & 1 == 1;
    match cmd & 0xFE {
        0xB0 => {
            let byte = *buf.get(at(3)?).ok_or(Error::Corrupt(cmd))?;
            Ok((Cmd::Fill { byte, len, last }, at(4)?))
        }
        0xB2 => Ok((
            Cmd::Copy {
                src_end: at(3)?,
                len,
                last,
            },
            at(3 + len)?,
        )),
        _ => Err(Error::Corrupt(cmd)),
    }
}
