//! Structured representations of SPIR-V types.
use crate::walk::Walk;
use crate::AccessType;
use std::fmt;
use std::hash::Hash;
use std::rc::Rc;

use spirv::Dim;
pub use spirv::ImageFormat;

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
    /// Signed integer.
    Signed(u32),
    /// Unsigned integer.
    Unsigned(u32),
    /// IEEE 754 floating-point number.
    Float(u32),
}
impl ScalarType {
    pub fn void() -> ScalarType {
        Self::Void
    }
    pub fn boolean() -> ScalarType {
        Self::Boolean
    }
    pub fn int(nbyte: u32, is_signed: bool) -> ScalarType {
        if is_signed {
            Self::Signed(nbyte)
        } else {
            Self::Unsigned(nbyte)
        }
    }
    pub fn float(nbyte: u32) -> ScalarType {
        Self::Float(nbyte)
    }
    /// Whether the scalar type is signed. A floating-point type is always
    /// signed. A boolean type is not Scalar so it's neither signed or
    /// unsigned, represented by a `None`.
    pub fn is_signed(&self) -> Option<bool> {
        match self {
            Self::Void => None,
            Self::Boolean => None,
            Self::Signed(_) => Some(true),
            Self::Unsigned(_) => Some(false),
            Self::Float(_) => Some(true),
        }
    }
    /// Number of bytes an instance of the type takes.
    pub fn nbyte(&self) -> usize {
        let nbyte = match self {
            Self::Void => 0,
            Self::Boolean => 1,
            Self::Signed(nbyte) => *nbyte,
            Self::Unsigned(nbyte) => *nbyte,
            Self::Float(nbyte) => *nbyte,
        };
        nbyte as usize
    }

    pub fn is_boolean(&self) -> bool {
        if let Self::Boolean = self {
            true
        } else {
            false
        }
    }
    pub fn is_sint(&self) -> bool {
        if let Self::Signed(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_uint(&self) -> bool {
        if let Self::Unsigned(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_float(&self) -> bool {
        if let Self::Float(_) = self {
            true
        } else {
            false
        }
    }
}
impl fmt::Display for ScalarType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Void => f.write_str("void"),
            Self::Boolean => f.write_str("bool"),
            Self::Signed(nbyte) => write!(f, "i{}", nbyte << 3),
            Self::Unsigned(nbyte) => write!(f, "u{}", nbyte << 3),
            Self::Float(nbyte) => write!(f, "f{}", nbyte << 3),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct VectorType {
    /// Vector scalar type.
    pub scalar_ty: ScalarType,
    /// Number of scalar components in the vector.
    pub nscalar: u32,
}
impl VectorType {
    pub fn new(scalar_ty: ScalarType, nscalar: u32) -> VectorType {
        VectorType {
            scalar_ty: scalar_ty,
            nscalar: nscalar,
        }
    }
    pub fn nbyte(&self) -> usize {
        self.nscalar as usize * self.scalar_ty.nbyte()
    }
}
impl fmt::Display for VectorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "vec{}<{}>", self.nscalar, self.scalar_ty)
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
    pub vec_ty: VectorType,
    /// Number of vectors in the matrix.
    pub nvec: u32,
    /// Axis order of the matrix. Valid SPIR-V never gives a `None` major.
    pub major: Option<MatrixAxisOrder>,
    /// Stride between vectors in the matrix. Valid SPIR-V never gives a `None`
    /// stride.
    pub stride: Option<usize>,
}
impl MatrixType {
    pub fn new(vec_ty: VectorType, nvec: u32) -> MatrixType {
        MatrixType {
            vec_ty: vec_ty,
            nvec: nvec,
            major: None,
            stride: None,
        }
    }
    /// Get the number of bytes the matrix physically occupied.
    pub fn nbyte(&self) -> usize {
        self.nvec as usize * self.stride.unwrap_or(self.vec_ty.nbyte())
    }
}
impl fmt::Display for MatrixType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let major = match self.major {
            Some(MatrixAxisOrder::ColumnMajor) => "ColumnMajor",
            Some(MatrixAxisOrder::RowMajor) => "RowMajor",
            None => "AxisOrder?",
        };
        let nrow = self.vec_ty.nscalar;
        let ncol = self.nvec;
        let scalar_ty = &self.vec_ty.scalar_ty;
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
    /// Whether  the image is an array of images. In Vulkan, it means that the
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
pub struct CombinedImageSamplerType {
    pub sampled_img_ty: SampledImageType,
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
    pub proto_ty: Box<Type>,
    pub nrepeat: Option<u32>,
    pub stride: Option<usize>,
}
impl ArrayType {
    pub(crate) fn new_multibind(proto_ty: &Type, nrepeat: u32) -> ArrayType {
        ArrayType {
            proto_ty: Box::new(proto_ty.clone()),
            nrepeat: Some(nrepeat),
            stride: None,
        }
    }
    pub(crate) fn new_unsized_multibind(proto_ty: &Type) -> ArrayType {
        ArrayType {
            proto_ty: Box::new(proto_ty.clone()),
            nrepeat: None,
            stride: None,
        }
    }
    pub fn new(proto_ty: &Type, nrepeat: u32, stride: usize) -> ArrayType {
        ArrayType {
            proto_ty: Box::new(proto_ty.clone()),
            nrepeat: Some(nrepeat),
            stride: Some(stride),
        }
    }
    pub fn new_unsized(proto_ty: &Type, stride: usize) -> ArrayType {
        ArrayType {
            proto_ty: Box::new(proto_ty.clone()),
            nrepeat: None,
            stride: Some(stride),
        }
    }

    /// Get the minimum size of the array type. If the number of elements is not
    /// given until runtime, 0 is returned.
    pub fn nbyte(&self) -> usize {
        self.stride.unwrap_or_default() * self.nrepeat.unwrap_or_default() as usize
    }
}
impl fmt::Display for ArrayType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(nrepeat) = self.nrepeat {
            write!(f, "[{}; {}]", self.proto_ty, nrepeat)
        } else {
            write!(f, "[{}]", self.proto_ty)
        }
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct StructMember {
    pub name: Option<String>,
    pub offset: usize,
    pub ty: Type,
    pub access_ty: AccessType,
}
#[derive(PartialEq, Eq, Default, Clone, Hash, Debug)]
pub struct StructType {
    pub name: Option<String>,
    pub members: Vec<StructMember>, // Offset and type.
}
impl StructType {
    pub(crate) fn new(name: Option<String>) -> StructType {
        StructType {
            name,
            ..Default::default()
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(AsRef::as_ref)
    }
    pub fn nbyte(&self) -> usize {
        self.members
            .last()
            .map(|last| last.offset + last.ty.nbyte().unwrap_or(0))
            .unwrap_or(0)
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

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct PointerType {
    pub pointee_ty: Box<Type>,
}
impl PointerType {
    pub(crate) fn new(pointee_ty: &Type) -> PointerType {
        PointerType {
            pointee_ty: Box::new(pointee_ty.clone()),
        }
    }
}
impl fmt::Display for PointerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Pointer { ")?;
        write!(f, "{}", *self.pointee_ty)?;
        f.write_str(" }")
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
    Sampler(),
    /// Pixel store from input attachments.
    SubpassData(SubpassDataType),
    /// Repetition of a single type.
    Array(ArrayType),
    /// Aggregation of types.
    Struct(StructType),
    /// Acceleration structure for ray-tracing. Only available with
    /// `RayTracingKHR` capability enabled.
    AccelStruct(),
    /// Forward-declared pointer but the type of pointed data is unknown.
    /// Usually used for GPU linked lists. See `VK_KHR_buffer_device_address`.
    DeviceAddress(),
    /// Forward-declared pointer. Usually used for bindless resources with the
    /// `buffer_reference` extension. See `VK_KHR_buffer_device_address`.
    DevicePointer(PointerType),
    /// Ray query payload type (`rayQueryEXT`). Only available with `RayQueryKHR`
    /// capability enabled.
    RayQuery(),
}
impl Type {
    pub fn nbyte(&self) -> Option<usize> {
        use Type::*;
        match self {
            Scalar(x) => Some(x.nbyte()),
            Vector(x) => Some(x.nbyte()),
            Matrix(x) => Some(x.nbyte()),
            Image(_) => None,
            Sampler() => None,
            CombinedImageSampler(_) => None,
            SampledImage(_) => None,
            StorageImage(_) => None,
            SubpassData(_) => None,
            Array(x) => Some(x.nbyte()),
            Struct(x) => Some(x.nbyte()),
            AccelStruct() => None,
            DeviceAddress() => Some(8),
            DevicePointer(_) => Some(8),
            RayQuery() => None,
        }
    }
    pub fn access_ty(&self) -> Option<AccessType> {
        use Type::*;
        match self {
            Scalar(_) => None,
            Vector(_) => None,
            Matrix(_) => None,
            Image(_) => None,
            Sampler() => None,
            CombinedImageSampler(_) => None,
            SampledImage(_) => None,
            StorageImage(_) => None,
            SubpassData(_) => None,
            Array(x) => x.proto_ty.access_ty(),
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
            AccelStruct() => None,
            DeviceAddress() => None,
            DevicePointer(x) => x.pointee_ty.access_ty(),
            RayQuery() => None,
        }
    }
    // Iterate over all entries in the type tree.
    pub fn walk<'a>(&'a self) -> Walk<'a> {
        Walk::new(self)
    }
    declr_ty_accessor! {
        [Type]
        is_scalar -> Scalar,
        is_vec -> Vector,
        is_mat -> Matrix,
        is_img -> Image,
        is_sampler -> Sampler,
        is_combined_img_sampler -> CombinedImageSampler,
        is_sampled_img -> SampledImage,
        is_storage_img -> StorageImage,
        is_subpass_data -> SubpassData,
        is_arr -> Array,
        is_struct -> Struct,
        is_accel_struct -> AccelStruct,
        is_devaddr -> DeviceAddress,
        is_devptr -> DevicePointer,
    }
    declr_ty_downcast! {
        [Type]
        as_scalar -> Scalar(ScalarType),
        as_vec -> Vector(VectorType),
        as_mat -> Matrix(MatrixType),
        as_img -> Image(ImageType),
        as_combined_img_sampler -> CombinedImageSampler(CombinedImageSamplerType),
        as_sampled_img -> SampledImage(SampledImageType),
        as_storage_img -> StorageImage(StorageImageType),
        as_subpass_data -> SubpassData(SubpassDataType),
        as_arr -> Array(ArrayType),
        as_struct -> Struct(StructType),
        as_devptr -> DevicePointer(PointerType),
    }
    fn mutate_impl<F: Fn(Type) -> Type>(self, f: Rc<F>) -> Type {
        use Type::*;
        let out = match self {
            Array(src) => {
                let dst = ArrayType {
                    proto_ty: Box::new(src.proto_ty.mutate_impl(f.clone())),
                    nrepeat: src.nrepeat,
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
            Type::Sampler() => f.write_str("Sampler"),
            Type::CombinedImageSampler(x) => x.fmt(f),
            Type::SampledImage(x) => x.fmt(f),
            Type::StorageImage(x) => x.fmt(f),
            Type::SubpassData(x) => x.fmt(f),
            Type::Array(x) => x.fmt(f),
            Type::Struct(x) => x.fmt(f),
            Type::AccelStruct() => f.write_str("AccelStruct"),
            Type::DeviceAddress() => f.write_str("Address"),
            Type::DevicePointer(ptr_ty) => ptr_ty.fmt(f),
            Type::RayQuery() => f.write_str("RayQuery"),
        }
    }
}
