//! Structured representations of SPIR-V types.
use std::collections::BTreeMap;
use std::fmt;
use spirv_headers::{Dim, ImageFormat};
use crate::MemberVariableResolution;
use crate::error::*;
use crate::sym::{Sym, Seg};
use std::hash::{Hash, Hasher};

#[derive(Hash, Clone)]
pub enum ScalarType {
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
            Self::Unsigned(nbyte) => write!(f, "i{}", nbyte << 3),
            Self::Float(nbyte) => write!(f, "f{}", nbyte << 3),
        }
    }
}


#[derive(Hash, Clone)]
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


#[derive(Hash, Clone, Copy)]
pub enum MatrixAxisOrder {
    ColumnMajor,
    RowMajor,
}
impl Default for MatrixAxisOrder {
    fn default() -> MatrixAxisOrder { MatrixAxisOrder::ColumnMajor }
}


#[derive(Hash, Clone)]
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


#[derive(Hash, Clone, Copy)]
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
            _ => return Err(Error::UnsupportedSpirv),
        };
        Ok(img_unit_fmt)
    }
}


#[derive(Hash, Clone, Copy)]
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
            (Dim::Dim3D, false, false) => ImageArrangement::Image3D,
            (Dim::Dim3D, true, false) => ImageArrangement::Image3D,
            (Dim::DimCube, false, false) => ImageArrangement::CubeMap,
            (Dim::DimCube, true, false) => ImageArrangement::CubeMapArray,
            _ => return Err(Error::UnsupportedSpirv),
        };
        Ok(arng)
    }
}


#[derive(Hash, Clone)]
pub struct ImageType {
    unit_fmt: ImageUnitFormat,
    arng: ImageArrangement,
}
impl ImageType {
    pub fn new(unit_fmt: ImageUnitFormat, arng: ImageArrangement) -> ImageType {
        ImageType { unit_fmt, arng }
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
            
            (Image1D, Sampled) => f.write_str("sampler1D"),
            (Image2D, Sampled) => f.write_str("sampler2D"),
            (Image2DMS, Sampled) => f.write_str("sampler2DMS"),
            (Image3D, Sampled) => f.write_str("sampler3D"),
            (CubeMap, Sampled) => f.write_str("samplerCube"),
            (Image1DArray, Sampled) => f.write_str("sampler1DArray"),
            (Image2DArray, Sampled) => f.write_str("sampler2DArray"),
            (Image2DMSArray, Sampled) => f.write_str("sampler2DMSArray"),
            (CubeMapArray, Sampled) => f.write_str("samplerCubeArray"),

            (Image1D, Depth) => f.write_str("sampler1DShadow"),
            (Image2D, Depth) => f.write_str("sampler2DShadow"),
            (CubeMap, Depth) => f.write_str("samplerCubeShadow"),
            (Image1DArray, Depth) => f.write_str("sampler1DArrayShadow"),
            (Image2DArray, Depth) => f.write_str("sampler2DArrayShadow"),
            (CubeMapArray, Depth) => f.write_str("samplerCubeShadowArray"),
            _ => Err(fmt::Error::default()),
        }
    }
}


#[derive(Hash, Clone)]
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
            write!(f, "[{:?}", self.proto_ty)
        }
    }
}

#[derive(Clone, Hash)]
pub struct StructMember {
    pub name: Option<String>,
    pub offset: usize,
    pub ty: Type,
}
#[derive(Default, Clone)]
pub struct StructType {
    members: Vec<StructMember>, // Offset and type.
    // BTreeMap to keep the order for hashing.
    name_map: BTreeMap<String, usize>,
}
impl StructType {
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
                return Err(Error::CorruptedSpirv);
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
        f.write_str("{ ")?;
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


#[derive(Hash, Clone)]
pub enum Type {
    Scalar(ScalarType),
    Vector(VectorType),
    Matrix(MatrixType),
    Image(ImageType),
    SubpassData,
    Array(ArrayType),
    Struct(StructType),
}
impl Type {
    pub fn nbyte(&self) -> Option<usize> {
        use Type::*;
        match self {
            Scalar(scalar_ty) => Some(scalar_ty.nbyte()),
            Vector(vec_ty) => Some(vec_ty.nbyte()),
            Matrix(mat_ty) => Some(mat_ty.nbyte()),
            Image(_) => None,
            SubpassData => None,
            Array(arr_ty) => Some(arr_ty.nbyte()),
            Struct(struct_ty) => Some(struct_ty.nbyte()),
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
}
impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Scalar(scalar_ty) => scalar_ty.fmt(f),
            Type::Vector(vec_ty) => vec_ty.fmt(f),
            Type::Matrix(mat_ty) => mat_ty.fmt(f),
            Type::Image(img_ty) => img_ty.fmt(f),
            Type::SubpassData => write!(f, "subpassData"),
            Type::Array(arr_ty) => arr_ty.fmt(f),
            Type::Struct(struct_ty) => struct_ty.fmt(f),
        }
    }
}


/// Structured representation of descriptor types.
#[derive(Hash, Clone)]
pub enum DescriptorType {
    PushConstant(Type),
    UniformBuffer(u32, Type),
    StorageBuffer(u32, Type),
    Image(Type),
    InputAtatchment(u32),
}
impl DescriptorType {
    pub fn nbyte(&self) -> Option<usize> {
        use DescriptorType::*;
        match self {
            PushConstant(ty) => ty.nbyte(),
            UniformBuffer(_, ty) => ty.nbyte(),
            StorageBuffer(_, ty) => ty.nbyte(),
            Image(_) => None,
            InputAtatchment(_) => None,
        }
    }
    /// Resolve a symbol WITHIN the descriptor type. The symbol should not
    /// be led by descriptor set numbers and binding point numbers.
    pub fn resolve<S: AsRef<Sym>>(&self, sym: S) -> Option<MemberVariableResolution<'_>> {
        use DescriptorType::*;
        // Resolve for descriptor root.
        match self {
            PushConstant(ref ty) => ty,
            UniformBuffer(_, ref ty) => ty,
            StorageBuffer(_, ref ty) => ty,
            _ => { return None },
        }.resolve(sym)
    }
}
impl fmt::Debug for DescriptorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DescriptorType::*;
        match self {
            PushConstant(ty) => ty.fmt(f),
            UniformBuffer(nbind, ty) => write!(f, "{}x{:?}", nbind, ty),
            StorageBuffer(nbind, ty) => write!(f, "{}x{:?}", nbind, ty),
            Image(ty) => ty.fmt(f),
            InputAtatchment(idx) => write!(f, "subpassData[{}]", idx),
        }
    }
}
