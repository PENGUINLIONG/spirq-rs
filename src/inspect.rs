//! Inspect SPIR-V function parsing.
use crate::parse::Instr;
use crate::reflect::ReflectIntermediate;

pub trait Inspector {
    /// For each instruction iterated in a function parse, the inspector receive
    /// the instruction after the reflector finishes processing it.
    fn inspect<'a>(&mut self, itm: &ReflectIntermediate<'a>, instr: &Instr<'a>);
}

/// Inspector that does literally nothing.
pub(crate) struct NopInspector();
impl Inspector for NopInspector {
    fn inspect<'a>(&mut self, _itm: &ReflectIntermediate<'a>, _instr: &Instr<'a>) {}
}

/// Inspector that calls a function wrapped up in it.
pub(crate) struct FnInspector<F: FnMut(&ReflectIntermediate<'_>, &Instr<'_>)>(pub F);
impl<F: FnMut(&ReflectIntermediate<'_>, &Instr<'_>)> Inspector for FnInspector<F> {
    fn inspect<'a>(&mut self, itm: &ReflectIntermediate<'a>, instr: &Instr<'a>) {
        self.0(itm, instr)
    }
}
