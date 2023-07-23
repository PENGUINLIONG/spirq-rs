use crate::{
    error::{anyhow, Result},
    ty::{DescriptorType, PointerType, StorageClass, Type, Walk},
};
use fnv::FnvHashMap as HashMap;
use std::fmt;

type VariableId = u32;

/// Descriptor set and binding point carrier.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Default, Clone, Copy)]
pub struct DescriptorBinding(u32, u32);
impl DescriptorBinding {
    pub fn new(desc_set: u32, bind_point: u32) -> Self {
        DescriptorBinding(desc_set, bind_point)
    }

    pub fn set(&self) -> u32 {
        self.0
    }
    pub fn bind(&self) -> u32 {
        self.1
    }
    pub fn into_inner(self) -> (u32, u32) {
        (self.0, self.1)
    }
}
impl fmt::Display for DescriptorBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(set={}, bind={})", self.0, self.1)
    }
}
impl fmt::Debug for DescriptorBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self as &dyn fmt::Display).fmt(f)
    }
}

/// Interface variable location and component.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Default, Clone, Copy)]
pub struct InterfaceLocation(u32, u32);
impl InterfaceLocation {
    pub fn new(loc: u32, comp: u32) -> Self {
        InterfaceLocation(loc, comp)
    }

    pub fn loc(&self) -> u32 {
        self.0
    }
    pub fn comp(&self) -> u32 {
        self.1
    }
    pub fn into_inner(self) -> (u32, u32) {
        (self.0, self.1)
    }
}
impl fmt::Display for InterfaceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(loc={}, comp={})", self.0, self.1)
    }
}
impl fmt::Debug for InterfaceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self as &dyn fmt::Display).fmt(f)
    }
}

/// Specialization constant ID.
pub type SpecId = u32;

/// A SPIR-V variable - interface variables, descriptor resources and push
/// constants.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Variable {
    /// Input interface variable.
    Input {
        name: Option<String>,
        // Interface location of input.
        location: InterfaceLocation,
        /// The concrete SPIR-V type definition of descriptor resource.
        ty: Type,
    },
    /// Output interface variable.
    Output {
        name: Option<String>,
        // Interface location of output.
        location: InterfaceLocation,
        /// The concrete SPIR-V type definition of descriptor resource.
        ty: Type,
    },
    /// Descriptor resource.
    Descriptor {
        name: Option<String>,
        // Binding point of descriptor resource.
        desc_bind: DescriptorBinding,
        /// Descriptor resource type matching `VkDescriptorType`.
        desc_ty: DescriptorType,
        /// The concrete SPIR-V type definition of descriptor resource.
        ty: Type,
        /// Number of bindings at the binding point. All descriptors can have
        /// multiple binding points. If the multi-binding is dynamic, 0 will be
        /// returned.
        ///
        /// For more information about dynamic multi-binding, please refer to
        /// Vulkan extension `VK_EXT_descriptor_indexing`, GLSL extension
        /// `GL_EXT_nonuniform_qualifier` and SPIR-V extension
        /// `SPV_EXT_descriptor_indexing`. Dynamic multi-binding is only
        /// supported in Vulkan 1.2.
        bind_count: u32,
    },
    /// Push constant.
    PushConstant {
        name: Option<String>,
        /// The concrete SPIR-V type definition of descriptor resource.
        ty: Type,
    },
    /// Specialization constant.
    SpecConstant {
        name: Option<String>,
        /// Specialization constant ID.
        spec_id: SpecId,
        /// The concrete SPIR-V type definition of descriptor resource.
        ty: Type,
    },
}
impl Variable {
    pub fn name(&self) -> Option<&str> {
        match self {
            Variable::Input { name, .. } => name.as_deref(),
            Variable::Output { name, .. } => name.as_deref(),
            Variable::Descriptor { name, .. } => name.as_deref(),
            Variable::PushConstant { name, .. } => name.as_deref(),
            Variable::SpecConstant { name, .. } => name.as_deref(),
        }
    }
    pub fn ty(&self) -> &Type {
        match self {
            Variable::Input { ty, .. } => ty,
            Variable::Output { ty, .. } => ty,
            Variable::Descriptor { ty, .. } => ty,
            Variable::PushConstant { ty, .. } => ty,
            Variable::SpecConstant { ty, .. } => ty,
        }
    }
    pub fn walk<'a>(&'a self) -> Walk<'a> {
        self.ty().walk()
    }
}

/// Variable allocated by `OpVariable`.
pub struct VariableAlloc {
    pub name: Option<String>,
    /// Variable storage class.
    pub store_cls: StorageClass,
    /// Pointer type of the variable. It points to an array if it's a multibind.
    /// Otherwise, it directly points to the actual inner type.
    pub ptr_ty: PointerType,
}

#[derive(Default)]
pub struct VariableRegistry {
    var_map: HashMap<VariableId, VariableAlloc>,
}
impl VariableRegistry {
    pub fn set(&mut self, id: VariableId, var: VariableAlloc) -> Result<()> {
        use std::collections::hash_map::Entry;
        match self.var_map.entry(id) {
            Entry::Vacant(entry) => {
                entry.insert(var);
                Ok(())
            }
            _ => Err(anyhow!("variable id {} already existed", id)),
        }
    }

    pub fn get(&self, id: VariableId) -> Result<&VariableAlloc> {
        self.var_map
            .get(&id)
            .ok_or(anyhow!("variable id {} is not found", id))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&VariableId, &VariableAlloc)> {
        self.var_map.iter()
    }
}
