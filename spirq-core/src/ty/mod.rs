//! Structured representations of SPIR-V types.
use std::fmt;
use std::hash::Hash;
use std::rc::Rc;

pub mod reg;
pub mod walk;

pub use self::{reg::TypeRegistry, walk::Walk};
pub use crate::spirv::{Dim, ImageFormat, StorageClass};

pub trait SpirvType {
    /// Minimum size of the type in bytes if it can be represented in-memory.
    /// It's the size of all static members and plus one element if it's an array.
    /// Same as [`wgpu::BindingType::Buffer::min_binding_size`](https://docs.rs/wgpu/latest/wgpu/enum.BindingType.html).
    fn min_size(&self) -> Option<usize>;
    /// Size of the type in bytes if it can be represented in-memory.
    fn size(&self) -> Option<usize> {
        self.min_size()
    }
    /// Returns true if the type is sized. A sized type can be represented
    /// in-memory. Otherwise the type can only be used as a descriptor resource.
    fn is_sized(&self) -> bool {
        self.size().is_some()
    }
    /// Returns the offset of the i-th member in bytes if it's a composite type.
    fn member_offset(&self, _member_index: usize) -> Option<usize> {
        None
    }
    /// Returns how the type can be accessed: `ReadOnly`, `WriteOnly` or
    /// `ReadWrite`.
    fn access_ty(&self) -> Option<AccessType> {
        None
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum ScalarType {
    /// Pseudo-type representing no data. It's sometimes used to represent data
    /// without a type hint at compile-time in SPIR-V. You shouldn't see this in
    /// your reflection results.
    Void,
    /// Boolean value of either true or false. Be careful with booleans.
    /// Booleans is NOT allowed to be exposed to the host according to the
    /// SPIR-V specification. You shouldn't see this in your reflection results.
    Boolean,
    /// Two's complement integer.
    Integer {
        /// Number of bytes the integer takes.
        bits: u32,
        /// Whether the integer is signed.
        is_signed: bool,
    },
    /// IEEE 754 floating-point number.
    Float {
        /// Number of bytes the float takes.
        bits: u32,
    },
}
impl ScalarType {
    /// Create a signed integer type with the given number of bits.
    pub fn int(bits: u32) -> Self {
        Self::Integer {
            bits,
            is_signed: true,
        }
    }
    /// Create an unsigned integer type with the given number of bits.
    pub fn uint(bits: u32) -> Self {
        Self::Integer {
            bits,
            is_signed: false,
        }
    }
    /// Create a floating point type with the given number of bits.
    pub fn float(bits: u32) -> Self {
        Self::Float { bits }
    }
    /// Create a 32-bit signed integer type.
    pub fn i32() -> Self {
        Self::int(32)
    }
    /// Create a 32-bit unsigned integer type.
    pub fn u32() -> Self {
        Self::uint(32)
    }
    /// Create a 32-bit floating-point type.
    pub fn f32() -> Self {
        Self::float(32)
    }
}
impl SpirvType for ScalarType {
    fn min_size(&self) -> Option<usize> {
        match self {
            Self::Void => None,
            Self::Boolean => None,
            Self::Integer { bits, .. } => Some((*bits / 8) as usize),
            Self::Float { bits } => Some((*bits / 8) as usize),
        }
    }
}
impl fmt::Display for ScalarType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Void => f.write_str("void"),
            Self::Boolean => f.write_str("bool"),
            Self::Integer { bits, is_signed } => match is_signed {
                true => write!(f, "i{}", bits),
                false => write!(f, "u{}", bits),
            },
            Self::Float { bits } => write!(f, "f{}", bits),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct VectorType {
    /// Vector scalar type.
    pub scalar_ty: ScalarType,
    /// Number of scalar components in the vector.
    pub scalar_count: u32,
}
impl SpirvType for VectorType {
    fn min_size(&self) -> Option<usize> {
        Some(self.scalar_ty.min_size()? * self.scalar_count as usize)
    }
}
impl fmt::Display for VectorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "vec{}<{}>", self.scalar_count, self.scalar_ty)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum MatrixAxisOrder {
    ColumnMajor,
    RowMajor,
}
impl Default for MatrixAxisOrder {
    fn default() -> MatrixAxisOrder {
        MatrixAxisOrder::ColumnMajor
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct MatrixType {
    /// Matrix vector type.
    pub vector_ty: VectorType,
    /// Number of vectors in the matrix.
    pub vector_count: u32,
    /// Axis order of the matrix. Valid SPIR-V never gives a `None` major.
    pub axis_order: Option<MatrixAxisOrder>,
    /// Stride between vectors in the matrix. Valid SPIR-V never gives a `None`
    /// stride.
    pub stride: Option<usize>,
}
impl SpirvType for MatrixType {
    fn min_size(&self) -> Option<usize> {
        Some(self.stride? * self.vector_count as usize)
    }
}
impl fmt::Display for MatrixType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let major = match self.axis_order {
            Some(MatrixAxisOrder::ColumnMajor) => "ColumnMajor",
            Some(MatrixAxisOrder::RowMajor) => "RowMajor",
            None => "AxisOrder?",
        };
        let nrow = self.vector_ty.scalar_count;
        let ncol = self.vector_count;
        let scalar_ty = &self.vector_ty.scalar_ty;
        let stride = match self.stride {
            Some(x) => x.to_string(),
            None => "?".to_owned(),
        };
        write!(f, "mat{nrow}x{ncol}<{scalar_ty},{major},{stride}>")
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct ImageType {
    /// Scalar type of image access result.
    pub scalar_ty: ScalarType,
    /// Dimension of the image.
    pub dim: Dim,
    /// Whether the image is a depth image, or `None` if it's unknown at compile
    /// time.
    pub is_depth: Option<bool>,
    /// Whether the image is an array of images. In Vulkan, it means that the
    /// image can have multiple layers. In Vulkan, only `Dim1D`, `Dim2D`, and
    /// `DimCube` can be arrayed.
    pub is_array: bool,
    /// Whether the image is multisampled. In Vulkan, only 2D images and 2D
    /// image arrays can be multi sampled.
    pub is_multisampled: bool,
    /// Whether the image is sampled, or `None` if it's unknown at compile time.
    pub is_sampled: Option<bool>,
    /// Matches `VkImageCreateInfo::format`. Can be `ImageFormat::Unknown` in
    /// case of a sampled image.
    pub fmt: ImageFormat,
}
impl SpirvType for ImageType {
    fn min_size(&self) -> Option<usize> {
        None
    }
}
impl fmt::Display for ImageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let scalar_ty = &self.scalar_ty;
        let is_sampled = match self.is_sampled {
            Some(true) => "Sampled",
            Some(false) => "Storage",
            None => "Sampled?",
        };
        let depth = match self.is_depth {
            Some(true) => "Depth",
            Some(false) => "Color",
            None => "Depth?",
        };
        let dim = match self.dim {
            Dim::Dim1D => "1D",
            Dim::Dim2D => "2D",
            Dim::Dim3D => "3D",
            Dim::DimBuffer => "Buffer",
            Dim::DimCube => "Cube",
            Dim::DimRect => "Rect",
            Dim::DimSubpassData => "SubpassData",
        };
        let is_array = match self.is_array {
            true => "Array",
            false => "",
        };
        let is_multisampled = match self.is_multisampled {
            true => "MS",
            false => "",
        };
        write!(
            f,
            "Image{dim}{is_array}{is_multisampled}<{scalar_ty},{is_sampled},{depth},{:?}>",
            self.fmt
        )
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct SamplerType {}
impl SpirvType for SamplerType {
    fn min_size(&self) -> Option<usize> {
        None
    }
}
impl fmt::Display for SamplerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Sampler")
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct CombinedImageSamplerType {
    pub sampled_img_ty: SampledImageType,
}
impl SpirvType for CombinedImageSamplerType {
    fn min_size(&self) -> Option<usize> {
        None
    }
}
impl fmt::Display for CombinedImageSamplerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CombinedImageSampler<{}>", self.sampled_img_ty)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct SampledImageType {
    /// Scalar type of image access result.
    pub scalar_ty: ScalarType,
    /// Dimension of the image.
    pub dim: Dim,
    /// Whether  the image is an array of images. In Vulkan, it means that the
    /// image can have multiple layers. In Vulkan, only `Dim1D`, `Dim2D`, and
    /// `DimCube` can be arrayed.
    pub is_array: bool,
    /// Whether the image is multisampled. In Vulkan, only 2D images and 2D
    /// image arrays can be multi sampled.
    pub is_multisampled: bool,
}
impl SpirvType for SampledImageType {
    fn min_size(&self) -> Option<usize> {
        None
    }
}
impl fmt::Display for SampledImageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let scalar_ty = &self.scalar_ty;
        let dim = match self.dim {
            Dim::Dim1D => "1D",
            Dim::Dim2D => "2D",
            Dim::Dim3D => "3D",
            Dim::DimBuffer => "Buffer",
            Dim::DimCube => "Cube",
            Dim::DimRect => "Rect",
            Dim::DimSubpassData => "SubpassData",
        };
        let is_array = match self.is_array {
            true => "Array",
            false => "",
        };
        let is_multisampled = match self.is_multisampled {
            true => "MS",
            false => "",
        };
        write!(
            f,
            "SampledImage{dim}{is_array}{is_multisampled}<{scalar_ty}>"
        )
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct StorageImageType {
    /// Dimension of the image.
    pub dim: Dim,
    /// Whether  the image is an array of images. In Vulkan, it means that the
    /// image can have multiple layers. In Vulkan, only `Dim1D`, `Dim2D`, and
    /// `DimCube` can be arrayed.
    pub is_array: bool,
    /// Whether the image is multisampled. In Vulkan, only 2D images and 2D
    /// image arrays can be multi sampled.
    pub is_multisampled: bool,
    /// Matches `VkImageCreateInfo::format`. Can be `ImageFormat::Unknown` in
    /// case of a sampled image.
    pub fmt: ImageFormat,
}
impl SpirvType for StorageImageType {
    fn min_size(&self) -> Option<usize> {
        None
    }
}
impl fmt::Display for StorageImageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dim = match self.dim {
            Dim::Dim1D => "1D",
            Dim::Dim2D => "2D",
            Dim::Dim3D => "3D",
            Dim::DimBuffer => "Buffer",
            Dim::DimCube => "Cube",
            Dim::DimRect => "Rect",
            Dim::DimSubpassData => "SubpassData",
        };
        let is_array = match self.is_array {
            true => "Array",
            false => "",
        };
        let is_multisampled = match self.is_multisampled {
            true => "MS",
            false => "",
        };
        write!(
            f,
            "StorageImage{dim}{is_array}{is_multisampled}<{:?}>",
            self.fmt
        )
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct SubpassDataType {
    /// Scalar type of subpass data access result.
    pub scalar_ty: ScalarType,
    /// Image arrangement which encodes multisampling state.
    pub is_multisampled: bool,
}
impl SpirvType for SubpassDataType {
    fn min_size(&self) -> Option<usize> {
        None
    }
}
impl fmt::Display for SubpassDataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let scalar_ty = &self.scalar_ty;
        let is_multisampled = match self.is_multisampled {
            true => "MS",
            false => "",
        };
        write!(f, "SubpassData{is_multisampled}<{scalar_ty}>")
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct ArrayType {
    /// Type of the array element.
    pub element_ty: Box<Type>,
    /// Number of elements in the array. None if the array length is only known
    /// at runtime.
    pub element_count: Option<u32>,
    /// Stride between elements in the array. None if the array doesn't have
    /// a explicitly specified layout. For example, an array of descriptor
    /// resources doesn't have a physical layout.
    pub stride: Option<usize>,
}
impl SpirvType for ArrayType {
    fn min_size(&self) -> Option<usize> {
        Some(self.stride? * self.element_count.unwrap_or(0).max(1) as usize)
    }
    fn size(&self) -> Option<usize> {
        Some(self.stride? * self.element_count.unwrap_or(0) as usize)
    }
    fn member_offset(&self, member_index: usize) -> Option<usize> {
        Some(self.stride? * member_index)
    }
    fn access_ty(&self) -> Option<AccessType> {
        self.element_ty.access_ty()
    }
}
impl fmt::Display for ArrayType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(nrepeat) = self.element_count {
            write!(f, "[{}; {}]", self.element_ty, nrepeat)
        } else {
            write!(f, "[{}]", self.element_ty)
        }
    }
}

/// Access type of a variable.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccessType {
    /// The variable can be accessed by read.
    ReadOnly = 1,
    /// The variable can be accessed by write.
    WriteOnly = 2,
    /// The variable can be accessed by read or by write.
    ReadWrite = 3,
}
impl std::ops::BitOr<AccessType> for AccessType {
    type Output = AccessType;
    fn bitor(self, rhs: AccessType) -> AccessType {
        return match (self, rhs) {
            (Self::ReadOnly, Self::ReadOnly) => Self::ReadOnly,
            (Self::WriteOnly, Self::WriteOnly) => Self::WriteOnly,
            _ => Self::ReadWrite,
        };
    }
}
impl std::ops::BitAnd<AccessType> for AccessType {
    type Output = Option<AccessType>;
    fn bitand(self, rhs: AccessType) -> Option<AccessType> {
        return match (self, rhs) {
            (Self::ReadOnly, Self::ReadWrite)
            | (Self::ReadWrite, Self::ReadOnly)
            | (Self::ReadOnly, Self::ReadOnly) => Some(Self::ReadOnly),
            (Self::WriteOnly, Self::ReadWrite)
            | (Self::ReadWrite, Self::WriteOnly)
            | (Self::WriteOnly, Self::WriteOnly) => Some(Self::WriteOnly),
            (Self::ReadWrite, Self::ReadWrite) => Some(Self::ReadWrite),
            (_, _) => None,
        };
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct StructMember {
    pub name: Option<String>,
    /// Offset of this member from the beginning of the struct. None if the
    /// struct doesn't have a explicitly specified layout. For example,
    /// `gl_PerVertex` doesn't have a physical layout. You won't see a `None` in
    /// reflected result.
    pub offset: Option<usize>,
    pub ty: Type,
    pub access_ty: AccessType,
}
#[derive(PartialEq, Eq, Default, Clone, Hash, Debug)]
pub struct StructType {
    pub name: Option<String>,
    pub members: Vec<StructMember>, // Offset and type.
}
impl StructType {
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(AsRef::as_ref)
    }
}
impl SpirvType for StructType {
    fn min_size(&self) -> Option<usize> {
        let last_member = &self.members.last()?;
        Some(last_member.offset? + last_member.ty.min_size()?)
    }
    fn size(&self) -> Option<usize> {
        let last_member = &self.members.last()?;
        Some(last_member.offset? + last_member.ty.size()?)
    }
    fn member_offset(&self, member_index: usize) -> Option<usize> {
        self.members.get(member_index).and_then(|x| x.offset)
    }
    fn access_ty(&self) -> Option<AccessType> {
        self.members.iter().fold(None, |seed, x| match seed {
            None => Some(x.access_ty),
            Some(AccessType::ReadOnly) => match x.access_ty {
                AccessType::ReadOnly => Some(AccessType::ReadOnly),
                _ => Some(AccessType::ReadWrite),
            },
            Some(AccessType::WriteOnly) => match x.access_ty {
                AccessType::WriteOnly => Some(AccessType::WriteOnly),
                _ => Some(AccessType::ReadWrite),
            },
            Some(AccessType::ReadWrite) => Some(AccessType::ReadWrite),
        })
    }
}
impl fmt::Display for StructType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "{} {{ ", name)?;
        } else {
            f.write_str("{ ")?;
        }
        for (i, member) in self.members.iter().enumerate() {
            if i != 0 {
                f.write_str(", ")?;
            }
            if let Some(name) = &member.name {
                write!(f, "{}: {}", name, member.ty)?;
            } else {
                write!(f, "{}: {}", i, member.ty)?;
            }
        }
        f.write_str(" }")
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct AccelStructType {}
impl SpirvType for AccelStructType {
    fn min_size(&self) -> Option<usize> {
        None
    }
}
impl fmt::Display for AccelStructType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AccelStruct")
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct DeviceAddressType {}
impl SpirvType for DeviceAddressType {
    fn min_size(&self) -> Option<usize> {
        Some(std::mem::size_of::<u64>())
    }
}
impl fmt::Display for DeviceAddressType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Address")
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct PointerType {
    pub pointee_ty: Box<Type>,
    pub store_cls: StorageClass,
}
impl SpirvType for PointerType {
    fn min_size(&self) -> Option<usize> {
        Some(std::mem::size_of::<u64>())
    }
    fn access_ty(&self) -> Option<AccessType> {
        self.pointee_ty.access_ty()
    }
}
impl fmt::Display for PointerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Pointer { ")?;
        write!(f, "{}", *self.pointee_ty)?;
        f.write_str(" }")
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct RayQueryType {}
impl SpirvType for RayQueryType {
    fn min_size(&self) -> Option<usize> {
        None
    }
}
impl fmt::Display for RayQueryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("RayQuery")
    }
}

macro_rules! declr_ty_accessor {
    ([$e:ident] $($name:ident -> $ty:ident,)+) => {
        $(
            pub fn $name(&self) -> bool {
                match self {
                    $e::$ty(..) => true,
                    _ => false
                }
            }
        )+
    }
}
macro_rules! declr_ty_downcast {
    ([$e:ident] $($name:ident -> $ty:ident($inner_ty:ident),)+) => {
        $(
            pub fn $name(&self) -> Option<&$inner_ty> {
                match self {
                    $e::$ty(x) => Some(x),
                    _ => None
                }
            }
        )+
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
#[non_exhaustive]
pub enum Type {
    /// A single value, which can be a signed or unsigned integer, a floating
    /// point number, or a boolean value.
    Scalar(ScalarType),
    /// A collection of scalars.
    Vector(VectorType),
    /// A collection of vectors.
    Matrix(MatrixType),
    /// An image. In most cases, this variant is elevated to
    /// `CombinedImageSampler`, `SampledImage`, or `StorageImage` so you
    /// shouldn't see this in your reflection results.
    Image(ImageType),
    /// A sampled image externally combined with a sampler state. Such design is
    /// preferred in OpenGL and Vulkan.
    CombinedImageSampler(CombinedImageSamplerType),
    /// A sampled image yet to be combined with a `Sampler`. Such design is
    /// preferred in DirectX and Vulkan.
    SampledImage(SampledImageType),
    /// A storage image that shaders can read and/or write.
    StorageImage(StorageImageType),
    /// Separable sampler state.
    Sampler(SamplerType),
    /// Pixel store from input attachments.
    SubpassData(SubpassDataType),
    /// Repetition of a single type.
    Array(ArrayType),
    /// Aggregation of types.
    Struct(StructType),
    /// Acceleration structure for ray-tracing. Only available with
    /// `RayTracingKHR` capability enabled.
    AccelStruct(AccelStructType),
    /// Forward-declared pointer but the type of pointed data is unknown.
    /// Usually used for GPU linked lists. See `VK_KHR_buffer_device_address`.
    DeviceAddress(DeviceAddressType),
    /// Forward-declared pointer. Usually used for bindless resources with the
    /// `buffer_reference` extension. See `VK_KHR_buffer_device_address`.
    DevicePointer(PointerType),
    /// Ray query payload type (`rayQueryEXT`). Only available with `RayQueryKHR`
    /// capability enabled.
    RayQuery(RayQueryType),
}
impl SpirvType for Type {
    fn min_size(&self) -> Option<usize> {
        match self {
            Type::Scalar(x) => x.min_size(),
            Type::Vector(x) => x.min_size(),
            Type::Matrix(x) => x.min_size(),
            Type::Image(x) => x.min_size(),
            Type::CombinedImageSampler(x) => x.min_size(),
            Type::SampledImage(x) => x.min_size(),
            Type::StorageImage(x) => x.min_size(),
            Type::Sampler(x) => x.min_size(),
            Type::SubpassData(x) => x.min_size(),
            Type::Array(x) => x.min_size(),
            Type::Struct(x) => x.min_size(),
            Type::AccelStruct(x) => x.min_size(),
            Type::DeviceAddress(x) => x.min_size(),
            Type::DevicePointer(x) => x.min_size(),
            Type::RayQuery(x) => x.min_size(),
        }
    }
    fn size(&self) -> Option<usize> {
        match self {
            Type::Scalar(x) => x.size(),
            Type::Vector(x) => x.size(),
            Type::Matrix(x) => x.size(),
            Type::Image(x) => x.size(),
            Type::CombinedImageSampler(x) => x.size(),
            Type::SampledImage(x) => x.size(),
            Type::StorageImage(x) => x.size(),
            Type::Sampler(x) => x.size(),
            Type::SubpassData(x) => x.size(),
            Type::Array(x) => x.size(),
            Type::Struct(x) => x.size(),
            Type::AccelStruct(x) => x.size(),
            Type::DeviceAddress(x) => x.size(),
            Type::DevicePointer(x) => x.size(),
            Type::RayQuery(x) => x.size(),
        }
    }
    fn member_offset(&self, member_index: usize) -> Option<usize> {
        match self {
            Type::Scalar(x) => x.member_offset(member_index),
            Type::Vector(x) => x.member_offset(member_index),
            Type::Matrix(x) => x.member_offset(member_index),
            Type::Image(x) => x.member_offset(member_index),
            Type::CombinedImageSampler(x) => x.member_offset(member_index),
            Type::SampledImage(x) => x.member_offset(member_index),
            Type::StorageImage(x) => x.member_offset(member_index),
            Type::Sampler(x) => x.member_offset(member_index),
            Type::SubpassData(x) => x.member_offset(member_index),
            Type::Array(x) => x.member_offset(member_index),
            Type::Struct(x) => x.member_offset(member_index),
            Type::AccelStruct(x) => x.member_offset(member_index),
            Type::DeviceAddress(x) => x.member_offset(member_index),
            Type::DevicePointer(x) => x.member_offset(member_index),
            Type::RayQuery(x) => x.member_offset(member_index),
        }
    }
    fn access_ty(&self) -> Option<AccessType> {
        match self {
            Type::Scalar(x) => x.access_ty(),
            Type::Vector(x) => x.access_ty(),
            Type::Matrix(x) => x.access_ty(),
            Type::Image(x) => x.access_ty(),
            Type::CombinedImageSampler(x) => x.access_ty(),
            Type::SampledImage(x) => x.access_ty(),
            Type::StorageImage(x) => x.access_ty(),
            Type::Sampler(x) => x.access_ty(),
            Type::SubpassData(x) => x.access_ty(),
            Type::Array(x) => x.access_ty(),
            Type::Struct(x) => x.access_ty(),
            Type::AccelStruct(x) => x.access_ty(),
            Type::DeviceAddress(x) => x.access_ty(),
            Type::DevicePointer(x) => x.access_ty(),
            Type::RayQuery(x) => x.access_ty(),
        }
    }
}
impl Type {
    pub fn access_ty(&self) -> Option<AccessType> {
        use Type::*;
        match self {
            Scalar(_) => None,
            Vector(_) => None,
            Matrix(_) => None,
            Image(_) => None,
            Sampler(_) => None,
            CombinedImageSampler(_) => None,
            SampledImage(_) => None,
            StorageImage(_) => None,
            SubpassData(_) => None,
            Array(x) => x.element_ty.access_ty(),
            Struct(x) => x.members.iter().fold(None, |seed, x| match seed {
                None => Some(x.access_ty),
                Some(AccessType::ReadOnly) => match x.access_ty {
                    AccessType::ReadOnly => Some(AccessType::ReadOnly),
                    _ => Some(AccessType::ReadWrite),
                },
                Some(AccessType::WriteOnly) => match x.access_ty {
                    AccessType::WriteOnly => Some(AccessType::WriteOnly),
                    _ => Some(AccessType::ReadWrite),
                },
                Some(AccessType::ReadWrite) => Some(AccessType::ReadWrite),
            }),
            AccelStruct(_) => None,
            DeviceAddress(_) => None,
            DevicePointer(x) => x.pointee_ty.access_ty(),
            RayQuery(_) => None,
        }
    }
    // Iterate over all entries in the type tree.
    pub fn walk<'a>(&'a self) -> Walk<'a> {
        Walk::new(self)
    }
    declr_ty_accessor! {
        [Type]
        is_scalar -> Scalar,
        is_vector -> Vector,
        is_matrix -> Matrix,
        is_image -> Image,
        is_sampler -> Sampler,
        is_combined_image_sampler -> CombinedImageSampler,
        is_sampled_image -> SampledImage,
        is_storage_image -> StorageImage,
        is_subpass_data -> SubpassData,
        is_array -> Array,
        is_struct -> Struct,
        is_accel_struct -> AccelStruct,
        is_device_address -> DeviceAddress,
        is_device_pointer -> DevicePointer,
    }
    declr_ty_downcast! {
        [Type]
        as_scalar -> Scalar(ScalarType),
        as_vector -> Vector(VectorType),
        as_matrix -> Matrix(MatrixType),
        as_image -> Image(ImageType),
        as_sampler -> Sampler(SamplerType),
        as_combined_image_sampler -> CombinedImageSampler(CombinedImageSamplerType),
        as_sampled_image -> SampledImage(SampledImageType),
        as_storage_image -> StorageImage(StorageImageType),
        as_subpass_data -> SubpassData(SubpassDataType),
        as_array -> Array(ArrayType),
        as_struct -> Struct(StructType),
        as_accel_struct -> AccelStruct(AccelStructType),
        as_device_address -> DeviceAddress(DeviceAddressType),
        as_device_pointer -> DevicePointer(PointerType),
    }
    fn mutate_impl<F: Fn(Type) -> Type>(self, f: Rc<F>) -> Type {
        use Type::*;
        let out = match self {
            Array(src) => {
                let dst = ArrayType {
                    element_ty: Box::new(src.element_ty.mutate_impl(f.clone())),
                    element_count: src.element_count,
                    stride: src.stride,
                };
                Type::Array(dst)
            }
            Struct(src) => {
                let dst = StructType {
                    name: src.name,
                    members: src
                        .members
                        .into_iter()
                        .map(|x| StructMember {
                            name: x.name,
                            offset: x.offset,
                            ty: x.ty.mutate_impl(f.clone()),
                            access_ty: x.access_ty,
                        })
                        .collect(),
                };
                Type::Struct(dst)
            }
            _ => self,
        };
        (*f)(out)
    }
    pub fn mutate<F: Fn(Type) -> Type>(self, f: F) -> Type {
        self.mutate_impl(Rc::new(f))
    }
}
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Scalar(x) => x.fmt(f),
            Type::Vector(x) => x.fmt(f),
            Type::Matrix(x) => x.fmt(f),
            Type::Image(x) => x.fmt(f),
            Type::Sampler(x) => x.fmt(f),
            Type::CombinedImageSampler(x) => x.fmt(f),
            Type::SampledImage(x) => x.fmt(f),
            Type::StorageImage(x) => x.fmt(f),
            Type::SubpassData(x) => x.fmt(f),
            Type::Array(x) => x.fmt(f),
            Type::Struct(x) => x.fmt(f),
            Type::AccelStruct(x) => x.fmt(f),
            Type::DeviceAddress(x) => x.fmt(f),
            Type::DevicePointer(x) => x.fmt(f),
            Type::RayQuery(x) => x.fmt(f),
        }
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
    StorageImage(AccessType),
    /// `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`.
    UniformTexelBuffer,
    /// `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`.
    StorageTexelBuffer(AccessType),
    /// `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or
    /// `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` depending on how you gonna
    /// use it.
    UniformBuffer,
    /// `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or
    /// `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` depending on how you gonna
    /// use it.
    StorageBuffer(AccessType),
    /// `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` and its input attachment index.
    InputAttachment(u32),
    /// `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR`
    AccelStruct,
}
