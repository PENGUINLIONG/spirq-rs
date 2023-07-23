use std::collections::{HashMap, HashSet};

use crate::{
    constant::Constant,
    error::{anyhow, Result},
};

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

#[derive(Default)]
pub struct FunctionRegistry {
    func_map: HashMap<FunctionId, Function>,
}
impl FunctionRegistry {
    pub fn set(&mut self, id: FunctionId, func: Function) -> Result<()> {
        use std::collections::hash_map::Entry;
        match self.func_map.entry(id) {
            Entry::Vacant(entry) => {
                entry.insert(func);
                Ok(())
            }
            _ => Err(anyhow!("function id {} already existed", id)),
        }
    }

    pub fn get(&self, id: FunctionId) -> Result<&Function> {
        self.func_map
            .get(&id)
            .ok_or(anyhow!("function id {} is not found", id))
    }
    pub fn get_mut(&mut self, id: FunctionId) -> Result<&mut Function> {
        self.func_map
            .get_mut(&id)
            .ok_or(anyhow!("function id {} is not found", id))
    }
    pub fn get_by_name(&self, name: &str) -> Result<&Function> {
        self.func_map
            .values()
            .find(|x| {
                if let Some(nm) = x.name.as_ref() {
                    nm == name
                } else {
                    false
                }
            })
            .ok_or(anyhow!("function {} is not found", name))
    }

    pub fn collect_ordered(&self) -> Vec<Function> {
        let mut out: Vec<_> = self.func_map.iter().collect();
        out.sort_by_key(|x| x.0);
        out.into_iter().map(|x| x.1.clone()).collect()
    }

    fn collect_fn_vars_impl(&self, func: FunctionId, vars: &mut Vec<VariableId>) {
        if let Ok(func) = self.get(func) {
            vars.extend(func.accessed_vars.iter());
            for call in func.callees.iter() {
                self.collect_fn_vars_impl(*call, vars);
            }
        }
    }
    pub fn collect_fn_vars(&self, func: FunctionId) -> Vec<VariableId> {
        let mut accessed_vars = Vec::new();
        self.collect_fn_vars_impl(func, &mut accessed_vars);
        accessed_vars
    }
}
