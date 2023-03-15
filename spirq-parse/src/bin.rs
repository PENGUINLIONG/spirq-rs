use std::{iter::FromIterator, convert::TryInto};

/// SPIR-V program binary.
#[derive(Debug, Default, Clone)]
pub struct SpirvBinary(Vec<u32>);
impl From<Vec<u32>> for SpirvBinary {
    fn from(x: Vec<u32>) -> Self {
        SpirvBinary(x)
    }
}
impl From<&[u32]> for SpirvBinary {
    fn from(x: &[u32]) -> Self {
        SpirvBinary(x.to_owned())
    }
}
impl FromIterator<u32> for SpirvBinary {
    fn from_iter<I: IntoIterator<Item = u32>>(iter: I) -> Self {
        SpirvBinary(iter.into_iter().collect::<Vec<u32>>())
    }
}
impl From<&[u8]> for SpirvBinary {
    fn from(x: &[u8]) -> Self {
        if x.len() == 0 {
            return SpirvBinary::default();
        }
        x.chunks_exact(4)
            .map(|x| x.try_into().unwrap())
            .map(match x[0] {
                0x03 => u32::from_le_bytes,
                0x07 => u32::from_be_bytes,
                _ => return SpirvBinary::default(),
            })
            .collect::<SpirvBinary>()
    }
}
impl From<Vec<u8>> for SpirvBinary {
    fn from(x: Vec<u8>) -> Self {
        SpirvBinary::from(x.as_ref() as &[u8])
    }
}

impl SpirvBinary {
    pub fn words(&self) -> &[u32] {
        &self.0
    }
    pub fn into_words(self) -> Vec<u32> {
        self.0
    }
}
