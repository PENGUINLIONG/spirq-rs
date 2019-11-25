use std::convert::{TryFrom};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::collections::hash_map::Entry::Vacant;
use std::iter::Peekable;
use std::fmt;
use std::ops::RangeInclusive;
use std::hash::{Hash, Hasher};
use super::sym::{Sym, Segment};
use super::consts::*;
use super::instr::*;
use super::{SpirvBinary, Instrs, Instr, Error, Result};

type ObjectId = u32;
type TypeId = ObjectId;
type VariableId = ObjectId;
type ConstantId = ObjectId;
type FunctionId = ObjectId;
type MemberIdx = usize;
type Decoration = u32;

#[derive(Hash, Clone)]
pub struct NumericType {
    /// Byte-width of this type.
    pub nbyte: usize,
    /// For integral types the field indicate it's signed ness, true for signed
    /// int and false for unsigned. Floating point number will have this field
    /// `None`.
    pub is_signed: Option<bool>,
}
impl NumericType {
    pub fn new(nbyte: u32, is_signed: Option<bool>) -> NumericType {
        NumericType { nbyte: nbyte as usize, is_signed: is_signed }
    }
    pub fn int(nbyte: u32, is_signed: bool) -> NumericType {
        NumericType { nbyte: nbyte as usize, is_signed: Some(is_signed) }
    }
    pub fn float(nbyte: u32) -> NumericType {
        NumericType { nbyte: nbyte as usize, is_signed: None }
    }
    pub fn nbyte(&self) -> usize { self.nbyte }

    pub fn is_int(&self) -> bool {
        if let Some(true) = self.is_signed { true } else { false }
    }
    pub fn is_uint(&self) -> bool {
        if let Some(false) = self.is_signed { true } else { false }
    }
    pub fn is_float(&self) -> bool {
        if let None = self.is_signed { true } else { false }
    }
}
impl fmt::Debug for NumericType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.is_signed {
            Some(true) => write!(f, "i{}", self.nbyte << 3),
            Some(false) => write!(f, "u{}", self.nbyte << 3),
            None => write!(f, "f{}", self.nbyte << 3),
        }
    }
}


#[derive(Hash, Clone)]
pub struct VectorType {
    pub num_ty: NumericType,
    pub nnum: u32,
}
impl VectorType {
    pub fn new(num_ty: NumericType, nnum: u32) -> VectorType {
        VectorType { num_ty: num_ty, nnum: nnum }
    }
    pub fn nbyte(&self) -> usize { self.nnum as usize * self.num_ty.nbyte }
}
impl fmt::Debug for VectorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "vec{}<{:?}>", self.nnum, self.num_ty)
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
    pub fn decorate(&mut self, stride: usize, major: MatrixAxisOrder) {
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
        let nrow = self.vec_ty.nnum;
        let ncol = self.nvec;
        let num_ty = &self.vec_ty.num_ty;
        write!(f, "mat{}x{}{}<{:?}>", nrow, ncol, transpose, num_ty)
    }
}


#[derive(Debug, Hash, Clone, Copy)]
pub enum ColorFormat {
    Rgba32f = 1,
    R32f = 3,
    Rgba8 = 4,
}
impl ColorFormat {
    fn from_spv_def(color_fmt: u32) -> Result<ColorFormat> {
        let color_fmt = match color_fmt {
            IMG_UNIT_FMT_RGBA32F => ColorFormat::Rgba32f,
            IMG_UNIT_FMT_R32F => ColorFormat::R32f,
            IMG_UNIT_FMT_RGBA8 => ColorFormat::Rgba8,
            _ => return Err(Error::UnsupportedSpirv),
        };
        Ok(color_fmt)
    }
}

#[derive(Hash, Clone, Copy)]
pub enum ImageUnitFormat {
    Color(ColorFormat),
    Sampled,
    Depth,
}
impl ImageUnitFormat {
    pub fn from_spv_def(is_sampled: u32, is_depth: u32, color_fmt: u32) -> Result<ImageUnitFormat> {
        let img_unit_fmt = match (is_sampled, is_depth, color_fmt) {
            (1, 0, _) => ImageUnitFormat::Sampled,
            (1, 1, _) => ImageUnitFormat::Depth,
            (2, 0, color_fmt) => ImageUnitFormat::Color(ColorFormat::from_spv_def(color_fmt)?),
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
    pub fn from_spv_def(dim: u32, is_array: bool, is_multisampled: bool) -> Result<ImageArrangement> {
        let arng = match (dim, is_array, is_multisampled) {
            (DIM_IMAGE_1D, false, false) => ImageArrangement::Image1D,
            (DIM_IMAGE_1D, true, false) => ImageArrangement::Image1DArray,
            (DIM_IMAGE_2D, false, false) => ImageArrangement::Image2D,
            (DIM_IMAGE_2D, false, true) => ImageArrangement::Image2DMS,
            (DIM_IMAGE_2D, true, false) => ImageArrangement::Image2DArray,
            (DIM_IMAGE_3D, false, false) => ImageArrangement::Image3D,
            (DIM_IMAGE_3D, true, false) => ImageArrangement::Image3D,
            (DIM_IMAGE_CUBE, false, false) => ImageArrangement::CubeMap,
            (DIM_IMAGE_CUBE, true, false) => ImageArrangement::CubeMapArray,
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
    proto_ty: Box<Type>,
    nrepeat: Option<u32>,
    stride: Option<usize>,
}
impl ArrayType {
    pub fn new_multibind(proto_ty: &Type, nrepeat: u32) -> ArrayType {
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


#[derive(Default, Clone)]
pub struct StructType {
    members: Vec<(usize, Type)>, // Offset and type.
    name_map: BTreeMap<String, MemberIdx>,
    // We assume a structure decorated by `Block` is uniform in the first place.
    // On instanciating the structure type into interface block, we check if the
    // storage class is `StorageClass`. If it is, this field will be canceled in
    // the end to false.
    is_iuniform: bool,
}
impl Hash for StructType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.members.hash(state);
        // NOTE: This enforces that the names for a same member in each stage
        // have to be the same to be correctly reflected.
        for x in self.name_map.iter() {
            x.hash(state);
        }
    }
}
impl fmt::Debug for StructType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("{ ")?;
        for (i, (_offset, member_ty)) in self.members.iter().enumerate() {
            if i != 0 { f.write_str(", ")?; }
            if let Some(name) = self.name_map.iter()
                .find_map(|(name, &idx)| if idx == i { Some(name) } else { None }) {
                write!(f, "{}: {:?}", name, member_ty)?;
            }
        }
        f.write_str(" }")
    }
}


#[derive(Debug, Clone)]
struct Constant<'a> {
    ty: InstrId,
    value: &'a [u32],
}
#[derive(Default, Debug, Clone)]
struct Function {
    accessed_vars: HashSet<InstrId>,
    calls: HashSet<InstrId>,
}

#[derive(Hash, Clone)]
pub enum Type {
    Numeric(NumericType),
    Vector(VectorType),
    Matrix(MatrixType),
    Image(Option<ImageType>),
    Array(ArrayType),
    Struct(StructType),
}
impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Numeric(num_ty) => num_ty.fmt(f),
            Type::Vector(vec_ty) => vec_ty.fmt(f),
            Type::Matrix(mat_ty) => mat_ty.fmt(f),
            Type::Image(Some(img_ty)) => img_ty.fmt(f),
            Type::Image(None) => write!(f, "subpassData"),
            Type::Array(arr_ty) => arr_ty.fmt(f),
            Type::Struct(struct_ty) => struct_ty.fmt(f),
        }
    }
}

pub type Location = u32;

#[derive(PartialEq, Eq, Hash, Default, Clone, Copy)]
pub struct DescriptorBinding(Option<(u32, u32)>);
impl DescriptorBinding {
    pub fn push_const() -> Self { DescriptorBinding(None) }
    pub fn desc_bind(desc_set: u32, bind_point: u32) -> Self { DescriptorBinding(Some((desc_set, bind_point))) }

    pub fn is_push_const(&self) -> bool { self.0.is_none() }
    pub fn is_desc_bind(&self) -> bool { self.0.is_some() }
    pub fn into_inner(self) -> Option<(u32, u32)> { self.0 }
}
impl fmt::Debug for DescriptorBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some((set, bind)) = self.0 {
            write!(f, "(set={}, bind={})", set, bind)
        } else {
            write!(f, "(push_constant)")
        }
    }
}


struct EntryPointDeclartion<'a> {
    func_id: u32,
    name: &'a str,
    exec_model: ExecutionModel,
}


#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum ResourceLocator {
    Attribute(Location),
    Attachment(Location),
    Descriptor(DescriptorBinding),
}


#[derive(Default, Clone)]
pub struct EntryPoint {
    exec_model: ExecutionModel,
    name: String,
    attr_map: HashMap<Location, InterfaceVariableType>,
    attm_map: HashMap<Location, InterfaceVariableType>,
    desc_map: HashMap<DescriptorBinding, DescriptorType>,
    var_name_map: HashMap<String, ResourceLocator>,
}
impl EntryPoint {
    pub fn resolve_desc(&self, sym: &Sym) -> Option<(Option<usize>, Type)> {
        let mut segs = sym.segments();
        let desc_bind = match segs.next() {
            Some(Segment::Index(desc_set)) => {
                if let Some(Segment::Index(bind_point)) = segs.next() {
                    DescriptorBinding::desc_bind(desc_set as u32, bind_point as u32)
                } else { return None; }
            },
            Some(Segment::Empty) => {
                // Symbols started with an empty head, like ".modelView", is
                // used to identify push constants.
                DescriptorBinding::push_const()
            }
            Some(Segment::Name(name)) => {
                if let Some(ResourceLocator::Descriptor(desc_bind)) = self.var_name_map.get(name) {
                    *desc_bind
                } else { return None; }
            },
            None => return None,
        };
        let desc_ty = self.desc_map.get(&desc_bind)?;
        let mut ty: Type = match desc_ty {
            DescriptorType::InputAtatchment(_) => {
                // Subpass data have no members.
                return if segs.next().is_none() {
                    Some((None, Type::Image(None)))
                } else { None };
            },
            DescriptorType::Image(img_ty) => {
                // Images have no members.
                return if segs.next().is_none() {
                    Some((None, Type::Image(Some(img_ty.clone()))))
                } else { None };
            },
            DescriptorType::PushConstant(struct_ty) => Type::Struct(struct_ty.clone()),
            DescriptorType::Block(iblock_ty) => Type::Struct(iblock_ty.block_ty.clone()),
        };
        let mut offset = 0;
        while let Some(seg) = segs.next() {
            match ty {
                Type::Struct(struct_ty) => {
                    let idx = match seg {
                        Segment::Index(idx) => idx,
                        Segment::Name(name) => {
                            if let Some(idx) = struct_ty.name_map.get(name) {
                                *idx
                            } else { return None; }
                        },
                        _ => return None,
                    };
                    if let Some((local_offset, new_ty)) = struct_ty.members.get(idx) {
                        offset += local_offset;
                        // TODO: Use `Cow`.
                        ty = new_ty.clone();
                    } else { return None; }
                },
                Type::Array(arr_ty) => {
                    if let Segment::Index(idx) = seg {
                        if let Some(nrepeat) = arr_ty.nrepeat {
                            if idx >= nrepeat as usize {
                                return None;
                            }
                        }
                        if let Some(stride) = arr_ty.stride {
                            offset += stride * idx;
                        } else { return None; }
                        ty = (*arr_ty.proto_ty).clone();
                    } else { return None; }
                },
                _ => return None,
            }
        }
        Some((Some(offset), ty.clone()))
    }
}
impl fmt::Debug for EntryPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct(&self.name)
            .field("attributes", &self.attr_map)
            .field("attachments", &self.attm_map)
            .field("descriptors", &self.desc_map)
            .finish()
    }
}


#[derive(Clone)]
enum InterfaceVariableType {
    Numeric(NumericType),
    Vector(VectorType),
    Matrix(MatrixType),
}
impl InterfaceVariableType {
    fn from_ty<'a>(ty: &Type) -> Option<InterfaceVariableType> {
        let ivar_ty = match ty.clone() {
            Type::Numeric(num_ty) => InterfaceVariableType::Numeric(num_ty),
            Type::Vector(vec_ty) => InterfaceVariableType::Vector(vec_ty),
            Type::Matrix(mat_ty) => InterfaceVariableType::Matrix(mat_ty),
            _ => return None,
        };
        Some(ivar_ty)
    }
}
impl fmt::Debug for InterfaceVariableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use InterfaceVariableType::*;
        match self {
            Numeric(num_ty) => fmt::Debug::fmt(num_ty, f),
            Vector(vec_ty) => fmt::Debug::fmt(vec_ty, f),
            Matrix(mat_ty) => fmt::Debug::fmt(mat_ty, f),
        }
    }
}


#[derive(Clone)]
pub struct InterfaceBlockType {
    block_ty: StructType,
    nbind: u32,
}
impl InterfaceBlockType {
    fn from_store_buf(block_ty: &Type) -> Option<InterfaceBlockType> {
        let (mut block_ty, nbind) = match block_ty {
            Type::Array(arr_ty) => if let Type::Struct(struct_ty) = &*arr_ty.proto_ty {
                (struct_ty.clone(), arr_ty.nrepeat?)
            } else { return None },
            Type::Struct(struct_ty) => (struct_ty.clone(), 1),
            _ => return None,
        };
        // Force cancel the uniformity of current block type.
        block_ty.is_iuniform = false;
        let iblock_ty = InterfaceBlockType { block_ty: block_ty, nbind: nbind };
        Some(iblock_ty)
    }
    fn from_uniform(block_ty: &Type) -> Option<InterfaceBlockType> {
        let (block_ty, nbind) = match block_ty {
            Type::Array(arr_ty) => if let Type::Struct(struct_ty) = &*arr_ty.proto_ty {
                (struct_ty.clone(), arr_ty.nrepeat?)
            } else { return None },
            Type::Struct(struct_ty) => (struct_ty.clone(), 1),
            _ => return None,
        };
        let iblock_ty = InterfaceBlockType { block_ty: block_ty, nbind: nbind };
        Some(iblock_ty)
    }
    pub fn is_uniform(&self) -> bool { self.block_ty.is_iuniform }
    pub fn is_storage(&self) -> bool { !self.block_ty.is_iuniform }
}
impl fmt::Debug for InterfaceBlockType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let buf_ty = if self.block_ty.is_iuniform { "uniform" } else { "buffer" };
        write!(f, "[{}: {:?}; {}]", buf_ty, self.block_ty, self.nbind)
    }
}


#[derive(Clone)]
pub enum DescriptorType {
    PushConstant(StructType),
    Block(InterfaceBlockType),
    Image(ImageType),
    InputAtatchment(u32),
}
impl fmt::Debug for DescriptorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DescriptorType::*;
        match self {
            PushConstant(struct_ty) => write!(f, "{:?}", struct_ty),
            Block(iblock_ty) => write!(f, "{:?}", iblock_ty),
            Image(img_ty) => write!(f, "{:?}", img_ty),
            InputAtatchment(idx) => write!(f, "subpassData[{}]", idx),
        }
    }
}

#[derive(Clone)]
enum Variable {
    Input(Location, InterfaceVariableType),
    Output(Location, InterfaceVariableType),
    Descriptor(DescriptorBinding, DescriptorType),
}

#[derive(Default)]
struct ReflectIntermediate<'a> {
    entry_point_declrs: Vec<EntryPointDeclartion<'a>>,
    name_map: HashMap<(InstrId, Option<u32>), &'a str>,
    deco_map: HashMap<(InstrId, Option<u32>, Decoration), &'a [u32]>,
    ty_map: HashMap<TypeId, Type>,
    var_map: HashMap<VariableId, Variable>,
    const_map: HashMap<ConstantId, Constant<'a>>,
    ptr_map: HashMap<TypeId, TypeId>,
    func_map: HashMap<FunctionId, Function>,
}
impl<'a> ReflectIntermediate<'a> {
    /// Resolve one recurring layer of pointers to the pointer that refer to the
    /// data directly.
    fn resolve_ref(&self, ty_id: TypeId) -> Option<(TypeId, &Type)> {
        self.ptr_map.get(&ty_id)
            .and_then(|ty_id| {
                self.ty_map.get(ty_id)
                    .map(|ty| (*ty_id, ty))
            })
    }
    fn contains_deco(&self, id: ObjectId, member_idx: Option<u32>, deco: Decoration) -> bool {
        self.deco_map.contains_key(&(id, member_idx, deco))
    }
    fn _get_deco(&self, id: InstrId, member_idx: Option<u32>, deco: Decoration) -> Option<&[u32]> {
        self.deco_map.get(&(id, member_idx, deco))
            .cloned()
    }
    fn get_deco_u32(&self, id: InstrId, member_idx: Option<u32>, deco: Decoration) -> Option<u32> {
        self.deco_map.get(&(id, member_idx, deco))
            .and_then(|x| x.get(0))
            .cloned()
    }
    fn get_var_location_or_default(&self, var_id: VariableId) -> u32 {
        self.get_deco_u32(var_id, None, DECO_LOCATION)
            .unwrap_or(0)
    }
    fn get_var_desc_bind_or_default(&self, var_id: VariableId) -> DescriptorBinding {
        let desc_set = self.get_deco_u32(var_id, None, DECO_DESCRIPTOR_SET)
            .unwrap_or(0);
        let bind_point = self.get_deco_u32(var_id, None, DECO_BINDING)
            .unwrap_or(0);
        DescriptorBinding::desc_bind(desc_set, bind_point)
    }
    fn get_name(&self, id: InstrId, member_idx: Option<u32>) -> Option<&'a str> {
        self.name_map.get(&(id, member_idx))
            .map(|x| *x)
    }
    fn populate_entry_points(&mut self, instrs: &'_ mut Peekable<Instrs<'a>>) -> Result<()> {
        while let Some(instr) = instrs.peek() {
            if instr.opcode() != OP_ENTRY_POINT { break; }
            let op = OpEntryPoint::try_from(instr)?;
            let entry_point_declr = EntryPointDeclartion {
                exec_model: op.exec_model,
                func_id: op.func_id,
                name: op.name,
            };
            self.entry_point_declrs.push(entry_point_declr);
            instrs.next();
        }
        Ok(())
    }
    fn populate_names(&mut self, instrs: &'_ mut Peekable<Instrs<'a>>) -> Result<()> {
        // Extract naming. Names are generally produced as debug information by
        // `glslValidator` but it might be in absence.
        while let Some(instr) = instrs.peek() {
            let (key, value) = match instr.opcode() {
                OP_NAME => {
                    let op = OpName::try_from(instr)?;
                    ((op.target_id, None), op.name)
                },
                OP_MEMBER_NAME => {
                    let op = OpMemberName::try_from(instr)?;
                    ((op.target_id, Some(op.member_idx)), op.name)
                },
                _ => break,
            };
            let collision = self.name_map.insert(key, value);
            if collision.is_some() { return Err(Error::CorruptedSpirv); }
            instrs.next();
        }
        Ok(())
    }
    fn populate_decos(&mut self, instrs: &'_ mut Peekable<Instrs<'a>>) -> Result<()> {
        while let Some(instr) = instrs.peek() {
            let (key, value) = match instr.opcode() {
                OP_DECORATE => {
                    let op = OpDecorate::try_from(instr)?;
                    ((op.target_id, None, op.deco), op.params)
                }
                OP_MEMBER_DECORATE => {
                    let op = OpMemberDecorate::try_from(instr)?;
                    ((op.target_id, Some(op.member_idx), op.deco), op.params)
                },
                _ => break,
            };
            let collision = self.deco_map.insert(key, value);
            if collision.is_some() { return Err(Error::CorruptedSpirv); }
            instrs.next();
        }
        Ok(())
    }
    fn populate_one_ty(&mut self, instr: &Instr<'a>) -> Result<()> {
        let (key, value) = match instr.opcode() {
            OP_TYPE_VOID | OP_TYPE_FUNCTION => { return Ok(()) },
            OP_TYPE_BOOL => return Err(Error::UnsupportedSpirv),
            OP_TYPE_INT => {
                let op = OpTypeInt::try_from(instr)?;
                let num_ty = NumericType::int(op.nbyte >> 3, op.is_signed);
                (op.ty_id, Type::Numeric(num_ty))
            }
            OP_TYPE_FLOAT => {
                let op = OpTypeFloat::try_from(instr)?;
                let num_ty = NumericType::float(op.nbyte >> 3);
                (op.ty_id, Type::Numeric(num_ty))
            },
            OP_TYPE_VECTOR => {
                let op = OpTypeVector::try_from(instr)?;
                if let Some(Type::Numeric(num_ty)) = self.ty_map.get(&op.num_ty_id).cloned() {
                    let vec_ty = VectorType::new(num_ty, op.nnum);
                    (op.ty_id, Type::Vector(vec_ty))
                } else { return Err(Error::CorruptedSpirv); }
            },
            OP_TYPE_MATRIX => {
                let op = OpTypeMatrix::try_from(instr)?;
                if let Some(Type::Vector(vec_ty)) = self.ty_map.get(&op.vec_ty_id).cloned() {
                    let mat_ty = MatrixType::new(vec_ty, op.nvec);
                    (op.ty_id, Type::Matrix(mat_ty))
                } else { return Err(Error::CorruptedSpirv); }
            },
            OP_TYPE_IMAGE => {
                let op = OpTypeImage::try_from(instr)?;
                let img_ty = if op.dim == DIM_IMAGE_SUBPASS_DATA {
                    Type::Image(None)
                } else {
                    // Only unit types allowed to be stored in storage images can
                    // have given format.
                    let unit_fmt = ImageUnitFormat::from_spv_def(op.is_sampled, op.is_depth, op.color_fmt)?;
                    let arng = ImageArrangement::from_spv_def(op.dim, op.is_array, op.is_multisampled)?;
                    let img_ty = ImageType { unit_fmt: unit_fmt, arng: arng };
                    Type::Image(Some(img_ty))
                };
                (op.ty_id, img_ty)
            },
            OP_TYPE_SAMPLED_IMAGE => {
                let op = OpTypeSampledImage::try_from(instr)?;
                if let Some(Type::Image(img_ty)) = self.ty_map.get(&op.img_ty_id) {
                    (op.ty_id, Type::Image(img_ty.clone()))
                } else { return Err(Error::CorruptedSpirv); }
            },
            OP_TYPE_ARRAY => {
                let op = OpTypeArray::try_from(instr)?;
                let proto_ty = self.ty_map.get(&op.proto_ty_id)
                    .ok_or(Error::CorruptedSpirv)?;
                let nrepeat = self.const_map.get(&op.nrepeat_const_id)
                    .and_then(|constant| {
                        if let Some(Type::Numeric(num_ty)) = self.ty_map.get(&constant.ty) {
                            if num_ty.nbyte == 4 && num_ty.is_uint() {
                                return Some(constant.value[0]);
                            }
                        }
                        None
                    })
                    .ok_or(Error::CorruptedSpirv)?;
                let stride = self.get_deco_u32(op.ty_id, None, DECO_ARRAY_STRIDE)
                    .map(|x| x as usize);
                let arr_ty = if let Some(stride) = stride {
                    ArrayType::new(proto_ty, nrepeat, stride)
                } else {
                    ArrayType::new_multibind(proto_ty, nrepeat)
                };
                (op.ty_id, Type::Array(arr_ty))
            },
            OP_TYPE_RUNTIME_ARRAY => {
                let op = OpTypeRuntimeArray::try_from(instr)?;
                let proto_ty = self.ty_map.get(&op.proto_ty_id)
                    .ok_or(Error::CorruptedSpirv)?;
                let stride = self.get_deco_u32(op.ty_id, None, DECO_ARRAY_STRIDE)
                    .map(|x| x as usize)
                    .ok_or(Error::CorruptedSpirv)?;
                let arr_ty = ArrayType::new_unsized(proto_ty, stride);
                (op.ty_id, Type::Array(arr_ty))
            }
            OP_TYPE_STRUCT => {
                let op = OpTypeStruct::try_from(instr)?;
                let mut struct_ty = StructType::default();
                struct_ty.is_iuniform = self.contains_deco(op.ty_id, None, DECO_BLOCK);
                for (i, &member_ty_id) in op.member_ty_ids.iter().enumerate() {
                    let mut member_ty = self.ty_map.get(&member_ty_id)
                        .cloned()
                        .ok_or(Error::CorruptedSpirv)?;
                    let mut proto_ty = &mut member_ty;
                    while let Type::Array(arr_ty) = proto_ty {
                        proto_ty = &mut arr_ty.proto_ty;
                    }
                    if let &mut Type::Matrix(ref mut mat_ty) = proto_ty {
                        let i = i as u32;
                        let mat_stride = self.get_deco_u32(op.ty_id, Some(i), DECO_MATRIX_STRIDE)
                            .ok_or(Error::CorruptedSpirv)?;
                        let row_major = self.contains_deco(op.ty_id, Some(i), DECO_ROW_MAJOR);
                        let col_major = self.contains_deco(op.ty_id, Some(i), DECO_COL_MAJOR);
                        let major = match (row_major, col_major) {
                            (true, false) => MatrixAxisOrder::RowMajor,
                            (false, true) => MatrixAxisOrder::ColumnMajor,
                            _ => return Err(Error::CorruptedSpirv),
                        };
                        mat_ty.decorate(mat_stride as usize, major);
                    }
                    if let Some(name) = self.get_name(op.ty_id, Some(i as u32)) {
                        if !name.is_empty() {
                            struct_ty.name_map.insert(name.to_owned(), i);
                        }
                    }
                    if let Some(offset) = self.get_deco_u32(op.ty_id, Some(i as u32), DECO_OFFSET) {
                        struct_ty.members.push((offset as usize, member_ty));
                    } else {
                        // For shader input/output blocks there are no offset
                        // decoration. Since these variables are not externally
                        // accessible we don't have to worry about them.
                        return Ok(())
                    }
                }
                // Don't have to shrink-to-fit because the types in `ty_map`
                // won't be used directly and will be cloned later.
                (op.ty_id, Type::Struct(struct_ty))
            },
            OP_TYPE_POINTER => {
                let op = OpTypePointer::try_from(instr)?;
                if self.ptr_map.insert(op.ty_id, op.target_ty_id).is_some() {
                    return Err(Error::CorruptedSpirv)
                } else { return Ok(()) }
            },
            _ => return Err(Error::CorruptedSpirv),
        };
        if let Vacant(entry) = self.ty_map.entry(key) {
            entry.insert(value); Ok(())
        } else { Err(Error::CorruptedSpirv) }
    }
    fn populate_one_const(&mut self, instr: &Instr<'a>) -> Result<()> {
        if instr.opcode() != OP_CONSTANT { return Ok(()) }
        let op = OpConstant::try_from(instr)?;
        let constant = Constant { ty: op.ty_id, value: op.value };
        if let Vacant(entry) = self.const_map.entry(op.const_id) {
            entry.insert(constant); Ok(())
        } else { Err(Error::CorruptedSpirv) }
    }
    fn populate_one_var(&mut self, instr: &Instr<'a>) -> Result<()> {
        let op = OpVariable::try_from(instr)?;
        let (ty_id, ty) = if let Some(x) = self.resolve_ref(op.ty_id) { x } else {
            // If a variable is declared based on a unregistered type, very
            // likely it's a input/output block passed between shader stages. We
            // can safely ignore them.
            return Ok(());
        };
        match op.store_cls {
            STORE_CLS_INPUT => {
                if let Some(ivar_ty) = InterfaceVariableType::from_ty(ty) {
                    let location = self.get_var_location_or_default(op.alloc_id);
                    let var = Variable::Input(location, ivar_ty);
                    if self.var_map.insert(op.alloc_id, var).is_some() {
                        return Err(Error::CorruptedSpirv);
                    }
                }
                // There can be interface blocks for input and output but there
                // won't be any for attribute inputs nor for attachment outputs,
                // so we just ignore structs and arrays or something else here.
            },
            STORE_CLS_OUTPUT => {
                if let Some(ivar_ty) = InterfaceVariableType::from_ty(ty) {
                    let location = self.get_var_location_or_default(op.alloc_id);
                    let var = Variable::Output(location, ivar_ty);
                    if self.var_map.insert(op.alloc_id, var).is_some() {
                        return Err(Error::CorruptedSpirv);
                    }
                }
            },
            STORE_CLS_PUSH_CONSTANT => {
                // Push constants have no global offset. Offsets are applied to
                // members.
                if let Type::Struct(struct_ty) = ty.clone() {
                    let desc_bind = DescriptorBinding::push_const();
                    let desc_ty = DescriptorType::PushConstant(struct_ty);
                    let var = Variable::Descriptor(desc_bind, desc_ty);
                    if self.var_map.insert(op.alloc_id, var).is_some() {
                        return Err(Error::CorruptedSpirv);
                    }
                } else { return Err(Error::CorruptedSpirv); }
            },
            STORE_CLS_UNIFORM => {
                let ctor = if self.contains_deco(ty_id, None, DECO_BUFFER_BLOCK) {
                    InterfaceBlockType::from_store_buf
                } else {
                    InterfaceBlockType::from_uniform
                };
                if let Some(iblock_ty) = ctor(ty) {
                    let desc_bind = self.get_var_desc_bind_or_default(op.alloc_id);
                    let desc_ty = DescriptorType::Block(iblock_ty);
                    let var = Variable::Descriptor(desc_bind, desc_ty);
                    if self.var_map.insert(op.alloc_id, var).is_some() {
                        return Err(Error::CorruptedSpirv);
                    }
                } else { return Err(Error::CorruptedSpirv); }
            },
            STORE_CLS_STORAGE_BUFFER => {
                if let Some(iblock_ty) = InterfaceBlockType::from_store_buf(ty) {
                    let desc_bind = self.get_var_desc_bind_or_default(op.alloc_id);
                    let desc_ty = DescriptorType::Block(iblock_ty);
                    let var = Variable::Descriptor(desc_bind, desc_ty);
                    if self.var_map.insert(op.alloc_id, var).is_some() {
                        return Err(Error::CorruptedSpirv);
                    }
                } else { return Err(Error::CorruptedSpirv); }
            },
            STORE_CLS_UNIFORM_CONSTANT => {
                if let Type::Image(img_ty) = ty {
                    let desc_bind = self.get_var_desc_bind_or_default(op.alloc_id);
                    let desc_ty = if let Some(img_ty) = img_ty {
                        DescriptorType::Image(img_ty.clone())
                    } else {
                        let input_attm_idx = self.get_deco_u32(op.alloc_id, None, DECO_INPUT_ATTACHMENT_INDEX)
                            .ok_or(Error::CorruptedSpirv)?;
                        DescriptorType::InputAtatchment(input_attm_idx)
                    };
                    let var = Variable::Descriptor(desc_bind, desc_ty);
                    if self.var_map.insert(op.alloc_id, var).is_some() {
                        return Err(Error::CorruptedSpirv);
                    }
                }
                // Leak out unknown types of uniform constants.
            },
            _ => {
                // Leak out unknown storage classes.
            },
        }
        Ok(())
    }
    fn populate_defs(&mut self, instrs: &'_ mut Peekable<Instrs<'a>>) -> Result<()> {
        // type definitions always follow decorations, so we don't skip
        // instructions here.
        while let Some(instr) = instrs.peek() {
            let opcode = instr.opcode();
            if TYPE_RANGE.contains(&opcode) {
                self.populate_one_ty(instr)?;
            } else if opcode == OP_VARIABLE {
                self.populate_one_var(instr)?;
            } else if CONST_RANGE.contains(&opcode) {
                self.populate_one_const(instr)?;
            } else if SPEC_CONST_RANGE.contains(&opcode) {
                // TODO: (penguinliong)
            } else { break; }
            instrs.next();
        }
        Ok(())
    }
    fn populate_access(&mut self, instrs: &'_ mut Peekable<Instrs<'a>>) -> Result<()> {
        while instrs.peek().is_some() {
            let mut access_chain_map = HashMap::new();
            let mut func: &mut Function = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
            while let Some(instr) = instrs.peek() {
                if instr.opcode() == OP_FUNCTION {
                    let op = OpFunction::try_from(instr)?;
                    func = self.func_map.entry(op.func_id).or_default();
                    break;
                }
                instrs.next();
            }
            while let Some(instr) = instrs.peek() {
                match instr.opcode() {
                    OP_FUNCTION_CALL => {
                        let op = OpFunctionCall::try_from(instr)?;
                        if !func.calls.insert(op.func_id) {
                            return Err(Error::CorruptedSpirv);
                        }
                    },
                    OP_LOAD => {
                        let op = OpLoad::try_from(instr)?;
                        let mut rsc_id = op.rsc_id;
                        if let Some(&x) = access_chain_map.get(&rsc_id) { rsc_id = x }
                        func.accessed_vars.insert(rsc_id);
                    },
                    OP_STORE => {
                        let op = OpStore::try_from(instr)?;
                        let mut rsc_id = op.rsc_id;
                        if let Some(&x) = access_chain_map.get(&rsc_id) { rsc_id = x }
                        func.accessed_vars.insert(rsc_id);
                    },
                    OP_ACCESS_CHAIN => {
                        let op = OpAccessChain::try_from(instr)?;
                        if access_chain_map.insert(op.rsc_id, op.accessed_rsc_id).is_some() {
                            return Err(Error::CorruptedSpirv);
                        }
                    },
                    OP_FUNCTION_END => break,
                    _ => { },
                }
                instrs.next();
            }
        }
        Ok(())
    }
    fn collect_fn_vars_impl(&self, func: FunctionId, vars: &mut HashSet<VariableId>) {
        if let Some(func) = self.func_map.get(&func) {
            let it = func.accessed_vars.iter()
                .filter(|x| self.var_map.contains_key(x));
            vars.extend(it);
            for call in func.calls.iter() {
                self.collect_fn_vars_impl(*call, vars);
            }
        }
    }
    fn collect_fn_vars(&self, func: FunctionId) -> HashSet<VariableId> {
        let mut accessed_vars = HashSet::new();
        self.collect_fn_vars_impl(func, &mut accessed_vars);
        accessed_vars
    }
    fn collect_entry_points(&self) -> Result<Box<[EntryPoint]>> {
        let mut entry_points = Vec::with_capacity(self.entry_point_declrs.len());
        for entry_point_declr in self.entry_point_declrs.iter() {
            let mut entry_point = EntryPoint {
                name: entry_point_declr.name.to_owned(),
                exec_model: entry_point_declr.exec_model,
                ..Default::default()
            };
            let accessed_var_ids = self.collect_fn_vars(entry_point_declr.func_id);
            for accessed_var_id in accessed_var_ids {
                let accessed_var = self.var_map.get(&accessed_var_id)
                    .cloned()
                    .ok_or(Error::CorruptedSpirv)?;
                let collision = match accessed_var {
                    Variable::Input(location, ivar_ty) => {
                        let mut collision = entry_point.attr_map.insert(location, ivar_ty).is_some();
                        if let Some(name) = self.get_name(accessed_var_id, None) {
                            collision |= entry_point.var_name_map
                                .insert(name.to_owned(), ResourceLocator::Attribute(location)).is_some();
                        }
                        collision
                    },
                    Variable::Output(location, ivar_ty) => {
                        let mut collision = entry_point.attm_map.insert(location, ivar_ty).is_some();
                        if let Some(name) = self.get_name(accessed_var_id, None) {
                            collision |= entry_point.var_name_map
                                .insert(name.to_owned(), ResourceLocator::Attachment(location)).is_some();
                        }
                        collision
                    },
                    Variable::Descriptor(desc_bind, desc_ty) => {
                        let mut collision = entry_point.desc_map.insert(desc_bind, desc_ty).is_some();
                        if let Some(name) = self.get_name(accessed_var_id, None) {
                            collision |= entry_point.var_name_map
                                .insert(name.to_owned(), ResourceLocator::Descriptor(desc_bind)).is_some();
                        }
                        collision
                    },
                };
                if collision { return Err(Error::CorruptedSpirv); }
            }
            entry_points.push(entry_point);
        }
        Ok(entry_points.into_boxed_slice())
    }
}

pub fn reflect_spirv<'a>(module: &'a SpirvBinary) -> Result<Box<[EntryPoint]>> {
    fn skip_until_range_inclusive<'a>(instrs: &'_ mut Peekable<Instrs<'a>>, rng: RangeInclusive<u32>) {
        while let Some(instr) = instrs.peek() {
            if !rng.contains(&instr.opcode()) { instrs.next(); } else { break; }
        }
    }
    // Don't change the order. See _2.4 Logical Layout of a Module_ of the
    // SPIR-V specification for more information.
    let mut instrs = module.instrs().peekable();
    let mut itm = ReflectIntermediate::default();
    skip_until_range_inclusive(&mut instrs, ENTRY_POINT_RANGE);
    itm.populate_entry_points(&mut instrs)?;
    skip_until_range_inclusive(&mut instrs, NAME_RANGE);
    itm.populate_names(&mut instrs)?;
    skip_until_range_inclusive(&mut instrs, DECO_RANGE);
    itm.populate_decos(&mut instrs)?;
    itm.populate_defs(&mut instrs)?;
    itm.populate_access(&mut instrs)?;
    Ok(itm.collect_entry_points()?)
}

/*
#[derive(Debug, Default)]
pub struct PipelineMetadata {
    attr_templates: Vec<VertexAttributeContractTemplate>,
    attm_templates: Vec<AttachmentContractTemplate>,
    desc_binds: HashMap<DescriptorBinding, Descriptor>,
    desc_name_map: HashMap<String, DescriptorBinding>,
}
impl PipelineMetadata {
    pub fn new(spvs: &[SpirvBinary]) -> Result<PipelineMetadata> {
        use std::convert::TryInto;
        use log::debug;
        let mut found_stages = HashSet::new();
        let mut meta = PipelineMetadata::default();
        for spv in spvs {
            let spv_meta: SpirvMetadata = spv.try_into()?;
            for entry_point in spv_meta.entry_points {
                let EntryPoint { func, name, exec_model } = entry_point;
                if !found_stages.insert(entry_point.exec_model) {
                    // Stage collision.
                    return Err(Error::MalformedPipeline);
                }

                match entry_point.exec_model {
                    EXEC_MODEL_VERTEX => meta.attr_templates = attr_templates,
                    EXEC_MODEL_FRAGMENT => meta.attm_templates = attm_templates,
                    _ => {},
                }
                // TODO: (pengunliong) Resolve structural and naming conflicts.
                for (desc_bind, desc) in desc_binds.into_iter() {
                    meta.desc_binds.entry(desc_bind).or_insert(desc);
                }
                for (name, desc_bind) in desc_name_map.into_iter() {
                    meta.desc_name_map.entry(name).or_insert(desc_bind);
                }
            }
        }
        Ok(meta)
    }
}
*/
