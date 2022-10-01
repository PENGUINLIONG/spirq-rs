//! Error and result reported by SPIR-Q procedures.
use std::error;
use std::fmt;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Error {
    CorruptedSpirv(&'static str),
    UnsupportedSpirv(&'static str),
    MismatchedManifest,
}
impl Error {
    pub const INSTR_TOO_SHORT: Self = Self::CorruptedSpirv("instruction is too short");
    pub const STR_NOT_TERMINATED: Self =
        Self::CorruptedSpirv("instruction has a string operand that is not terminated by nul");
    pub const UNENCODED_ENUM: Self =
        Self::CorruptedSpirv("instruction has a unencoded enumeration value");

    pub const ID_COLLISION: Self = Self::CorruptedSpirv("id can only be assigned once");
    pub const NAME_COLLISION: Self = Self::CorruptedSpirv("item can only be named once");
    pub const DECO_COLLISION: Self =
        Self::CorruptedSpirv("item can only be decorated of a kind once");
    pub const MISSING_DECO: Self = Self::CorruptedSpirv("missing decoration");
    pub const TY_NOT_FOUND: Self = Self::CorruptedSpirv("cannot find a type");
    pub const CONST_NOT_FOUND: Self = Self::CorruptedSpirv("cannot find a constant");
    pub const FUNC_NOT_FOUND: Self = Self::CorruptedSpirv("cannot find a function");
    pub const BROKEN_NESTED_TY: Self =
        Self::CorruptedSpirv("nested type member violated the specification");
    pub const BROKEN_ACCESS_CHAIN: Self =
        Self::CorruptedSpirv("pointer in access chain points to non-existing type");
    pub const ACCESS_CONFLICT: Self =
        Self::CorruptedSpirv("variable is both read-only and write-only");
    pub const SPEC_DIV_BY_ZERO: Self =
        Self::CorruptedSpirv("specialized constexpr contains division by zero");
    pub const SPEC_TY_MISMATCHED: Self =
        Self::CorruptedSpirv("specialized constexpr param type mismatched");

    // TODO: (penguinliong) Mechanism to ignore unsupported features.
    pub const UNSUPPORTED_TY: Self = Self::UnsupportedSpirv("unsupported type");
    pub const UNSUPPORTED_EXEC_MODE: Self = Self::UnsupportedSpirv("unsupported execution mode");
    pub const UNSUPPORTED_IMG_CFG: Self = Self::UnsupportedSpirv("unsupported image configuration");
    pub const UNSUPPORTED_SPEC: Self = Self::UnsupportedSpirv("unsupported specialization");
    pub const UNSUPPORTED_CONST_TY: Self = Self::UnsupportedSpirv("unsupported constant type");
    pub const UNSUPPORTED_CONST_VALUE: Self = Self::UnsupportedSpirv("unsupported constant value");
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
impl error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
