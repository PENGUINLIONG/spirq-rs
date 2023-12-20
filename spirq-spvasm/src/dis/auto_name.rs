use std::collections::HashMap;
use anyhow::Result;
use half::f16;

use spirq::{ReflectConfig, reflect::ReflectIntermediate};
use spirq_core::{parse::SpirvBinary, ty::{Type, ScalarType, VectorType, MatrixType, PointerType, ArrayType, StructType}, constant::ConstantValue};

fn sanitize_name(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_ascii_punctuation() { '_' } else { c })
        .collect::<String>()
}

struct AutoNamer {
    names: HashMap<u32, String>,
    cache: HashMap<Type, u32>,
    name_counter: HashMap<String, u32>,
}
impl AutoNamer {
    fn assign_name(&mut self, id: u32, name: String) {
        // Ignore second assignment.
        if self.names.contains_key(&id) {
            return;
        }
        match self.name_counter.entry(name.clone()) {
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(0);
                self.names.entry(id).or_insert(name.clone());
            },
            std::collections::hash_map::Entry::Occupied(e) => {
                let counter = e.into_mut();
                let name = format!("{}_{}", name, counter);
                *counter += 1;
                self.names.entry(id).or_insert(name);
            }
        }
    }

    fn collect_annotated_names(
        &mut self,
        itm: &ReflectIntermediate,
    ) -> Result<()> {
        for (name_key, name) in itm.name_reg.iter() {
            if name_key.member_idx.is_none() {
                // Sanitize name. Convert all punctuations to underscore.
                let name = sanitize_name(name);
                self.assign_name(name_key.id, name);
            }
        }
        Ok(())
    }

    // Make type names.

    fn make_scalar_ty_name(&mut self, scalar_ty: &ScalarType) -> Option<String> {
        let out = match scalar_ty {
            ScalarType::Void => "void".to_string(),
            ScalarType::Boolean => "bool".to_string(),
            ScalarType::Integer { bits: 8, is_signed: true } => "char".to_string(),
            ScalarType::Integer { bits: 16, is_signed: true } => "short".to_string(),
            ScalarType::Integer { bits: 32, is_signed: true } => "int".to_string(),
            ScalarType::Integer { bits: 64, is_signed: true } => "long".to_string(),
            ScalarType::Integer { bits: 8, is_signed: false } => "uchar".to_string(),
            ScalarType::Integer { bits: 16, is_signed: false } => "ushort".to_string(),
            ScalarType::Integer { bits: 32, is_signed: false } => "uint".to_string(),
            ScalarType::Integer { bits: 64, is_signed: false } => "ulong".to_string(),
            ScalarType::Float { bits: 16 } => "half".to_string(),
            ScalarType::Float { bits: 32 } => "float".to_string(),
            ScalarType::Float { bits: 64 } => "double".to_string(),
            _ => return None,
        };
        Some(out)
    }
    fn make_vector_ty_name(&mut self, vector_ty: &VectorType) -> Option<String> {
        let out = format!("v{}{}", vector_ty.nscalar, self.make_scalar_ty_name(&vector_ty.scalar_ty)?);
        Some(out)
    }
    fn make_matrix_ty_name(&mut self, matrix_ty: &MatrixType) -> Option<String> {
        let out = format!("mat{}{}", matrix_ty.nvector, self.make_vector_ty_name(&matrix_ty.vector_ty)?);
        Some(out)
    }

    fn make_arr_ty_name(&mut self, arr_ty: &ArrayType) -> Option<String> {
        let out = if let Some(nelement) = arr_ty.nelement {
            format!("_arr_{}_uint_{}", self.make_ty_name(&arr_ty.element_ty)?, nelement)
        } else {
            format!("_runtimearr_{}", self.make_ty_name(&arr_ty.element_ty)?)
        };
        Some(out)
    }

    fn make_pointer_ty_name(&mut self, pointer_ty: &PointerType) -> Option<String> {
        let out = if let Some(pointee_name) = self.make_ty_name(&pointer_ty.pointee_ty) {
            format!("_ptr_{:?}_{}", pointer_ty.store_cls, pointee_name)
        } else {
            if let Some(id) = self.cache.get(&pointer_ty.pointee_ty) {
                format!("_ptr_{:?}_{}", pointer_ty.store_cls, id)
            } else {
                return None;
            }
        };
        Some(out)
    }

    fn make_ty_name(&mut self, ty: &Type) -> Option<String> {
        if let Some(cached_id) = self.cache.get(ty) {
            if let Some(cached_name) = self.names.get(cached_id) {
                return Some(cached_name.clone());
            }
        }

        let out = match ty {
            Type::Scalar(scalar_ty) => self.make_scalar_ty_name(scalar_ty),
            Type::Vector(vector_ty) => self.make_vector_ty_name(vector_ty),
            Type::Matrix(matrix_ty) => self.make_matrix_ty_name(matrix_ty),
            Type::Array(arr_ty) => self.make_arr_ty_name(arr_ty),
            Type::Struct(StructType { name, .. }) => name.as_ref().map(|x| sanitize_name(x)),
            Type::DevicePointer(pointer_ty) => self.make_pointer_ty_name(pointer_ty),
            _ => None,
        };

        out
    }

    fn collect_ty_names(
        &mut self,
        itm: &ReflectIntermediate,
    ) -> Result<()> {
        let mut tys = itm.ty_reg.iter().collect::<Vec<_>>();
        tys.sort_by_key(|(id, _)| *id);
        for (id, ty) in tys {
            if let Some(name) = self.make_ty_name(ty) {
                self.assign_name(*id, name);
                self.cache.entry(ty.clone()).or_insert(*id);
            } else {
                self.cache.insert(ty.clone(), *id);
            }
        }

        Ok(())
    }

    fn make_const_name(
        &mut self,
        value: &ConstantValue,
    ) -> Option<String> {
        let mut out = match value {
            ConstantValue::Bool(true) => "true".to_owned(),
            ConstantValue::Bool(false) => "false".to_owned(),
            ConstantValue::S8(x) => format!("char_{}", x),
            ConstantValue::S16(x) => format!("short_{}", x),
            ConstantValue::S32(x) => format!("int_{}", x),
            ConstantValue::S64(x) => format!("long_{}", x),
            ConstantValue::U8(x) => format!("uchar_{}", x),
            ConstantValue::U16(x) => format!("ushort_{}", x),
            ConstantValue::U32(x) => format!("uint_{}", x),
            ConstantValue::U64(x) => format!("ulong_{}", x),
            ConstantValue::F16(x) => if x.0 < f16::ZERO {
                format!("half_n{}", -x)
            } else {
                format!("half_{}", x)
            },
            ConstantValue::F32(x) => if x.0 < 0.0 {
                format!("float_n{}", -x)
            } else {
                format!("float_{}", x)
            },
            ConstantValue::F64(x) => if x.0 < 0.0 {
                format!("double_n{}", -x)
            } else {
                format!("double_{}", x)
            },
            _ => return None,
        };
        out = sanitize_name(&out);
        Some(out)
    }

    fn collect_const_names(
        &mut self,
        itm: &ReflectIntermediate,
    ) -> Result<()> {
        for (id, constant) in itm.interp.iter() {
            if let Some(name) = constant.name.as_ref() {
                self.assign_name(*id, name.clone());
            } else if let Some(name) = self.make_const_name(&constant.value) {
                self.assign_name(*id, name);
            }
        }
        Ok(())
    }
}

pub fn collect_names(
    itm: &ReflectIntermediate,
    name_ids: bool,
    name_type_ids: bool,
    name_const_ids: bool,
) -> Result<HashMap<u32, String>> {
    let mut auto_namer = AutoNamer {
        names: HashMap::new(),
        cache: HashMap::new(),
        name_counter: HashMap::new(),
    };

    // Infer type names.
    if name_type_ids {
        auto_namer.collect_ty_names(&itm)?;
        auto_namer.collect_ty_names(&itm)?;
    }

    // Infer constant names.
    if name_const_ids {
        auto_namer.collect_const_names(&itm)?;
    }

    // Manually annotated ID by OpName.
    if name_ids {
        auto_namer.collect_annotated_names(&itm)?;
    }

    Ok(auto_namer.names)
}
