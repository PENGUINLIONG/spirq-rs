use std::fmt;
use crate::ty::Type;

#[derive(Clone)]
pub enum Seg<'a> {
    NamedIndex(usize, &'a str),
    Index(usize),
}
impl<'a> fmt::Debug for Seg<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Seg::NamedIndex(_, name) => f.write_str(name),
            Seg::Index(i) => write!(f, "{}", i),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MemberVariableRouting<'a> {
    pub sym: Vec<Seg<'a>>,
    pub offset: usize,
    pub ty: &'a Type,
}

struct WalkFrame<'a> {
    sym_stem: Option<Vec<Seg<'a>>>,
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
                    let seg = if let Some(name) = &member.name {
                        Seg::NamedIndex(i, &name)
                    } else {
                        Seg::Index(i)
                    };
                    Some((&member.ty, member.offset, seg))
                },
                Type::Array(arr_ty) => {
                    // Unsized buffer are treated as 0-sized.
                    if i < arr_ty.nrepeat().unwrap_or(0) as usize {
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
                    let offset = frame.base_offset + offset;
                    let ty = child_ty;
                    let sym = if let Some(sym_stem) = &frame.sym_stem {
                        let mut sym = sym_stem.clone();
                        sym.push(seg);
                        sym
                    } else { vec![seg] };
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
