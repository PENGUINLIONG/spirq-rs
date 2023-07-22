use spirq_types::{DescriptorType, Type, Walk};

use crate::{DescriptorBinding, InterfaceLocation, Locator, SpecId};

pub trait SpirvVariable {
    /// Debug name of this variable.
    fn name(&self) -> Option<&str>;
    /// Locator of the variable.
    fn locator(&self) -> Locator;
    /// Concrete type of the variable.
    fn ty(&self) -> &Type;
    /// Enumerate variable members in post-order.
    fn walk<'a>(&'a self) -> Walk<'a> {
        self.ty().walk()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputVariable {
    pub name: Option<String>,
    // Interface location of input.
    pub location: InterfaceLocation,
    /// The concrete SPIR-V type definition of descriptor resource.
    pub ty: Type,
}
impl SpirvVariable for InputVariable {
    fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|x| x as &str)
    }
    fn locator(&self) -> Locator {
        Locator::Input(self.location)
    }
    fn ty(&self) -> &Type {
        &self.ty
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OutputVariable {
    pub name: Option<String>,
    // Interface location of output.
    pub location: InterfaceLocation,
    /// The concrete SPIR-V type definition of descriptor resource.
    pub ty: Type,
}
impl SpirvVariable for OutputVariable {
    fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|x| x as &str)
    }
    fn locator(&self) -> Locator {
        Locator::Output(self.location)
    }
    fn ty(&self) -> &Type {
        &self.ty
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DescriptorVariable {
    pub name: Option<String>,
    // Binding point of descriptor resource.
    pub desc_bind: DescriptorBinding,
    /// Descriptor resource type matching `VkDescriptorType`.
    pub desc_ty: DescriptorType,
    /// The concrete SPIR-V type definition of descriptor resource.
    pub ty: Type,
    /// Number of bindings at the binding point. All descriptors can have
    /// multiple binding points. If the multi-binding is dynamic, 0 will be
    /// returned.
    ///
    /// For more information about dynamic multi-binding, please refer to
    /// Vulkan extension `VK_EXT_descriptor_indexing`, GLSL extension
    /// `GL_EXT_nonuniform_qualifier` and SPIR-V extension
    /// `SPV_EXT_descriptor_indexing`. Dynamic multi-binding is only
    /// supported in Vulkan 1.2.
    pub bind_count: u32,
}
impl SpirvVariable for DescriptorVariable {
    fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|x| x as &str)
    }
    fn locator(&self) -> Locator {
        Locator::Descriptor(self.desc_bind)
    }
    fn ty(&self) -> &Type {
        &self.ty
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PushConstantVariable {
    pub name: Option<String>,
    /// The concrete SPIR-V type definition of descriptor resource.
    pub ty: Type,
}
impl SpirvVariable for PushConstantVariable {
    fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|x| x as &str)
    }
    fn locator(&self) -> Locator {
        Locator::PushConstant
    }
    fn ty(&self) -> &Type {
        &self.ty
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpecConstantVariable {
    pub name: Option<String>,
    /// Specialization constant ID.
    pub spec_id: SpecId,
    /// The concrete SPIR-V type definition of descriptor resource.
    pub ty: Type,
}
impl SpirvVariable for SpecConstantVariable {
    fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|x| x as &str)
    }
    fn locator(&self) -> Locator {
        Locator::SpecConstant(self.spec_id)
    }
    fn ty(&self) -> &Type {
        &self.ty
    }
}

/// A SPIR-V variable - interface variables, descriptor resources and push
/// constants.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Variable {
    /// Input interface variable.
    Input(InputVariable),
    /// Output interface variable.
    Output(OutputVariable),
    /// Descriptor resource.
    Descriptor(DescriptorVariable),
    /// Push constant.
    PushConstant(PushConstantVariable),
    /// Specialization constant.
    SpecConstant(SpecConstantVariable),
}
impl SpirvVariable for Variable {
    fn name(&self) -> Option<&str> {
        match self {
            Variable::Input(x) => x.name(),
            Variable::Output(x) => x.name(),
            Variable::Descriptor(x) => x.name(),
            Variable::PushConstant(x) => x.name(),
            Variable::SpecConstant(x) => x.name(),
        }
    }
    fn locator(&self) -> Locator {
        match self {
            Variable::Input(x) => x.locator(),
            Variable::Output(x) => x.locator(),
            Variable::Descriptor(x) => x.locator(),
            Variable::PushConstant(x) => x.locator(),
            Variable::SpecConstant(x) => x.locator(),
        }
    }
    fn ty(&self) -> &Type {
        match self {
            Variable::Input(x) => x.ty(),
            Variable::Output(x) => x.ty(),
            Variable::Descriptor(x) => x.ty(),
            Variable::PushConstant(x) => x.ty(),
            Variable::SpecConstant(x) => x.ty(),
        }
    }
}
