//! Structured representations of SPIR-V types.
use std::collections::BTreeMap;
use std::fmt;
use crate::MemberVariableResolution;
use crate::error::*;
use std::hash::{Hash, Hasher};

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
    pub scalar_ty: ScalarType,
    pub nscalar: u32,
}
impl VectorType {
    pub fn new(scalar_ty: ScalarType, nscalar: u32) -> VectorType {
        VectorType { scalar_ty: scalar_ty, nscalar: nscalar }
    }
    pub fn nbyte(&self) -> usize { self.nscalar as usize * self.scalar_ty.nbyte() }
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
    pub vec_ty: VectorType,
    pub nvec: u32,
    pub stride: usize,
    pub major: MatrixAxisOrder,
}
impl MatrixType {
    pub fn new(vec_ty: VectorType, nvec: u32) -> MatrixType {
        MatrixType {
            stride: vec_ty.nbyte(),
            vec_ty: vec_ty,
            nvec: nvec,
            major: MatrixAxisOrder::default(),
        }
    }
    pub(crate) fn decorate(&mut self, stride: usize, major: MatrixAxisOrder) {
        self.stride = stride;
        self.major = major;
    }
    pub fn nbyte(&self) -> usize { self.nvec as usize * self.stride }
}
impl fmt::Debug for MatrixType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let transpose = match self.major {
            MatrixAxisOrder::ColumnMajor => "",
            MatrixAxisOrder::RowMajor => "T",
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

            (Image1D, Sampled) => write!(f, "texture1D<{:?}>", self.scalar_ty),
            (Image2D, Sampled) => write!(f, "texture2D<{:?}>", self.scalar_ty),
            (Image2DMS, Sampled) => write!(f, "texture2DMS<{:?}>", self.scalar_ty),
            (Image3D, Sampled) => write!(f, "texture3D<{:?}>", self.scalar_ty),
            (CubeMap, Sampled) => write!(f, "textureCube<{:?}>", self.scalar_ty),
            (Image1DArray, Sampled) => write!(f, "texture1DArray<{:?}>", self.scalar_ty),
            (Image2DArray, Sampled) => write!(f, "texture2DArray<{:?}>", self.scalar_ty),
            (Image2DMSArray, Sampled) => write!(f, "texture2DMSArray<{:?}>", self.scalar_ty),
            (CubeMapArray, Sampled) => write!(f, "textureCubeArray<{:?}>", self.scalar_ty),
            (Image2DRect, Sampled) => write!(f, "texture2DRect<{:?}>", self.scalar_ty),
            (ImageBuffer, Sampled) => write!(f, "textureBuffer<{:?}>", self.scalar_ty),
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
        match (self.img_ty.arng, self.img_ty.unit_fmt) {
            (Image1D, Sampled) => write!(f, "sampler1D<{:?}>", self.img_ty.scalar_ty),
            (Image2D, Sampled) => write!(f, "sampler2D<{:?}>", self.img_ty.scalar_ty),
            (Image2DMS, Sampled) => write!(f, "sampler2DMS<{:?}>", self.img_ty.scalar_ty),
            (Image3D, Sampled) => write!(f, "sampler3D<{:?}>", self.img_ty.scalar_ty),
            (CubeMap, Sampled) => write!(f, "samplerCube<{:?}>", self.img_ty.scalar_ty),
            (Image1DArray, Sampled) => write!(f, "sampler1DArray<{:?}>", self.img_ty.scalar_ty),
            (Image2DArray, Sampled) => write!(f, "sampler2DArray<{:?}>", self.img_ty.scalar_ty),
            (Image2DMSArray, Sampled) => write!(f, "sampler2DMSArray<{:?}>", self.img_ty.scalar_ty),
            (CubeMapArray, Sampled) => write!(f, "samplerCubeArray<{:?}>", self.img_ty.scalar_ty),
            (Image2DRect, Sampled) => write!(f, "sampler2DRect<{:?}>", self.img_ty.scalar_ty),
            (ImageBuffer, Sampled) => write!(f, "samplerBuffer<{:?}>", self.img_ty.scalar_ty),

            (Image1D, Depth) => f.write_str("sampler1DShadow"),
            (Image2D, Depth) => f.write_str("sampler2DShadow"),
            (CubeMap, Depth) => f.write_str("samplerCubeShadow"),
            (Image1DArray, Depth) => f.write_str("sampler1DArrayShadow"),
            (Image2DArray, Depth) => f.write_str("sampler2DArrayShadow"),
            (CubeMapArray, Depth) => f.write_str("samplerCubeShadowArray"),
            (Image2DRect, Depth) => write!(f, "sampler2DRectShadow"),
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
        match self.arng {
            SubpassDataArrangement::SubpassData => {
                write!(f, "subpassData<{:?}>", self.scalar_ty)
            },
            SubpassDataArrangement::SubpassDataMS => {
                write!(f, "subpassDataMS<{:?}>", self.scalar_ty)
            },
        }
    }
}


#[derive(PartialEq, Eq, Hash, Clone)]
pub struct ArrayType {
    pub(crate) proto_ty: Box<Type>,
    nrepeat: Option<u32>,
    stride: Option<usize>,
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
        match (self.stride, self.nrepeat) {
            (Some(stride), Some(nrepeat)) => stride * nrepeat as usize,
            _ => 0,
        }
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
// #[non_exhaustive] // TODO: (penguinliong) For SPIR-Q v0.5.
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
        }
    }
}
impl std::ops::BitAnd<AccessType> for AccessType {
    type Output = Option<AccessType>;
    fn bitand(self, rhs: AccessType) -> Option<AccessType> {
        return match (self, rhs) {
            (Self::ReadOnly, Self::ReadWrite) |
                (Self::ReadWrite, Self::ReadOnly) |
                (Self::ReadOnly, Self::ReadOnly) => Some(Self::ReadOnly),
            (Self::WriteOnly, Self::ReadWrite) |
                (Self::ReadWrite, Self::WriteOnly) |
                (Self::WriteOnly, Self::WriteOnly) => Some(Self::WriteOnly),
            (Self::ReadWrite, Self::ReadWrite) => Some(Self::ReadWrite),
            (_, _) => None,
        }
    }
}

pub struct MemberVariableRouting<'a> {
    pub sym: Vec<u32>,
    pub offset: usize,
    pub ty: &'a Type,
}

struct WalkFrame<'a> {
    sym_stem: Option<Vec<u32>>,
    base_offset: usize,
    ty: &'a Type,
    i: usize,
}
pub struct Walk<'a> {
    inner: Vec<WalkFrame<'a>>,
}
impl<'a> Walk<'a> {
    pub fn new(ty: &'a Type) -> Walk<'a> {
        let frame = WalkFrame {
            sym_stem: None,
            base_offset: 0,
            ty: ty,
            i: 0,
        };
        Walk { inner: vec![frame] }
    }
}
impl<'a> Iterator for Walk<'a> {
    type Item = MemberVariableRouting<'a>;
    fn next(&mut self) -> Option<MemberVariableRouting<'a>> {
        fn get_child_ty_offset_seg<'a>(ty: &'a Type, i: usize) -> Option<(&'a Type, usize)> {
            match ty {
                Type::Struct(struct_ty) => {
                    let member = struct_ty.members.get(i)?;
                    Some((&member.ty, member.offset))
                },
                Type::Array(arr_ty) => {
                    // Unsized buffer are treated as 0-sized.
                    if i < arr_ty.nrepeat.unwrap_or(0) as usize {
                        Some((&arr_ty.proto_ty, arr_ty.stride() * i))
                    } else { None }
                },
                _ => None,
            }
        }
        enum LoopEnd<'a> {
            Push(WalkFrame<'a>),
            PopReturn(MemberVariableRouting<'a>),
        }
        loop {
            // If used, this field will be filled with the next frame to be
            // pushed at the back of the walk stack; or the last frame will be
            // popped if the field is kept `None`.
            let loop_end = if let Some(frame) = self.inner.last_mut() {
                if let Some((child_ty, offset)) = get_child_ty_offset_seg(frame.ty, frame.i) {
                    frame.i += 1; // Step member.
                    let sym = if let Some(sym_stem) = &frame.sym_stem {
                        let mut sym = sym_stem.clone();
                        sym.push(frame.i as u32);
                        sym
                    } else { vec![frame.i as u32] };
                    let offset = frame.base_offset + offset;
                    let ty = child_ty;
                    if child_ty.is_struct() || child_ty.is_arr() {
                        // Found composite type, step into it.
                        LoopEnd::Push(WalkFrame { sym_stem: Some(sym), base_offset: offset, ty, i: 0 })
                    } else {
                        // Return directly if it's not a composite type.
                        return Some(MemberVariableRouting { sym, offset, ty });
                    }
                } else {
                    // Here can be reached only when the first frame's type is
                    // neither an array nor a struct; or a later frame's
                    // composite type's elements has been exhausted.
                    let ty = frame.ty;
                    let offset = frame.base_offset;
                    let sym = frame.sym_stem.clone().unwrap_or_default();
                    LoopEnd::PopReturn(MemberVariableRouting { sym, offset, ty })
                }
            } else {
                // We have exhausted all types we have, including the root type
                // of walk.
                return None;
            };
            match loop_end {
                LoopEnd::Push(frame) => {
                    self.inner.push(frame)
                },
                LoopEnd::PopReturn(route) => {
                    self.inner.pop();
                    return Some(route);
                }
            }
        }
    }
}
