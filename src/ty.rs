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
        let transpose = match self.major {
            Some(MatrixAxisOrder::ColumnMajor) => "",
            Some(MatrixAxisOrder::RowMajor) => "T",
            None => "?",
        };
        let nrow = self.vec_ty.nscalar;
        let ncol = self.nvec;
        let scalar_ty = &self.vec_ty.scalar_ty;
        write!(f, "mat{}x{}{}<{:?}>", nrow, ncol, transpose, scalar_ty)
    }
}


#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum ImageUnitFormat {
    /// The image is used as a storage image and the read/write format is
    /// explicitly specified.
    Color(ImageFormat),
    /// The image is used as a sampled image.
    Sampled,
    /// The image is used as a sampled depth image. Note that you cannot access
    /// depth-stencil images by read/write.
    Depth,
}
impl ImageUnitFormat {
    pub fn from_spv_def(is_sampled: u32, is_depth: u32, color_fmt: ImageFormat) -> Result<ImageUnitFormat> {
        let img_unit_fmt = match (is_sampled, is_depth, color_fmt) {
            (1, 0, _) => ImageUnitFormat::Sampled,
            (1, 1, _) => ImageUnitFormat::Depth,
            (2, 0, color_fmt) => ImageUnitFormat::Color(color_fmt),
            _ => return Err(Error::UNSUPPORTED_IMG_CFG),
        };
        Ok(img_unit_fmt)
    }
}


#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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
    /// Matches `VkImageCreateInfo::format`.
    pub unit_fmt: ImageUnitFormat,
    /// Image arrangement which encodes multisampling, array-ness and
    /// dimentionality.
    pub arng: ImageArrangement,
}
impl ImageType {
    pub fn new(
        scalar_ty: Option<ScalarType>,
        unit_fmt: ImageUnitFormat,
        arng: ImageArrangement,
    ) -> ImageType {
        ImageType { scalar_ty, unit_fmt, arng }
    }
}
impl fmt::Debug for ImageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ImageArrangement::*;
        use ImageUnitFormat::*;
        let scalar_ty = || -> String {
            let scalar_ty = self.scalar_ty.as_ref();
            if let Some(scalar_ty) = scalar_ty {
                format!("{:?}", scalar_ty)
            } else {
                "void".to_owned()
            }
        };
        match (self.arng, self.unit_fmt) {
            (Image1D, Color(fmt)) => write!(f, "image1D<{:?}>", fmt),
            (Image2D, Color(fmt)) => write!(f, "image2D<{:?}>", fmt),
            (Image2DMS, Color(fmt)) => write!(f, "image2DMS<{:?}>", fmt),
            (Image3D, Color(fmt)) => write!(f, "image3D<{:?}>", fmt),
            (CubeMap, Color(fmt)) => write!(f, "imageCube<{:?}>", fmt),
            (Image1DArray, Color(fmt)) => write!(f, "image1DArray<{:?}>", fmt),
            (Image2DArray, Color(fmt)) => write!(f, "image2DArray<{:?}>", fmt),
            (Image2DMSArray, Color(fmt)) => write!(f, "image2DMSArray<{:?}>", fmt),
            (CubeMapArray, Color(fmt)) => write!(f, "imageCubeArray<{:?}>", fmt),
            (Image2DRect, Color(fmt)) => write!(f, "image2DRect<{:?}>", fmt),
            (ImageBuffer, Color(fmt)) => write!(f, "imageBuffer<{:?}>", fmt),

            (Image1D, Sampled) => write!(f, "texture1D<{}>", scalar_ty()),
            (Image2D, Sampled) => write!(f, "texture2D<{}>", scalar_ty()),
            (Image2DMS, Sampled) => write!(f, "texture2DMS<{}>", scalar_ty()),
            (Image3D, Sampled) => write!(f, "texture3D<{}>", scalar_ty()),
            (CubeMap, Sampled) => write!(f, "textureCube<{}>", scalar_ty()),
            (Image1DArray, Sampled) => write!(f, "texture1DArray<{}>", scalar_ty()),
            (Image2DArray, Sampled) => write!(f, "texture2DArray<{}>", scalar_ty()),
            (Image2DMSArray, Sampled) => write!(f, "texture2DMSArray<{}>", scalar_ty()),
            (CubeMapArray, Sampled) => write!(f, "textureCubeArray<{}>", scalar_ty()),
            (Image2DRect, Sampled) => write!(f, "texture2DRect<{}>", scalar_ty()),
            (ImageBuffer, Sampled) => write!(f, "textureBuffer<{}>", scalar_ty()),
            _ => Err(fmt::Error::default()),
        }
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
        use ImageArrangement::*;
        use ImageUnitFormat::*;
        let scalar_ty = {
            let scalar_ty = self.img_ty.scalar_ty.as_ref();
            if let Some(scalar_ty) = scalar_ty {
                format!("{:?}", scalar_ty)
            } else {
                "void".to_owned()
            }
        };
        match (self.img_ty.arng, self.img_ty.unit_fmt) {
            (Image1D, Sampled) => write!(f, "sampler1D<{}>", scalar_ty),
            (Image2D, Sampled) => write!(f, "sampler2D<{}>", scalar_ty),
            (Image2DMS, Sampled) => write!(f, "sampler2DMS<{}>", scalar_ty),
            (Image3D, Sampled) => write!(f, "sampler3D<{}>", scalar_ty),
            (CubeMap, Sampled) => write!(f, "samplerCube<{}>", scalar_ty),
            (Image1DArray, Sampled) => write!(f, "sampler1DArray<{}>", scalar_ty),
            (Image2DArray, Sampled) => write!(f, "sampler2DArray<{}>", scalar_ty),
            (Image2DMSArray, Sampled) => write!(f, "sampler2DMSArray<{}>", scalar_ty),
            (CubeMapArray, Sampled) => write!(f, "samplerCubeArray<{}>", scalar_ty),
            (Image2DRect, Sampled) => write!(f, "sampler2DRect<{}>", scalar_ty),
            (ImageBuffer, Sampled) => write!(f, "samplerBuffer<{}>", scalar_ty),

            (Image1D, Depth) => write!(f, "sampler1DShadow<{}>", scalar_ty),
            (Image2D, Depth) => write!(f, "sampler2DShadow<{}>", scalar_ty),
            (CubeMap, Depth) => write!(f, "samplerCubeShadow<{}>", scalar_ty),
            (Image1DArray, Depth) => write!(f, "sampler1DArrayShadow<{}>", scalar_ty),
            (Image2DArray, Depth) => write!(f, "sampler2DArrayShadow<{}>", scalar_ty),
            (CubeMapArray, Depth) => write!(f, "samplerCubeShadowArray<{}>", scalar_ty),
            (Image2DRect, Depth) => write!(f, "sampler2DRectShadow<{}>", scalar_ty),
            _ => Err(fmt::Error::default()),
        }
    }
}


#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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
                "void".to_owned()
            }
        };
        match self.arng {
            SubpassDataArrangement::SubpassData => {
                write!(f, "subpassData<{}>", scalar_ty)
            },
            SubpassDataArrangement::SubpassDataMS => {
                write!(f, "subpassDataMS<{}>", scalar_ty)
            },
        }
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
        f.write_str("<pointer> { ")?;
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
            Type::Void() => write!(f, "void"),
            Type::Scalar(scalar_ty) => scalar_ty.fmt(f),
            Type::Vector(vec_ty) => vec_ty.fmt(f),
            Type::Matrix(mat_ty) => mat_ty.fmt(f),
            Type::Image(img_ty) => write!(f, "{:?}", img_ty),
            Type::Sampler() => write!(f, "sampler"),
            Type::SampledImage(sampled_img_ty) => sampled_img_ty.fmt(f),
            Type::SubpassData(subpass_data_ty) => subpass_data_ty.fmt(f),
            Type::Array(arr_ty) => arr_ty.fmt(f),
            Type::Struct(struct_ty) => struct_ty.fmt(f),
            Type::AccelStruct() => write!(f, "accelerationStructure"),
            Type::DeviceAddress() => write!(f, "uint64_t"),
            Type::DevicePointer(ptr_ty) => ptr_ty.fmt(f),
        }
    }
}
