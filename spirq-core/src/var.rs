use crate::{
    error::{anyhow, Result},
    locator::{DescriptorBinding, InterfaceLocation, Locator, SpecId},
    ty::{AccessType, PointerType, StorageClass, Type, Walk},
};
use fnv::FnvHashMap as HashMap;

type VariableId = u32;

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

/// Descriptor type matching `VkDescriptorType`.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum DescriptorType {
    /// `VK_DESCRIPTOR_TYPE_SAMPLER`
    Sampler,
    /// `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`
    CombinedImageSampler,
    /// `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`
    SampledImage,
    /// `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`
    StorageImage { access_ty: AccessType },
    /// `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`.
    UniformTexelBuffer,
    /// `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`.
    StorageTexelBuffer { access_ty: AccessType },
    /// `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or
    /// `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` depending on how you gonna
    /// use it.
    UniformBuffer,
    /// `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or
    /// `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` depending on how you gonna
    /// use it.
    StorageBuffer { access_ty: AccessType },
    /// `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` and its input attachment index.
    InputAttachment { input_attachment_index: u32 },
    /// `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR`
    AccelStruct,
}
impl DescriptorType {
    pub fn sampler() -> Self {
        Self::Sampler
    }
    pub fn combined_image_sampler() -> Self {
        Self::CombinedImageSampler
    }
    pub fn sampled_image() -> Self {
        Self::SampledImage
    }
    pub fn storage_image(access_ty: AccessType) -> Self {
        Self::StorageImage { access_ty }
    }
    pub fn read_only_storage_image() -> Self {
        Self::storage_image(AccessType::ReadOnly)
    }
    pub fn write_only_storage_image() -> Self {
        Self::storage_image(AccessType::WriteOnly)
    }
    pub fn read_write_storage_image() -> Self {
        Self::storage_image(AccessType::ReadWrite)
    }
    pub fn uniform_texel_buffer() -> Self {
        Self::UniformTexelBuffer
    }
    pub fn storage_texel_buffer(access_ty: AccessType) -> Self {
        Self::StorageTexelBuffer { access_ty }
    }
    pub fn read_only_storage_texel_buffer() -> Self {
        Self::storage_texel_buffer(AccessType::ReadOnly)
    }
    pub fn write_only_storage_texel_buffer() -> Self {
        Self::storage_texel_buffer(AccessType::WriteOnly)
    }
    pub fn read_write_storage_texel_buffer() -> Self {
        Self::storage_texel_buffer(AccessType::ReadWrite)
    }
    pub fn uniform_buffer() -> Self {
        Self::UniformBuffer
    }
    pub fn storage_buffer(access_ty: AccessType) -> Self {
        Self::StorageBuffer { access_ty }
    }
    pub fn read_only_storage_buffer() -> Self {
        Self::storage_buffer(AccessType::ReadOnly)
    }
    pub fn write_only_storage_buffer() -> Self {
        Self::storage_buffer(AccessType::WriteOnly)
    }
    pub fn read_write_storage_buffer() -> Self {
        Self::storage_buffer(AccessType::ReadWrite)
    }
    pub fn input_attachment(input_attachment_index: u32) -> Self {
        Self::InputAttachment {
            input_attachment_index,
        }
    }
    pub fn accel_struct() -> Self {
        Self::AccelStruct
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
