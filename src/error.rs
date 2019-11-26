use std::fmt;
use std::error;

#[derive(Debug)]
pub enum Error {
    CorruptedSpirv,
    UnsupportedSpirv,
    PipelineStageConflict,
    MismatchedManifest,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            CorruptedSpirv => write!(f, "spirv binary is corrupted"),
            UnsupportedSpirv => write!(f, "spirv binary used unsupported feature"),
            PipelineStageConflict => write!(f, "pipeline cannot have two stages of the same execution model"),
            MismatchedManifest => write!(f, "mismatched manifest cannot be merged"),
        }
    }
}
impl error::Error for Error { }
