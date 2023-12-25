//! Annotation instruction (decorations and names) utilities.
use fnv::FnvHashMap as HashMap;

use crate::{
    error::{anyhow, Result},
    ty::{AccessType, Type},
    var::{DescriptorBinding, InterfaceLocation},
};

pub use spirv::Decoration;

type VariableId = u32;
type InstrId = u32;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct DecorationKey {
    pub id: InstrId,
    pub member_idx: Option<u32>,
    pub deco: Decoration,
}
impl DecorationKey {
    pub fn new(id: InstrId, deco: Decoration) -> Self {
        Self {
            id,
            member_idx: None,
            deco,
        }
    }
    pub fn new_member(id: InstrId, member_idx: u32, deco: Decoration) -> Self {
        Self {
            id,
            member_idx: Some(member_idx),
            deco,
        }
    }
}

#[derive(Default)]
pub struct DecorationRegistry<'a> {
    deco_map: HashMap<DecorationKey, &'a [u32]>,
}
impl<'a> DecorationRegistry<'a> {
    fn set_impl(&mut self, key: DecorationKey, operands: &'a [u32]) -> Result<()> {
        use std::collections::hash_map::Entry;
        match self.deco_map.entry(key) {
            Entry::Vacant(entry) => {
                entry.insert(operands);
                Ok(())
            }
            Entry::Occupied(_) => Err(anyhow!("duplicate decoration at id {}", key.id)),
        }
    }
    fn get_impl(&self, key: DecorationKey) -> Result<&'a [u32]> {
        self.deco_map
            .get(&key)
            .copied()
            .ok_or(anyhow!("missing decoration at id {}", key.id))
    }

    pub fn set(&mut self, id: InstrId, deco: Decoration, operands: &'a [u32]) -> Result<()> {
        self.set_impl(DecorationKey::new(id, deco), operands)
    }
    pub fn set_member(
        &mut self,
        id: InstrId,
        member_idx: u32,
        deco: Decoration,
        operands: &'a [u32],
    ) -> Result<()> {
        self.set_impl(DecorationKey::new_member(id, member_idx, deco), operands)
    }
    pub fn get(&self, id: InstrId, deco: Decoration) -> Result<&'a [u32]> {
        self.get_impl(DecorationKey::new(id, deco))
    }
    pub fn get_member(&self, id: InstrId, member_idx: u32, deco: Decoration) -> Result<&'a [u32]> {
        self.get_impl(DecorationKey::new_member(id, member_idx, deco))
    }

    pub fn get_u32(&self, id: InstrId, deco: Decoration) -> Result<u32> {
        self.get(id, deco)
            .and_then(|x| {
                x.get(0).ok_or(anyhow!(
                    "expected a single operand for decoration {:?} at id {}",
                    deco,
                    id
                ))
            })
            .copied()
    }
    pub fn get_member_u32(&self, id: InstrId, member_idx: u32, deco: Decoration) -> Result<u32> {
        self.get_member(id, member_idx, deco)
            .and_then(|x| {
                x.get(0).ok_or(anyhow!(
                    "expected a single operand for member decoration {:?} at id {} for member {}",
                    deco,
                    id,
                    member_idx
                ))
            })
            .copied()
    }

    pub fn contains(&self, id: InstrId, deco: Decoration) -> bool {
        self.deco_map.contains_key(&DecorationKey::new(id, deco))
    }
    pub fn contains_member(&self, id: InstrId, member_idx: u32, deco: Decoration) -> bool {
        self.deco_map
            .contains_key(&DecorationKey::new_member(id, member_idx, deco))
    }

    pub fn get_all(&self, deco: Decoration) -> impl Iterator<Item = (InstrId, &[u32])> {
        self.deco_map
            .iter()
            .filter(move |(key, _)| key.deco == deco)
            .map(|(key, value)| (key.id, *value))
    }

    /// Get the location-component pair of an interface variable.
    pub fn get_var_location(&self, var_id: VariableId) -> Result<InterfaceLocation> {
        let comp = self.get_u32(var_id, Decoration::Component).unwrap_or(0);
        self.get_u32(var_id, Decoration::Location)
            .map(|loc| InterfaceLocation::new(loc, comp))
    }
    /// Get the set-binding pair of a descriptor resource.
    pub fn get_var_desc_bind(&self, var_id: VariableId) -> Result<DescriptorBinding> {
        let desc_set = self.get_u32(var_id, Decoration::DescriptorSet).unwrap_or(0);
        self.get_u32(var_id, Decoration::Binding)
            .map(|bind_point| DescriptorBinding::new(desc_set, bind_point))
    }
    /// Get the set-binding pair of a descriptor resource, but the binding point
    /// is forced to 0 if it's not specified in SPIR-V source.
    pub fn get_var_desc_bind_or_default(&self, var_id: VariableId) -> DescriptorBinding {
        self.get_var_desc_bind(var_id)
            .unwrap_or(DescriptorBinding::new(0, 0))
    }
    /// Get the access type of an memory object.
    pub fn get_desc_access_ty(&self, id: InstrId, ty: &Type) -> Option<AccessType> {
        self.get_access_ty_from_deco(id).and_then(|x| {
            // Use the stricter one.
            if x == AccessType::ReadWrite {
                match ty.access_ty() {
                    Some(x) => Some(x),
                    None => Some(AccessType::ReadWrite),
                }
            } else {
                Some(x)
            }
        })
    }
    pub fn get_access_ty_from_deco(&self, id: InstrId) -> Option<AccessType> {
        let write_only = self.contains(id, Decoration::NonReadable);
        let read_only = self.contains(id, Decoration::NonWritable);
        match (write_only, read_only) {
            (true, true) => None,
            (true, false) => Some(AccessType::WriteOnly),
            (false, true) => Some(AccessType::ReadOnly),
            (false, false) => Some(AccessType::ReadWrite),
        }
    }
    pub fn get_member_access_ty_from_deco(
        &self,
        id: InstrId,
        member_idx: u32,
    ) -> Option<AccessType> {
        let write_only = self.contains_member(id, member_idx, Decoration::NonReadable);
        let read_only = self.contains_member(id, member_idx, Decoration::NonWritable);
        match (write_only, read_only) {
            (true, true) => None,
            (true, false) => Some(AccessType::WriteOnly),
            (false, true) => Some(AccessType::ReadOnly),
            (false, false) => Some(AccessType::ReadWrite),
        }
    }

    /// Get the input attachment index of the variable.
    pub fn get_var_input_attm_idx(&self, var_id: VariableId) -> Result<u32> {
        self.get_u32(var_id, Decoration::InputAttachmentIndex)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct NameKey {
    pub id: InstrId,
    pub member_idx: Option<u32>,
}
#[derive(Default)]
pub struct NameRegistry<'a> {
    name_map: HashMap<NameKey, &'a str>,
}
impl<'a> NameRegistry<'a> {
    // Names are debuf information. Not important so ID collisions are ignored.
    pub fn set(&mut self, id: InstrId, name: &'a str) {
        use std::collections::hash_map::Entry;
        let key = NameKey {
            id,
            member_idx: None,
        };
        match self.name_map.entry(key) {
            Entry::Vacant(entry) => {
                entry.insert(name);
            }
            _ => {}
        }
    }
    pub fn set_member(&mut self, id: InstrId, member_idx: u32, name: &'a str) {
        use std::collections::hash_map::Entry;
        let key = NameKey {
            id,
            member_idx: Some(member_idx),
        };
        match self.name_map.entry(key) {
            Entry::Vacant(entry) => {
                entry.insert(name);
            }
            _ => {}
        }
    }

    pub fn get(&self, id: InstrId) -> Option<&'a str> {
        self.name_map
            .get(&NameKey {
                id,
                member_idx: None,
            })
            .copied()
    }
    pub fn get_member(&self, id: InstrId, member_idx: u32) -> Option<&'a str> {
        self.name_map
            .get(&NameKey {
                id,
                member_idx: Some(member_idx),
            })
            .copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&NameKey, &&'a str)> {
        self.name_map.iter()
    }
}
