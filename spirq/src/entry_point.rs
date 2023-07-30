//! Entry-point function record.
use std::fmt;

use crate::{func::ExecutionMode, spirv, var::Variable};

pub use spirv::ExecutionModel;

/// Representing an entry point described in a SPIR-V.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct EntryPoint {
    /// Entry point execution model.
    pub exec_model: spirv::ExecutionModel,
    /// Name of the entry point.
    pub name: String,
    /// Variables that contains specialization constant, input, output and
    /// descriptor type information.
    ///
    /// Note that it is possible that multiple resources are bound to a same
    /// `Locator` so this is not a map.
    pub vars: Vec<Variable>,
    /// Execution modes the entry point will execute in, including predefined
    /// compute shader local sizes and specialization constant IDs of local
    /// sizes.
    pub exec_modes: Vec<ExecutionMode>,
}
impl fmt::Debug for EntryPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct(&self.name)
            .field("exec_model", &self.exec_model)
            .field("name", &self.name)
            .field("vars", &self.vars)
            .field("exec_modes", &self.exec_modes)
            .finish()
    }
}
