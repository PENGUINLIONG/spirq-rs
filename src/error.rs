//! Error and result reported by SPIR-Q procedures.
use std::fmt;
use std::error;

#[derive(Debug)]
pub enum Error {
    CorruptedSpirv,
    UnsupportedSpirv,
    MismatchedManifest,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            CorruptedSpirv => write!(f, "spirv binary is corrupted"),
            UnsupportedSpirv => write!(f, "spirv binary used unsupported feature"),
            MismatchedManifest => write!(f, "mismatched manifest cannot be merged"),
        }
    }
}
impl error::Error for Error { }

pub type Result<T> = std::result::Result<T, Error>;
