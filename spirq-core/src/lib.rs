pub use spirv;

pub mod annotation;
pub mod constant;
pub mod evaluator;
pub mod func;
pub mod locator;
pub mod parse;
pub mod ty;
pub mod var;

/// Error infrastructure.
pub mod error {
    pub use anyhow::{anyhow, Error, Result};
}
