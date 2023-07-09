use spirq_types::{DescriptorType, Type, Walk};

use crate::{DescriptorBinding, InterfaceLocation, Locator, SpecId};

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
        nbind: u32,
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
        /// The type of the specialization constant.
        ty: Type,
    },
}
impl Variable {
    /// Debug name of this variable.
    pub fn name(&self) -> Option<&str> {
        match self {
            Variable::Input { name, .. } => name.as_ref().map(|x| x as &str),
            Variable::Output { name, .. } => name.as_ref().map(|x| x as &str),
            Variable::Descriptor { name, .. } => name.as_ref().map(|x| x as &str),
            Variable::PushConstant { name, .. } => name.as_ref().map(|x| x as &str),
            Variable::SpecConstant { name, .. } => name.as_ref().map(|x| x as &str),
        }
    }
    /// Remove name of the variable.
    pub fn clear_name(&mut self) {
        match self {
            Variable::Input { name, .. } => *name = None,
            Variable::Output { name, .. } => *name = None,
            Variable::Descriptor { name, .. } => *name = None,
            Variable::PushConstant { name, .. } => *name = None,
            Variable::SpecConstant { name, .. } => *name = None,
        }
    }
    /// Locator of the variable.
    pub fn locator(&self) -> Locator {
        match self {
            Variable::Input { location, .. } => Locator::Input(*location),
            Variable::Output { location, .. } => Locator::Output(*location),
            Variable::Descriptor { desc_bind, .. } => Locator::Descriptor(*desc_bind),
            Variable::PushConstant { .. } => Locator::PushConstant,
            Variable::SpecConstant { spec_id, .. } => Locator::SpecConstant(*spec_id),
        }
    }
    /// Descriptor type if it's a descriptor resource.
    pub fn desc_ty(&self) -> Option<DescriptorType> {
        if let Variable::Descriptor { desc_ty, .. } = self {
            Some(desc_ty.clone())
        } else {
            None
        }
    }
    /// Specialization constant ID if it's a specialization constant.
    pub fn spec_id(&self) -> Option<SpecId> {
        if let Variable::SpecConstant { spec_id, .. } = self {
            Some(*spec_id)
        } else {
            None
        }
    }
    /// Concrete type of the variable.
    pub fn ty(&self) -> &Type {
        match self {
            Variable::Input { ty, .. } => ty,
            Variable::Output { ty, .. } => ty,
            Variable::Descriptor { ty, .. } => ty,
            Variable::PushConstant { ty, .. } => ty,
            Variable::SpecConstant { ty, .. } => ty,
        }
    }
    /// Number of bindings at the binding point it it's a descriptor resource.
    pub fn nbind(&self) -> Option<u32> {
        if let Variable::Descriptor { nbind, .. } = self {
            Some(*nbind)
        } else {
            None
        }
    }
    /// Enumerate variable members in post-order.
    pub fn walk<'a>(&'a self) -> Walk<'a> {
        self.ty().walk()
    }
}
