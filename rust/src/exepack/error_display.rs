use super::Error;
use std::fmt::{Display, Formatter};

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadMz => write!(f, "not an MZ executable"),
            Self::MissingPack => write!(f, "no EXEPACK header at entry CS"),
            Self::Corrupt(b) => write!(f, "packed file is corrupt (byte {b:#04X})"),
        }
    }
}

impl std::error::Error for Error {}
