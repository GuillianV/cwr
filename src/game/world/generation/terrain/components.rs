use std::ops::RangeInclusive;

use bevy::prelude::info_span;
use itertools::iproduct;

use crate::game::world::{
    block::components::Block,
    generation::{
        constants::{CHUNK_S1, CHUNK_S1I, MAX_GEN_HEIGHT},
        pos::ColPos,
    },
    voxel::resources::VoxelWorld,
};

pub fn gen_terrain(world: &VoxelWorld, col: ColPos) {
    //let landratio = self.config.get("land_ratio").copied().unwrap_or(0.4);

    for (dx, dz) in iproduct!(0..CHUNK_S1, 0..CHUNK_S1) {
        // Randomly decide the starting y-coordinate for Ground
        let ground_start_y :i32 = 3;

        // Fill with Air from the top to the ground_start_y
        world.set_yrange(
            col,
            (dx, dz),
            MAX_GEN_HEIGHT as i32 - ground_start_y,
            MAX_GEN_HEIGHT,
            Block::Ground(),
        );

        // Fill with Ground from ground_start_y to the bottom
        world.set_yrange(col, (dx, dz), ground_start_y, ground_start_y as usize, Block::Ground());
    }
}

fn pos_to_range(pos: ColPos) -> [RangeInclusive<i32>; 2] {
    let x = pos.z * CHUNK_S1I;
    let y = pos.x * CHUNK_S1I;
    [x..=(x + CHUNK_S1I - 1), y..=(y + CHUNK_S1I - 1)]
}
