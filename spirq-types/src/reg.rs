use std::collections::HashMap;

use anyhow::{Result, anyhow};
use crate::Type;

pub type TypeId = u32;

#[derive(Default)]
pub struct TypeRegistry {
    ty_map: HashMap<TypeId, Type>,
}
impl TypeRegistry {
    /// Allocate a type handle referred by `ty_id` and optionally assign a type
    /// to it.
    pub fn set(&mut self, id: TypeId, ty: Type) -> Result<()> {
        use std::collections::hash_map::Entry;
        match self.ty_map.entry(id) {
            Entry::Vacant(entry) => {
                entry.insert(ty);
                Ok(())
            }
            Entry::Occupied(mut entry) => {
                if entry.get().is_devaddr() && ty.is_devptr() {
                    entry.insert(ty);
                    Ok(())
                } else {
                    Err(anyhow!("type collision at id {}: {:?} vs {:?}", id, entry.get(), ty))
                }
            },
        }
    }

    /// Get the type identified by `handle`.
    pub fn get(&self, id: TypeId) -> Result<&Type> {
        self.ty_map.get(&id)
            .ok_or(anyhow!("missing type id {}", id))
    }

}
