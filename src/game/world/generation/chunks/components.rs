use std::ops::{Deref, DerefMut};

use crate::game::world::generation::{
    constants::{CHUNKP_S1, CHUNKP_S2, CHUNKP_S3},
    pos::ChunkedPos,
};

#[derive(Debug)]
pub struct Chunk {
    pub data: PackedUints,
}

pub fn linearize(x: usize, y: usize, z: usize) -> usize {
    z + x * CHUNKP_S1 + y * CHUNKP_S2
}

pub fn pad_linearize(x: usize, y: usize, z: usize) -> usize {
    z + 1 + (x + 1) * CHUNKP_S1 + (y + 1) * CHUNKP_S2
}

impl Chunk {
    pub fn get(&self, (x, y, z): ChunkedPos) {}

    pub fn set(&mut self, (x, y, z): ChunkedPos) {
        let idx = pad_linearize(x, y, z);
    }

    pub fn set_yrange(&mut self, (x, top, z): ChunkedPos, height: usize) {
        self.data.set_range_step(
            pad_linearize(x, top - height, z),
            pad_linearize(x, top, z) + 1,
            CHUNKP_S2,
            0,
        );
    }
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            data: PackedUints::new(CHUNKP_S3),
        }
    }
}

pub struct TrackedChunk {
    chunk: Chunk,
    pub changed: bool,
}

impl TrackedChunk {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            changed: false,
        }
    }
}

impl Deref for TrackedChunk {
    type Target = Chunk;

    fn deref(&self) -> &Self::Target {
        &self.chunk
    }
}

impl DerefMut for TrackedChunk {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.chunk
    }
}

const U4_IN_U8: usize = 8 / 4;
const PARITY_MASK: usize = U4_IN_U8 - 1;

#[derive(Debug, Clone)]
pub enum PackedEnum {
    U4(Vec<u8>),
    U8(Vec<u8>),
    U16(Vec<u16>),
    U32(Vec<u32>),
}

impl PackedEnum {
    pub fn mask(&self) -> usize {
        match self {
            Self::U4(_) => 0b1111 as usize,
            Self::U8(_) => u8::MAX as usize,
            Self::U16(_) => u16::MAX as usize,
            Self::U32(_) => u32::MAX as usize,
        }
    }

    #[inline(always)]
    fn get(&self, i: usize) -> usize {
        match self {
            Self::U4(data) => {
                let shift = 4 * (i & PARITY_MASK);
                ((data[i / U4_IN_U8] >> shift) & 0b1111) as usize
            }
            Self::U8(data) => data[i] as usize,
            Self::U16(data) => data[i] as usize,
            Self::U32(data) => data[i] as usize,
        }
    }

    #[inline(always)]
    fn set(&mut self, i: usize, value: usize) {
        match self {
            Self::U4(data) => {
                let shift: usize = 4 * (i & PARITY_MASK);
                let mask = 0b1111 << shift;
                let i = i / U4_IN_U8;
                data[i] &= !mask;
                data[i] |= (value as u8) << shift;
            }
            Self::U8(data) => {
                data[i] = value as u8;
            }
            Self::U16(data) => {
                data[i] = value as u16;
            }
            Self::U32(data) => {
                data[i] = value as u32;
            }
        }
    }

    fn set_range(&mut self, start: usize, end: usize, value: usize) {
        match self {
            Self::U4(data) => {
                // NOTE: this part assumes we're storing u4 in u8 (unlike the rest of the code)
                for i in start..end {
                    let shift: usize = 4 * (i & PARITY_MASK);
                    let mask = 0b1111 << shift;
                    let i = i / U4_IN_U8;
                    data[i] &= !mask;
                    data[i] |= (value as u8) << shift;
                }
            }
            Self::U8(data) => {
                for i in start..end {
                    data[i] = value as u8;
                }
            }
            Self::U16(data) => {
                for i in start..end {
                    data[i] = value as u16;
                }
            }
            Self::U32(data) => {
                for i in start..end {
                    data[i] = value as u32;
                }
            }
        }
    }

    fn set_range_step(&mut self, start: usize, end: usize, step: usize, value: usize) {
        match self {
            Self::U4(data) => {
                // NOTE: this part assumes we're storing u4 in u8 (unlike the rest of the code)
                for i in (start..end).step_by(step) {
                    let shift: usize = 4 * (i & PARITY_MASK);
                    let mask = 0b1111 << shift;
                    let i = i / U4_IN_U8;
                    data[i] &= !mask;
                    data[i] |= (value as u8) << shift;
                }
            }
            Self::U8(data) => {
                for i in (start..end).step_by(step) {
                    data[i] = value as u8;
                }
            }
            Self::U16(data) => {
                for i in (start..end).step_by(step) {
                    data[i] = value as u16;
                }
            }
            Self::U32(data) => {
                for i in (start..end).step_by(step) {
                    data[i] = value as u32;
                }
            }
        }
    }

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = usize> + 'a> {
        match self {
            Self::U4(data) => Box::new(
                data.iter()
                    .flat_map(|a| [(a & 0b1111) as usize, (a >> 4) as usize]),
            ),
            Self::U8(data) => Box::new(data.iter().map(|a| *a as usize)),
            Self::U16(data) => Box::new(data.iter().map(|a| *a as usize)),
            Self::U32(data) => Box::new(data.iter().map(|a| *a as usize)),
        }
    }

    pub fn unpack_u8(&self) -> Vec<u8> {
        match self {
            Self::U4(data) => data.iter().flat_map(|a| [(a & 0b1111), (a >> 4)]).collect(),
            Self::U8(data) => data.iter().map(|a| *a as u8).collect(),
            Self::U16(data) => data.iter().map(|a| *a as u8).collect(),
            Self::U32(data) => data.iter().map(|a| *a as u8).collect(),
        }
    }

    pub fn unpack_u16(&self) -> Vec<u16> {
        match self {
            Self::U4(data) => data
                .iter()
                .flat_map(|a| [(a & 0b1111) as u16, (a >> 4) as u16])
                .collect(),
            Self::U8(data) => data.iter().map(|a| *a as u16).collect(),
            Self::U16(data) => data.clone(),
            Self::U32(data) => data.iter().map(|a| *a as u16).collect(),
        }
    }

    pub fn unpack_u32(&self) -> Vec<u32> {
        match self {
            Self::U4(data) => data
                .iter()
                .flat_map(|a| [(a & 0b1111) as u32, (a >> 4) as u32])
                .collect(),
            Self::U8(data) => data.iter().map(|a| *a as u32).collect(),
            Self::U16(data) => data.iter().map(|a| *a as u32).collect(),
            Self::U32(data) => data.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PackedUints {
    pub data: PackedEnum,
    pub mask: usize,
    pub length: usize,
}

impl PackedUints {
    pub fn new(length: usize) -> Self {
        PackedUints::filled(length, 0)
    }

    pub fn filled(length: usize, value: usize) -> Self {
        let bits = value.max(2).ilog2();
        let data = if bits < 4 {
            let value = value | (value << 4);
            PackedEnum::U4(vec![value as u8; (length + U4_IN_U8 - 1) / U4_IN_U8])
        } else if bits < 8 {
            PackedEnum::U8(vec![value as u8; length])
        } else if bits < 16 {
            PackedEnum::U16(vec![value as u16; length])
        } else {
            PackedEnum::U32(vec![value as u32; length])
        };
        PackedUints {
            data: data,
            mask: 0b1111,
            length: length,
        }
    }

    pub fn from(values: &[usize]) -> Self {
        let bits = values.iter().max().unwrap_or(&2).ilog2();
        let data = if bits < 4 {
            let mut res = vec![0; (values.len() + U4_IN_U8 - 1) / U4_IN_U8];
            for i in (0..values.len()).step_by(2) {
                res[i / 2] = (values[i + 1] << 4 | values[i]) as u8;
            }
            PackedEnum::U4(res)
        } else if bits < 8 {
            PackedEnum::U8(values.iter().map(|a| *a as u8).collect())
        } else if bits < 16 {
            PackedEnum::U16(values.iter().map(|a| *a as u16).collect())
        } else {
            PackedEnum::U32(values.iter().map(|a| *a as u32).collect())
        };
        PackedUints {
            mask: data.mask(),
            data,
            length: values.len(),
        }
    }

    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = usize> + 'a> {
        self.data.iter()
    }

    #[inline]
    pub fn get(&self, i: usize) -> usize {
        self.data.get(i)
    }

    #[inline]
    fn upscale_if_needed(&mut self, value: usize) {
        if (value & self.mask) != value {
            let bits = value.ilog2();
            self.data = if bits < 8 {
                PackedEnum::U8(
                    self.data
                        .iter()
                        .take(self.length)
                        .map(|a| a as u8)
                        .collect(),
                )
            } else if bits < 16 {
                PackedEnum::U16(
                    self.data
                        .iter()
                        .take(self.length)
                        .map(|a| a as u16)
                        .collect(),
                )
            } else {
                PackedEnum::U32(
                    self.data
                        .iter()
                        .take(self.length)
                        .map(|a| a as u32)
                        .collect(),
                )
            };
            self.mask = self.data.mask();
        }
    }

    #[inline]
    pub fn set(&mut self, i: usize, value: usize) {
        self.upscale_if_needed(value);
        self.data.set(i, value)
    }

    #[inline]
    pub fn set_range(&mut self, start: usize, end: usize, value: usize) {
        // check that both start and length are even
        self.upscale_if_needed(value);
        self.data.set_range(start, end, value);
    }

    #[inline]
    pub fn set_range_step(&mut self, start: usize, end: usize, step: usize, value: usize) {
        self.upscale_if_needed(value);
        self.data.set_range_step(start, end, step, value);
    }

    /// Same thing as iter().map(|value| value as u8).collect() but 5x faster
    #[inline]
    pub fn unpack_u8(&self) -> Vec<u8> {
        self.data.unpack_u8()
    }

    /// Same thing as iter().map(|value| value as u16).collect() but 5x faster
    #[inline]
    pub fn unpack_u16(&self) -> Vec<u16> {
        self.data.unpack_u16()
    }

    /// Same thing as iter().map(|value| value as u32).collect() but 5x faster
    #[inline]
    pub fn unpack_u32(&self) -> Vec<u32> {
        self.data.unpack_u32()
    }
}
