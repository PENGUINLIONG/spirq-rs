//! Structured representations of SPIR-V types.
use std::collections::BTreeMap;
use std::fmt;
use spirv_headers::{Dim, ImageFormat};
use crate::MemberVariableResolution;
use crate::error::*;
use crate::sym::{Sym, Seg, Symbol};
use std::hash::{Hash, Hasher};

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
    Color(ImageFormat),
    Sampled,
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
    pub scalar_ty: ScalarType,
    pub unit_fmt: ImageUnitFormat,
    pub arng: ImageArrangement,
}
impl ImageType {
    pub fn new(
        scalar_ty: ScalarType,
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
    img_ty: ImageType,
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
    pub scalar_ty: ScalarType,
    pub arng: SubpassDataArrangement,
}
impl SubpassDataType {
    pub fn new(scalar_ty: ScalarType, arng: SubpassDataArrangement) -> Self {
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
#[derive(PartialEq, Eq, Default, Clone)]
pub struct StructType {
    name: Option<String>,
    members: Vec<StructMember>, // Offset and type.
    // BTreeMap to keep the order for hashing.
    name_map: BTreeMap<String, usize>,
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
    pub fn nmember(&self) -> usize { self.members.len() }
    pub fn get_member(&self, i: usize) -> Option<&'_ StructMember> {
        self.members.get(i)
    }
    pub fn get_member_by_name(&self, name: &str) -> Option<&'_ StructMember> {
        self.name_map.get(name).and_then(|x| self.get_member(*x))
    }
    pub fn get_member_name(&self, i: usize) -> Option<&'_ str> {
        self.name_map.iter()
            .find_map(|(name, &j)| if i == j { Some(name.as_ref()) } else { None })
    }
    /// Merge another structure type's member into this structure type.
    pub(crate) fn merge(&mut self, src_struct_ty: &StructType) -> Result<()> {
        use crate::hash;
        let dst_struct_ty = self;
        let member_offset = dst_struct_ty.members.len();
        let member_appendix = src_struct_ty.members.iter().cloned();
        dst_struct_ty.members.extend(member_appendix);
        for (name, &member_idx) in src_struct_ty.name_map.iter() {
            if let Some(&old_member_idx) = dst_struct_ty.name_map.get(name) {
                let old_hash = hash(&dst_struct_ty.members[old_member_idx]);
                let new_hash = hash(&src_struct_ty.members[member_idx]);
                if old_hash != new_hash {
                    return Err(Error::MismatchedManifest);
                }
            } else {
                dst_struct_ty.name_map
                    .insert(name.to_owned(), member_offset + member_idx);
            }
        }
        Ok(())
    }
    /// Push a structure member to this type. Note that the method can fail when
    /// the member's name has already been registered in the current type.
    pub(crate) fn push_member(&mut self, member: StructMember) -> Result<()> {
        let i = self.members.len();
        if let Some(name) = member.name.as_ref() {
            if self.name_map.insert(name.to_owned(), i).is_some() {
                return Err(Error::NAME_COLLISION);
            }
        }
        self.members.push(member);
        Ok(())
    }
}
impl Hash for StructType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.members.hash(state);
        // NOTE: This enforces that the names for a same member in each stage
        // have to be the same to be correctly reflected.
        for x in self.name_map.values() {
            x.hash(state);
        }
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
            if let Some(name) = self.name_map.iter()
                .find_map(|(name, &idx)| if idx == i { Some(name) } else { None }) {
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
pub enum Type {
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
    pub fn resolve<S: AsRef<Sym>>(&self, sym: S) -> Option<MemberVariableResolution<'_>> {
        let mut ty = self;
        let mut offset = 0;
        for seg in sym.as_ref().segs() {
            // Ensure the outer-most type can be addressed.
            if seg == Seg::Empty { break }
            match ty {
                Type::Struct(struct_ty) => {
                    let member = match seg {
                        Seg::Index(i) => struct_ty.get_member(i),
                        Seg::Name(name) => struct_ty.get_member_by_name(name),
                        _ => return None,
                    }?;
                    offset += member.offset;
                    ty = &member.ty;
                },
                Type::Array(arr_ty) => {
                    if let Seg::Index(idx) = seg {
                        if let Some(nrepeat) = arr_ty.nrepeat() {
                            if idx >= nrepeat as usize {
                                return None;
                            }
                        }
                        offset += arr_ty.stride() * idx;
                        ty = &*arr_ty.proto_ty();
                    } else { return None; }
                },
                _ => return None,
            }
        }
        let member_var_res = MemberVariableResolution { offset, ty };
        Some(member_var_res)
    }
    // Iterate over all entries in the type tree.
    pub fn walk<'a>(&'a self) -> Walk<'a> { Walk::new(self) }
    declr_ty_accessor! {
        [Type]
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


/// Structured representation of descriptor types.
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum DescriptorType {
    UniformBuffer(u32, Type),
    StorageBuffer(u32, Type),
    Image(u32, Type),
    Sampler(u32),
    SampledImage(u32, Type),
    // Note that the third parameter is input attachment index, the first one
    // is the binding count.
    InputAttachment(u32, Type, u32),
    AccelStruct(u32),
}
impl DescriptorType {
    /// Get the size of buffer (in bytes) needed to contain all the data for
    /// this buffer object or push constant buffer.
    pub fn nbyte(&self) -> Option<usize> {
        use DescriptorType::*;
        match self {
            UniformBuffer(_, ty) => ty.nbyte(),
            StorageBuffer(_, ty) => ty.nbyte(),
            _ => None,
        }
    }
    /// Number of bindings at the binding point. All descriptors can have
    /// multiple binding points. If the multi-binding is dynamic, 0 will be
    /// returned.
    ///
    /// For more information about dynamic multi-binding, please refer to
    /// Vulkan extension `VK_EXT_descriptor_indexing`, GLSL extension
    /// `GL_EXT_nonuniform_qualifier` and SPIR-V extension
    /// `SPV_EXT_descriptor_indexing`. Dynamic multi-binding is only supported
    /// in Vulkan 1.2.
    pub fn nbind(&self) -> u32 {
        use DescriptorType::*;
        match self {
            UniformBuffer(nbind, _) => *nbind,
            StorageBuffer(nbind, _) => *nbind,
            Image(nbind, _) => *nbind,
            Sampler(nbind) => *nbind,
            SampledImage(nbind, _) => *nbind,
            InputAttachment(nbind, _, _) => *nbind,
            AccelStruct(nbind) => *nbind,
        }
    }
    /// Resolve a symbol WITHIN the descriptor type. The symbol should not
    /// be led by descriptor set numbers and binding point numbers.
    pub fn resolve<S: AsRef<Sym>>(&self, sym: S) -> Option<MemberVariableResolution<'_>> {
        use DescriptorType::*;
        // Resolve for descriptor root.
        match self {
            UniformBuffer(_, ref ty) => ty,
            StorageBuffer(_, ref ty) => ty,
            _ => { return None },
        }.resolve(sym)
    }
    // Iterate over all entries in the type tree.
    pub fn walk<'a>(&'a self) -> Walk<'a> {
        use DescriptorType::*;
        let ty = match self {
            UniformBuffer(_, ty) => ty,
            StorageBuffer(_, ty) => ty,
            Image(_, ty) => ty,
            Sampler(_) => &Type::Sampler(),
            SampledImage(_, ty) => ty,
            InputAttachment(_, ty, _) => ty,
            AccelStruct(_) => &Type::AccelStruct(),
        };
        Walk::new(ty)
    }
    declr_ty_accessor! {
        [DescriptorType]
        is_uniform_buf -> UniformBuffer,
        is_storage_buf -> StorageBuffer,
        is_img -> Image,
        is_sampler -> Sampler,
        is_sampled_img -> SampledImage,
        is_input_attm -> InputAttachment,
        is_accel_struct -> AccelStruct,
    }
}
impl fmt::Debug for DescriptorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DescriptorType::*;
        match self {
            UniformBuffer(nbind, ty) => write!(f, "{}x {:?}", nbind, ty),
            StorageBuffer(nbind, ty) => write!(f, "{}x {:?}", nbind, ty),
            Image(nbind, ty) => write!(f, "{}x {:?}", nbind, ty),
            Sampler(nbind) => write!(f, "{}x sampler", nbind),
            SampledImage(nbind, ty) => write!(f, "{}x {:?}", nbind, ty),
            InputAttachment(nbind, ty, input_attm_idx) => write!(f, "{}x {:?}[{}]", nbind, ty, input_attm_idx),
            AccelStruct(nbind) => write!(f, "{}x accelerationStructure", nbind),
        }
    }
}

pub struct MemberVariableRouting<'a> {
    pub sym: Symbol,
    pub offset: usize,
    pub ty: &'a Type,
}

struct WalkFrame<'a> {
    sym_stem: Option<Symbol>,
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
        fn get_child_ty_offset_seg<'a>(ty: &'a Type, i: usize) -> Option<(&'a Type, usize, Seg<'a>)> {
            match ty {
                Type::Struct(struct_ty) => {
                    let member = struct_ty.members.get(i)?;
                    let seg = if let Some(ref name) = member.name {
                        Seg::Name(name)
                    } else {
                        Seg::Index(i)
                    };
                    Some((&member.ty, member.offset, seg))
                },
                Type::Array(arr_ty) => {
                    // Unsized buffer are treated as 0-sized.
                    if i < arr_ty.nrepeat.unwrap_or(0) as usize {
                        Some((&arr_ty.proto_ty, arr_ty.stride() * i, Seg::Index(i)))
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
                if let Some((child_ty, offset, seg)) = get_child_ty_offset_seg(frame.ty, frame.i) {
                    frame.i += 1; // Step member.
                    let sym = if let Some(sym_stem) = &frame.sym_stem {
                        let mut sym = sym_stem.clone();
                        sym.push(&seg);
                        sym
                    } else { seg.into() };
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
