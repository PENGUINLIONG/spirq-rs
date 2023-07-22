use std::fmt;

/// Descriptor set and binding point carrier.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Default, Clone, Copy)]
pub struct DescriptorBinding(u32, u32);
impl DescriptorBinding {
    pub fn new(desc_set: u32, bind_point: u32) -> Self {
        DescriptorBinding(desc_set, bind_point)
    }

    pub fn set(&self) -> u32 {
        self.0
    }
    pub fn bind(&self) -> u32 {
        self.1
    }
    pub fn into_inner(self) -> (u32, u32) {
        (self.0, self.1)
    }
}
impl fmt::Display for DescriptorBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(set={}, bind={})", self.0, self.1)
    }
}
impl fmt::Debug for DescriptorBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self as &dyn fmt::Display).fmt(f)
    }
}

/// Interface variable location and component.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Default, Clone, Copy)]
pub struct InterfaceLocation(u32, u32);
impl InterfaceLocation {
    pub fn new(loc: u32, comp: u32) -> Self {
        InterfaceLocation(loc, comp)
    }

    pub fn loc(&self) -> u32 {
        self.0
    }
    pub fn comp(&self) -> u32 {
        self.1
    }
    pub fn into_inner(self) -> (u32, u32) {
        (self.0, self.1)
    }
}
impl fmt::Display for InterfaceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(loc={}, comp={})", self.0, self.1)
    }
}
impl fmt::Debug for InterfaceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self as &dyn fmt::Display).fmt(f)
    }
}

/// Specialization constant ID.
pub type SpecId = u32;

/// Variable locator.
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Locator {
    Input(InterfaceLocation),
    Output(InterfaceLocation),
    Descriptor(DescriptorBinding),
    PushConstant,
    SpecConstant(SpecId),
}
