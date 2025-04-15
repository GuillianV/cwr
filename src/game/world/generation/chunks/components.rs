use bevy::{
    asset::RenderAssetUsages, log::info_span, math::{I64Vec3, Vec3A}, render::{mesh::{Indices, Mesh, PrimitiveTopology}, primitives::{Aabb, Frustum}}
};
use binary_greedy_meshing as bgm;
use itertools::Itertools;
use std::{
    collections::BTreeSet,
    ops::{Deref, DerefMut},
};

use crate::{
    game::world::{
        block::components::{Block, Face},
        generation::{
            constants::{CHUNK_S1, CHUNKP_S1, CHUNKP_S2, CHUNKP_S3, MASK_6, MASK_XYZ},
            pos::{ChunkedPos, ColedPos},
        },
    },
    render::constants::ATTRIBUTE_VOXEL_DATA,
    util::palette::Palette,
};

#[derive(Debug)]
pub struct Chunk {
    pub data: PackedUints,
    pub palette: Palette<Block>,
}

pub fn linearize(x: usize, y: usize, z: usize) -> usize {
    z + x * CHUNKP_S1 + y * CHUNKP_S2
}

pub fn pad_linearize(x: usize, y: usize, z: usize) -> usize {
    z + 1 + (x + 1) * CHUNKP_S1 + (y + 1) * CHUNKP_S2
}

impl Chunk {
    pub fn get(&self, (x, y, z): ChunkedPos) -> &Block {
        &self.palette[self.data.get(pad_linearize(x, y, z))]
    }

    pub fn set(&mut self, (x, y, z): ChunkedPos, block: Block) {
        let idx = pad_linearize(x, y, z);
        self.data.set(idx, self.palette.index(block));
    }

    pub fn set_yrange(&mut self, (x, top, z): ChunkedPos, height: usize, block: Block) {
        let value = self.palette.index(block);
        // Note: we do end+1 because set_range(_step) is not inclusive
        self.data.set_range_step(
            pad_linearize(x, top - height, z),
            pad_linearize(x, top, z) + 1,
            CHUNKP_S2,
            value,
        );
    }

    // Used for efficient construction of mesh data
    pub fn copy_column(&self, buffer: &mut [Block], (x, z): ColedPos, lod: usize) {
        let start = pad_linearize(x, 0, z);
        let mut i = 0;
        for idx in (start..(start + CHUNK_S1)).step_by(lod) {
            buffer[i] = self.palette[self.data.get(idx)];
            i += 1;
        }
    }

    pub fn top(&self, (x, z): ColedPos) -> (&Block, usize) {
        for y in (0..CHUNK_S1).rev() {
            let b_idx = self.data.get(pad_linearize(x, y, z));
            if b_idx > 0 {
                return (&self.palette[b_idx], y);
            }
        }
        (&self.palette[0], 0)
    }

    pub fn set_if_empty(&mut self, (x, y, z): ChunkedPos, block: Block) -> bool {
        let idx = pad_linearize(x, y, z);
        false
        // self.data.set(idx, self.palette.index(block));
        // true
    }
}


pub fn intersects_aabb(frustum: &Frustum, aabb: &Aabb) -> bool {
    let min = aabb.min();
    let max = aabb.max();
    for half_space in &frustum.half_spaces[..5] {
        let mask: [u32; 3] = half_space.normal().cmpgt(Vec3A::ZERO).into();
        let mask = Vec3A::from_array(mask.map(|b| b as f32));
        let vmax = (mask * max + (1.0 - mask) * min).extend(1.0);
        let normal = half_space.normal_d();
        if normal.dot(vmax) < 0.0 {
            return false;
        }
    }
    true
}

pub fn face_visible(from: &I64Vec3, coord: I64Vec3, face: &Face) -> bool {
    let rel_coord = coord - *from;
    let res = match face {
        Face::Left => rel_coord.x >= 0,
        Face::Down => rel_coord.y >= 0,
        Face::Back => rel_coord.z >= 0,
        Face::Right => rel_coord.x <= 0,
        Face::Up => rel_coord.y <= 0,
        Face::Front => rel_coord.z <= 0,
    };
    res
}

impl From<&[Block]> for Chunk {
    fn from(values: &[Block]) -> Self {
        let mut palette = Palette::new();
        let values = values
            .iter()
            .map(|v| palette.index(v.clone()))
            .collect_vec();
        let data = PackedUints::from(values.as_slice());
        Chunk { data, palette }
    }
}

impl Chunk {
    pub fn new() -> Self {
        let mut palette = Palette::new();
        palette.index(Block::Air());
        palette.index(Block::Ground());
        Chunk {
            data: PackedUints::new(CHUNKP_S3),
            palette: palette,
        }
    }
}

impl Chunk {
    pub fn voxel_data_lod(&self, lod: usize) -> Vec<u16> {
        let voxels = self.data.unpack_u16();
        if lod == 1 {
            return voxels;
        }
        let mut res = vec![0; CHUNKP_S3];
        for x in 0..CHUNK_S1 {
            for y in 0..CHUNK_S1 {
                for z in 0..CHUNK_S1 {
                    let lod_i = pad_linearize(x / lod, y / lod, z / lod);
                    if res[lod_i] == 0 {
                        res[lod_i] = voxels[pad_linearize(x, y, z)];
                    }
                }
            }
        }
        res
    }

    /// Doesn't work with lod > 2, because chunks are of size 62 (to get to 64 with padding) and 62 = 2*31
    /// TODO: make it work with lod > 2 if necessary (by truncating quads)
    pub fn create_face_meshes(&self, lod: usize) -> [Option<Mesh>; 6] {
        // Gathering binary greedy meshing input data
        let mesh_data_span = info_span!("mesh voxel data", name = "mesh voxel data").entered();
        let voxels = self.voxel_data_lod(lod);
        let mut mesh_data = bgm::MeshData::new();
        mesh_data_span.exit();
        let mesh_build_span = info_span!("mesh build", name = "mesh build").entered();
        let transparents =
            BTreeSet::from_iter(self.palette.iter().enumerate().filter_map(|(i, block)| {
                if i != 0 && !block.is_opaque() {
                    Some(i as u16)
                } else {
                    None
                }
            }));
        bgm::mesh(&voxels, &mut mesh_data, transparents);
        let mut meshes = core::array::from_fn(|_| None);
        for (face_n, quads) in mesh_data.quads.iter().enumerate() {
            let mut voxel_data: Vec<[u32; 2]> = Vec::with_capacity(quads.len() * 4);
            let indices = bgm::indices(quads.len());
            let face: Face = face_n.into();
            for quad in quads {
                let voxel_i = (quad >> 32) as usize;
                let w = MASK_6 & (quad >> 18);
                let h = MASK_6 & (quad >> 24);
                let xyz = MASK_XYZ & quad;
                let block = self.palette[voxel_i];
                let layer = 0;
                let color = 0b010_101_001;
                let vertices = face.vertices_packed(xyz as u32, w as u32, h as u32, lod as u32);
                let quad_info = (layer << 12) | (color << 3) | face_n as u32;
                voxel_data.extend_from_slice(&[
                    [vertices[0], quad_info],
                    [vertices[1], quad_info],
                    [vertices[2], quad_info],
                    [vertices[3], quad_info],
                ]);
            }
            meshes[face_n] = Some(
                Mesh::new(
                    PrimitiveTopology::TriangleList,
                    RenderAssetUsages::RENDER_WORLD,
                )
                .with_inserted_attribute(ATTRIBUTE_VOXEL_DATA, voxel_data)
                .with_inserted_indices(Indices::U32(indices)),
            )
        }
        mesh_build_span.exit();
        meshes
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
