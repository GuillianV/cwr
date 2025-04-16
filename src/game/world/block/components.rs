use bevy::{ecs::component::Component, math::Vec3};

use crate::game::world::generation::pos::BlockPos;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BlockFamily {
    Air,
    Dirt,
    Stone,
    Deepslate,
}

pub struct Blocks {}

impl Blocks {
    pub fn dirt() -> Block {
        Block::new(BlockFamily::Dirt)
    }

    pub fn stone() -> Block {
        Block::new(BlockFamily::Stone)
    }

    pub fn deepslate() -> Block {
        Block::new(BlockFamily::Deepslate)
    }

    pub fn air() -> Block {
        Block::new(BlockFamily::Air)
    }


    pub fn list () -> Vec<Block> {
        vec![Blocks::air(),Blocks::dirt(), Blocks::stone(), Blocks::deepslate()]
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Block {
    pub family: BlockFamily,
}

impl Default for Block {
    fn default() -> Self {
        Self {
            family: BlockFamily::Air,
        }
    }
}

impl Block {
    pub fn new(family: BlockFamily) -> Self {
        Self { family }
    }

    pub fn is_traversable(&self) -> bool {
        match self.family {
            BlockFamily::Air => true,

            _ => false,
        }
    }

    pub fn is_opaque(&self) -> bool {
        match self.family {
            BlockFamily::Air => false,
            _ => true,
        }
    }
}

pub struct BlockRayCastHit {
    pub pos: BlockPos,
    pub normal: Vec3,
}

impl PartialEq for BlockRayCastHit {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

#[derive(Component, PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Face {
    Left,
    Down,
    Back,
    Right,
    Up,
    Front,
}

impl From<u8> for Face {
    fn from(value: u8) -> Self {
        assert!(value < 6);
        match value {
            0 => Self::Up,
            1 => Self::Down,
            2 => Self::Right,
            3 => Self::Left,
            4 => Self::Front,
            5 => Self::Back,
            _ => unreachable!(),
        }
    }
}

impl From<usize> for Face {
    fn from(value: usize) -> Self {
        (value as u8).into()
    }
}

fn packed_xyz(x: u32, y: u32, z: u32) -> u32 {
    (z << 12) | (y << 6) | x
}

fn vertex_info(xyz: u32, u: u32, v: u32) -> u32 {
    (v << 24) | (u << 18) | xyz
}

impl Face {
    pub fn vertices_packed(&self, xyz: u32, w: u32, h: u32, lod: u32) -> [u32; 4] {
        let xyz = xyz * lod;
        let w_ = w * lod;
        let h_ = h * lod;
        match self {
            Face::Left => [
                vertex_info(xyz, h, w),
                vertex_info(xyz + packed_xyz(0, 0, h_), 0, w),
                vertex_info(xyz + packed_xyz(0, w_, 0), h, 0),
                vertex_info(xyz + packed_xyz(0, w_, h_), 0, 0),
            ],
            Face::Down => [
                vertex_info(xyz - packed_xyz(w_, 0, 0) + packed_xyz(0, 0, h_), w, h),
                vertex_info(xyz - packed_xyz(w_, 0, 0), w, 0),
                vertex_info(xyz + packed_xyz(0, 0, h_), 0, h),
                vertex_info(xyz, 0, 0),
            ],
            Face::Back => [
                vertex_info(xyz, w, h),
                vertex_info(xyz + packed_xyz(0, h_, 0), w, 0),
                vertex_info(xyz + packed_xyz(w_, 0, 0), 0, h),
                vertex_info(xyz + packed_xyz(w_, h_, 0), 0, 0),
            ],
            Face::Right => [
                vertex_info(xyz, 0, 0),
                vertex_info(xyz + packed_xyz(0, 0, h_), h, 0),
                vertex_info(xyz - packed_xyz(0, w_, 0), 0, w),
                vertex_info(xyz + packed_xyz(0, 0, h_) - packed_xyz(0, w_, 0), h, w),
            ],
            Face::Up => [
                vertex_info(xyz + packed_xyz(w_, 0, h_), w, h),
                vertex_info(xyz + packed_xyz(w_, 0, 0), w, 0),
                vertex_info(xyz + packed_xyz(0, 0, h_), 0, h),
                vertex_info(xyz, 0, 0),
            ],
            Face::Front => [
                vertex_info(xyz - packed_xyz(w_, 0, 0) + packed_xyz(0, h_, 0), 0, 0),
                vertex_info(xyz - packed_xyz(w_, 0, 0), 0, h),
                vertex_info(xyz + packed_xyz(0, h_, 0), w, 0),
                vertex_info(xyz, w, h),
            ],
        }
    }
}
