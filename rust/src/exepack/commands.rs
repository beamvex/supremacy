use super::cmd::Cmd;
use super::copy::copy;
use super::fill::fill;
use super::read_cmd::read_cmd;
use super::Error;

/// The decompress loop at `0x23AD0`-`0x23B14`: read command blocks backwards
/// from `si`, expanding into `buf` backwards from `di`, until a block with
/// `cmd & 1` set.
pub(crate) fn run(buf: &mut [u8], mut si: usize, mut di: usize) -> Result<(), Error> {
    loop {
        let (c, next) = read_cmd(buf, si)?;
        let last = c.last();
        match c {
            Cmd::Fill { byte, len, .. } => fill(buf, &mut di, byte, len)?,
            Cmd::Copy { src_end, len, .. } => copy(buf, &mut di, src_end, len)?,
        }
        if last {
            return Ok(());
        }
        si = next;
    }
}
