use std::collections::HashMap;

use spirq_core::ty::Type;

/// A tiny text-based type registry. It serves mainly to decide the input format
/// of OpConstant's LiteralContextDependentNumber. So we only process scalar
/// types here.
struct TinyTypeRegistry {
    ty_map: HashMap<u32, Type>,
}
impl TinyTypeRegistry {
    pub fn reg_ty(&mut self, opcode: u32) {
        self.ty_map.insert(id, ty);
    }
}
