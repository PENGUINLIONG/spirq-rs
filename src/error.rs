//! Error and result reported by SPIR-Q procedures.
use std::fmt;
use std::error;

#[derive(Debug)]
pub enum Error {
    CorruptedSpirv(&'static str),
    UnsupportedSpirv(&'static str),
    MismatchedManifest,
}
impl Error {
    pub const INSTR_TOO_SHORT: Self = Self::CorruptedSpirv("instruction is too short");
    pub const STR_NOT_TERMINATED: Self = Self::CorruptedSpirv("instruction has a string operand that is not terminated by nul");
    pub const UNENCODED_ENUM: Self = Self::CorruptedSpirv("instruction has a unencoded enumeration value");

    pub const ID_COLLISION: Self = Self::CorruptedSpirv("id can only be assigned once");
    pub const NAME_COLLISION: Self = Self::CorruptedSpirv("item can only be named once");
    pub const DECO_COLLISION: Self = Self::CorruptedSpirv("item can only be decorated of a kind once");
    pub const MISSING_DECO: Self = Self::CorruptedSpirv("missing decoration");
    pub const TY_NOT_FOUND: Self = Self::CorruptedSpirv("cannot find a suitable type");
    pub const CONST_NOT_FOUND: Self = Self::CorruptedSpirv("cannot find a suitable constant");
    pub const UNDECLARED_VAR: Self = Self::CorruptedSpirv("accessing undeclared variable");
    pub const DESC_BIND_COLLISION: Self = Self::CorruptedSpirv("descriptor binding cannot be shared");
    pub const UNKNOWN_NBIND: Self = Self::CorruptedSpirv("binding count cannot be determined");
    pub const MULTI_PUSH_CONST: Self = Self::CorruptedSpirv("an entry point cannot have multiple push constant blocks");
    pub const SPEC_ID_COLLISION: Self = Self::CorruptedSpirv("specialization id can only be assigned once");
    pub const FUNC_NOT_FOUND: Self = Self::CorruptedSpirv("cannot find a function");
    pub const BROKEN_ACCESS_CHAIN: Self = Self::CorruptedSpirv("pointer in access chain points to non-existing type");
    pub const ACCESS_CONFLICT: Self = Self::CorruptedSpirv("variable is both read-only and write-only");
    pub const LOCATION_COLLISION: Self = Self::CorruptedSpirv("interface variable location cannot be shared");

    // TODO: (penguinliong) Mechanism to ignore unsupported features.
    pub const UNSUPPORTED_TY: Self = Self::UnsupportedSpirv("unsupported type");
    pub const UNSUPPORTED_EXEC_MODE: Self = Self::UnsupportedSpirv("unsupported execution mode");
    pub const UNSUPPORTED_IMG_CFG: Self = Self::UnsupportedSpirv("unsupported image configuration");
    pub const UNSUPPORTED_SPEC: Self = Self::UnsupportedSpirv("unsupported specialization");
    pub const MULTI_ENTRY_POINTS: Self = Self::UnsupportedSpirv("cannot fast reflect a module with multiple entry points");
    pub const SAMPLER_IMG_NBIND_MISMATCH: Self = Self::UnsupportedSpirv("sampler and images doesn't pair up as sampled images");
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            CorruptedSpirv(msg) => write!(f, "spirv binary is corrupted: {}", msg),
            UnsupportedSpirv(msg) => write!(f, "spirv binary is unsupported: {}", msg),
            MismatchedManifest => write!(f, "mismatched manifest cannot be merged"),
        }
    }
}
impl error::Error for Error { }

pub type Result<T> = std::result::Result<T, Error>;
