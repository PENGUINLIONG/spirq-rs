use anyhow::Result;
use spirv::{MAJOR_VERSION, MINOR_VERSION};
use std::{convert::TryInto, iter::FromIterator};

use super::{Instrs, Instruction};

#[derive(Debug, Clone)]
pub struct SpirvHeader {
    pub magic: u32,
    pub version: u32,
    pub generator: u32,
    pub bound: u32,
    pub schema: u32,
}
impl Default for SpirvHeader {
    fn default() -> Self {
        SpirvHeader {
            magic: 0x07230203,
            version: ((MAJOR_VERSION as u32) << 16) | ((MINOR_VERSION as u32) << 8),
            generator: 0,
            bound: 0,
            schema: 0,
        }
    }
}
impl SpirvHeader {
    pub fn new(version: u32, generator: u32) -> Self {
        SpirvHeader {
            version,
            generator,
            ..Default::default()
        }
    }
    pub fn words(&self) -> [u32; 5] {
        [
            self.magic,
            self.version,
            self.generator,
            self.bound,
            self.schema,
        ]
    }
}

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

    pub fn instrs(&self) -> Result<Instrs> {
        const HEADER_LEN: usize = 5;
        Instrs::new(&self.words()[HEADER_LEN..])
    }

    pub fn header(&self) -> Option<SpirvHeader> {
        let header = &self.words()[..5];
        if header.len() < 5 {
            return None;
        }
        Some(SpirvHeader {
            magic: header[0],
            version: header[1],
            generator: header[2],
            bound: header[3],
            schema: header[4],
        })
    }
}
