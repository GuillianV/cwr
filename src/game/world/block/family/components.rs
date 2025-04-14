use bevy::prelude::*;

use crate::game::world::block::components::Block;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BlockFamily {
    Air,
    Ground,
}
