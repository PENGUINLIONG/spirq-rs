//! Inspect SPIR-V function parsing.
use crate::parse::Instr;
use crate::reflect::ReflectIntermediate;

pub trait Inspector {
    /// For each instruction iterated in a function parse, the inspector receive
    /// the instruction after the reflector finishes processing it.
    fn inspect<'a>(
        &mut self,
        itm: &mut ReflectIntermediate<'a>,
        instr: &Instr,
    ) -> anyhow::Result<()>;

    /// Chain two inspectors together. The second inspector will be called after
    /// the first one.
    fn chain<'a, I2: Inspector>(&'a mut self, second: &'a mut I2) -> Chain<Self, I2>
    where
        Self: Sized,
    {
        Chain {
            first: self,
            second: second,
        }
    }
}

/// Inspector that calls a function wrapped up in it.
pub(crate) struct FnInspector<F: FnMut(&mut ReflectIntermediate<'_>, &Instr)>(pub F);
impl<F: FnMut(&mut ReflectIntermediate<'_>, &Instr)> Inspector for FnInspector<F> {
    fn inspect<'a>(
        &mut self,
        itm: &mut ReflectIntermediate<'a>,
        instr: &Instr,
    ) -> anyhow::Result<()> {
        Ok(self.0(itm, instr))
    }
}

pub struct Chain<'a, I1: Inspector, I2: Inspector> {
    first: &'a mut I1,
    second: &'a mut I2,
}
impl<I1: Inspector, I2: Inspector> Inspector for Chain<'_, I1, I2> {
    fn inspect<'a>(
        &mut self,
        itm: &mut ReflectIntermediate<'a>,
        instr: &Instr,
    ) -> anyhow::Result<()> {
        self.first.inspect(itm, instr)?;
        self.second.inspect(itm, instr)
    }
}
