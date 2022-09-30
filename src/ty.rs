//! Structured representations of SPIR-V types.
use std::fmt;
use std::hash::Hash;
use std::rc::Rc;
use crate::error::*;
use crate::walk::Walk;

use spirv_headers::Dim;
pub use spirv_headers::{ImageFormat};

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum ScalarType {
    // Be careful with booleans. Booleans is NOT allowed to be exposed to the
    // host according to the SPIR-V specification.
    Boolean,
    Signed(u32),
    Unsigned(u32),
    Float(u32),
}
impl ScalarType {
    pub fn boolean() -> ScalarType {
        Self::Boolean
    }
    pub fn int(nbyte: u32, is_signed: bool) -> ScalarType {
        if is_signed { Self::Signed(nbyte) } else { Self::Unsigned(nbyte) }
    }
    pub fn float(nbyte: u32) -> ScalarType {
        Self::Float(nbyte)
    }
    /// Whether the scalar type is signed. A floating-point type is always
    /// signed. A boolean type is not Scalar so it's neither signed or
    /// unsigned, represented by a `None`.
    pub fn is_signed(&self) -> Option<bool> {
        match self {
            Self::Boolean => None,
            Self::Signed(_) => Some(true),
            Self::Unsigned(_) => Some(false),
            Self::Float(_) => Some(true),
        }
    }
    /// Number of bytes an instance of the type takes.
    pub fn nbyte(&self) -> usize {
        let nbyte = match self {
            Self::Boolean => 1,
            Self::Signed(nbyte) => *nbyte,
            Self::Unsigned(nbyte) => *nbyte,
            Self::Float(nbyte) => *nbyte,
        };
        nbyte as usize
    }

    pub fn is_boolean(&self) -> bool {
        if let Self::Boolean = self { true } else { false }
    }
    pub fn is_sint(&self) -> bool {
        if let Self::Signed(_) = self { true } else { false }
    }
    pub fn is_uint(&self) -> bool {
        if let Self::Unsigned(_) = self { true } else { false }
    }
    pub fn is_float(&self) -> bool {
        if let Self::Float(_) = self { true } else { false }
    }
}
impl fmt::Debug for ScalarType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean => write!(f, "bool"),
            Self::Signed(nbyte) => write!(f, "i{}", nbyte << 3),
            Self::Unsigned(nbyte) => write!(f, "u{}", nbyte << 3),
            Self::Float(nbyte) => write!(f, "f{}", nbyte << 3),
        }
    }
}


#[derive(PartialEq, Eq, Hash, Clone)]
pub struct VectorType {
    /// Vector scalar type.
    pub scalar_ty: ScalarType,
    /// Number of scalar components in the vector.
    pub nscalar: u32,
}
impl VectorType {
    pub fn new(scalar_ty: ScalarType, nscalar: u32) -> VectorType {
        VectorType { scalar_ty: scalar_ty, nscalar: nscalar }
    }
    pub fn nbyte(&self) -> usize {
        self.nscalar as usize * self.scalar_ty.nbyte()
    }
}
impl fmt::Debug for VectorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "vec{}<{:?}>", self.nscalar, self.scalar_ty)
    }
}


#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum MatrixAxisOrder {
    ColumnMajor,
    RowMajor,
}
impl Default for MatrixAxisOrder {
    fn default() -> MatrixAxisOrder { MatrixAxisOrder::ColumnMajor }
}


#[derive(PartialEq, Eq, Hash, Clone)]
pub struct MatrixType {
    /// Matrix vector type.
    pub vec_ty: VectorType,
    /// Number of vectors in the matrix.
    pub nvec: u32,
    /// Stride between vectors in the matrix. Valid SPIR-V never gives a `None`
    /// stride.
    pub stride: Option<usize>,
    /// Axis order of the matrix. Valid SPIR-V never gives a `None` major.
    pub major: Option<MatrixAxisOrder>,
}
impl MatrixType {
    pub fn new(vec_ty: VectorType, nvec: u32) -> MatrixType {
        MatrixType {
            vec_ty: vec_ty,
            nvec: nvec,
            stride: None,
            major: None,
        }
    }
    /// Get the number of bytes the matrix physically occupied.
    pub fn nbyte(&self) -> usize {
        self.nvec as usize * self.stride.unwrap_or(self.vec_ty.nbyte())
    }
}
impl fmt::Debug for MatrixType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let major_lit = match self.major {
            Some(MatrixAxisOrder::ColumnMajor) => "ColumnMajor",
            Some(MatrixAxisOrder::RowMajor) => "RowMajor",
            None => "AxisOrder?",
        };
        let nrow = self.vec_ty.nscalar;
        let ncol = self.nvec;
        let scalar_ty = &self.vec_ty.scalar_ty;
        write!(f, "mat{}x{}<{:?},{}>", nrow, ncol, scalar_ty, major_lit)
    }
}


#[non_exhaustive]
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum ImageArrangement {
    Image1D,
    Image2D,
    Image2DMS,
    Image3D,
    CubeMap,
    Image1DArray,
    Image2DArray,
    Image2DMSArray,
    CubeMapArray,
    Image2DRect,
    ImageBuffer,
}
impl ImageArrangement {
    /// Do note this dim is not the number of dimensions but a enumeration of
    /// values specified in SPIR-V specification.
    pub fn from_spv_def(dim: Dim, is_array: bool, is_multisampled: bool) -> Result<ImageArrangement> {
        let arng = match (dim, is_array, is_multisampled) {
            (Dim::Dim1D, false, false) => ImageArrangement::Image1D,
            (Dim::Dim1D, true, false) => ImageArrangement::Image1DArray,
            (Dim::Dim2D, false, false) => ImageArrangement::Image2D,
            (Dim::Dim2D, false, true) => ImageArrangement::Image2DMS,
            (Dim::Dim2D, true, false) => ImageArrangement::Image2DArray,
            (Dim::Dim2D, true, true) => ImageArrangement::Image2DMSArray,
            (Dim::Dim3D, false, false) => ImageArrangement::Image3D,
            (Dim::Dim3D, true, false) => ImageArrangement::Image3D,
            (Dim::DimCube, false, false) => ImageArrangement::CubeMap,
            (Dim::DimCube, true, false) => ImageArrangement::CubeMapArray,
            (Dim::DimRect, false, false) => ImageArrangement::Image2DRect,
            (Dim::DimBuffer, false, false) => ImageArrangement::ImageBuffer,
            _ => return Err(Error::UNSUPPORTED_IMG_CFG),
        };
        Ok(arng)
    }
}


#[derive(PartialEq, Eq, Hash, Clone)]
pub struct ImageType {
    /// Scalar type of image access result. In most cases it's `Some`, but the
    /// SPIR-V specification allows it to be `OpTypeVoid`. I have never
    /// encounter one tho.
    pub scalar_ty: Option<ScalarType>,
    /// Whether the image is sampled, or `None` if it's unknown at compile time.
    pub is_sampled: Option<bool>,
    /// Whether the image is a depth image, or `None` if it's unknown at compile time.
    pub is_depth: Option<bool>,
    /// Matches `VkImageCreateInfo::format`. Can be `ImageFormat::Unknown` in
    /// case of a sampled image.
    pub fmt: ImageFormat,
    /// Image arrangement which encodes multisampling, array-ness and
    /// dimentionality.
    pub arng: ImageArrangement,
}
impl fmt::Debug for ImageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let scalar_ty = {
            let scalar_ty = self.scalar_ty.as_ref();
            if let Some(scalar_ty) = scalar_ty {
                format!("{:?}", scalar_ty)
            } else {
                "Void".to_owned()
            }
        };
        let sampled_lit = match self.is_sampled {
            Some(true) => "Sampled",
            Some(false) => "Storage",
            None => "Sampled?",
        };
        let depth_lit = match self.is_depth {
            Some(true) => "Color",
            Some(false) => "Depth",
            None => "Depth?",
        };
        write!(f, "Image<{},{},{},{:?},{:?}>", scalar_ty, sampled_lit, depth_lit, self.fmt, self.arng)
    }
}


#[derive(PartialEq, Eq, Hash, Clone)]
pub struct SampledImageType {
    pub img_ty: ImageType,
}
impl SampledImageType {
    pub fn new(img_ty: ImageType) -> SampledImageType {
        SampledImageType { img_ty }
    }
}
impl fmt::Debug for SampledImageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sampled<{:?}>", self.img_ty)
    }
}


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum SubpassDataArrangement {
    SubpassData,
    SubpassDataMS,
}
impl SubpassDataArrangement {
    /// Do note this dim is not the number of dimensions but a enumeration of
    /// values specified in SPIR-V specification.
    pub fn from_spv_def(is_multisampled: bool) -> Result<SubpassDataArrangement> {
        let arng = match is_multisampled {
            false => SubpassDataArrangement::SubpassData,
            true => SubpassDataArrangement::SubpassDataMS,
        };
        Ok(arng)
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct SubpassDataType {
    /// Scalar type of subpass data access result. In most cases it's `Some`,
    /// but the SPIR-V specification allows it to be `OpTypeVoid`. I have never
    /// encounter one tho.
    pub scalar_ty: Option<ScalarType>,
    /// Image arrangement which encodes multisampling state.
    pub arng: SubpassDataArrangement,
}
impl SubpassDataType {
    pub fn new(scalar_ty: Option<ScalarType>, arng: SubpassDataArrangement) -> Self {
        SubpassDataType { scalar_ty, arng }
    }
}
impl fmt::Debug for SubpassDataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let scalar_ty = {
            let scalar_ty = self.scalar_ty.as_ref();
            if let Some(scalar_ty) = scalar_ty {
                format!("{:?}", scalar_ty)
            } else {
                "Void".to_owned()
            }
        };
        write!(f, "SubpassData<{},{:?}>", scalar_ty, self.arng)
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct ArrayType {
    pub(crate) proto_ty: Box<Type>,
    pub(crate) nrepeat: Option<u32>,
    pub(crate) stride: Option<usize>,
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
            stride: Some(stride)
        }
    }

    /// Get the minimum size of the array type. If the number of elements is not
    /// given until runtime, 0 is returned.
    pub fn nbyte(&self) -> usize {
        self.stride.unwrap_or_default() * self.nrepeat().unwrap_or_default() as usize
    }
    pub fn proto_ty(&self) -> &Type {
        &self.proto_ty
    }
    pub fn stride(&self) -> usize {
        // Multibind which makes the `stride` be `None` is used internally only.
        self.stride.unwrap()
    }
    pub fn nrepeat(&self) -> Option<u32> {
        self.nrepeat.clone()
    }
}
impl fmt::Debug for ArrayType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(nrepeat) = self.nrepeat {
            write!(f, "[{:?}; {}]", self.proto_ty, nrepeat)
        } else {
            write!(f, "[{:?}]", self.proto_ty)
        }
    }
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct StructMember {
    pub name: Option<String>,
    pub offset: usize,
    pub ty: Type,
}
#[derive(PartialEq, Eq, Default, Clone, Hash)]
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
        self.members.last()
            .map(|last| last.offset + last.ty.nbyte().unwrap_or(0))
            .unwrap_or(0)
    }
}
impl fmt::Debug for StructType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "{} {{ ", name)?;
        } else {
            f.write_str("{ ")?;
        }
        for (i, member) in self.members.iter().enumerate() {
            if i != 0 { f.write_str(", ")?; }
            if let Some(name) = &member.name {
                write!(f, "{}: {:?}", name, member.ty)?;
            } else {
                write!(f, "{}: {:?}", i, member.ty)?;
            }
        }
        f.write_str(" }")
    }
}

#[derive(PartialEq, Eq, Clone, Hash)]
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
impl fmt::Debug for PointerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Pointer { ")?;
        write!(f, "{:?}", *self.pointee_ty)?;
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


#[derive(PartialEq, Eq, Hash, Clone)]
#[non_exhaustive]
pub enum Type {
    /// Literally nothing. You shouldn't find this in reflection results.
    Void(),
    /// A single value, which can be a signed or unsigned integer, a floating
    /// point number, or a boolean value.
    Scalar(ScalarType),
    /// A collection of scalars.
    Vector(VectorType),
    /// A collection of vectors.
    Matrix(MatrixType),
    /// An unsampled image, with no sampler state combined. Such design is
    /// preferred in DirectX.
    Image(ImageType),
    /// A sampled image externally combined with a sampler state. Such design is
    /// preferred in legacy OpenGL.
    SampledImage(SampledImageType),
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
}
impl Type {
    pub fn nbyte(&self) -> Option<usize> {
        use Type::*;
        match self {
            Void() => None,
            Scalar(scalar_ty) => Some(scalar_ty.nbyte()),
            Vector(vec_ty) => Some(vec_ty.nbyte()),
            Matrix(mat_ty) => Some(mat_ty.nbyte()),
            Image(_) => None,
            Sampler() => None,
            SampledImage(_) => None,
            SubpassData(_) => None,
            Array(arr_ty) => Some(arr_ty.nbyte()),
            Struct(struct_ty) => Some(struct_ty.nbyte()),
            AccelStruct() => None,
            DeviceAddress() => Some(8),
            DevicePointer(_) => Some(8),
        }
    }
    // Iterate over all entries in the type tree.
    pub fn walk<'a>(&'a self) -> Walk<'a> { Walk::new(self) }
    declr_ty_accessor! {
        [Type]
        is_void -> Void,
        is_scalar -> Scalar,
        is_vec -> Vector,
        is_mat -> Matrix,
        is_img -> Image,
        is_samper -> Sampler,
        is_sampled_img -> SampledImage,
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
        as_sampled_img -> SampledImage(SampledImageType),
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
            },
            Struct(src) => {
                let dst = StructType {
                    name: src.name,
                    members: src.members.into_iter()
                        .map(|x| {
                            StructMember {
                                name: x.name,
                                offset: x.offset,
                                ty: x.ty.mutate_impl(f.clone()),
                            }
                        })
                        .collect(),
                };
                Type::Struct(dst)
            },
            _ => self,
        };
        (*f)(out)
    }
    pub fn mutate<F: Fn(Type) -> Type>(self, f: F) -> Type {
        self.mutate_impl(Rc::new(f))
    }
}
impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Void() => write!(f, "Void"),
            Type::Scalar(scalar_ty) => scalar_ty.fmt(f),
            Type::Vector(vec_ty) => vec_ty.fmt(f),
            Type::Matrix(mat_ty) => mat_ty.fmt(f),
            Type::Image(img_ty) => write!(f, "{:?}", img_ty),
            Type::Sampler() => write!(f, "Sampler"),
            Type::SampledImage(sampled_img_ty) => sampled_img_ty.fmt(f),
            Type::SubpassData(subpass_data_ty) => subpass_data_ty.fmt(f),
            Type::Array(arr_ty) => arr_ty.fmt(f),
            Type::Struct(struct_ty) => struct_ty.fmt(f),
            Type::AccelStruct() => write!(f, "AccelStruct"),
            Type::DeviceAddress() => write!(f, "Address"),
            Type::DevicePointer(ptr_ty) => ptr_ty.fmt(f),
        }
    }
}
