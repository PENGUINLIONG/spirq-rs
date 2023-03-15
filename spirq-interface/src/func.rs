use std::collections::HashSet;

use crate::Constant;

type VariableId = u32;
type FunctionId = u32;

/// SPIR-V execution mode.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct ExecutionMode {
    pub exec_mode: spirv::ExecutionMode,
    pub operands: Vec<Constant>,
}

/// Function reflection intermediate.
#[derive(Default, Debug, Clone)]
pub struct Function {
    pub name: Option<String>,
    pub accessed_vars: HashSet<VariableId>,
    pub callees: HashSet<FunctionId>,
    pub exec_modes: Vec<ExecutionMode>,
}
