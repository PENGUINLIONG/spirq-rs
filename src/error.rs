use std::fmt;
use std::error;

#[derive(Debug)]
pub enum Error {
    CorruptedSpirv,
    UnsupportedSpirv,
    MalformedPipeline,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            CorruptedSpirv => write!(f, "spirv binary is corrupted"),
            UnsupportedSpirv => write!(f, "spirv binary used unsupported feature"),
            MalformedPipeline => write!(f, "pipeline is invalid"),
        }
    }
}
impl error::Error for Error { }
