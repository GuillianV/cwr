use std::ops::RangeInclusive;

use itertools::iproduct;

use crate::game::world::{
    block::components::{Block, BlockFamily, Blocks}, generation::{
        constants::{CHUNK_S1, CHUNK_S1I},
        pos::ColPos,
    }, voxel::resources::VoxelWorld
};

pub fn gen_terrain(world: &VoxelWorld, col: ColPos) {
    //let landratio = self.config.get("land_ratio").copied().unwrap_or(0.4);

    for (dx, dz) in iproduct!(0..CHUNK_S1, 0..CHUNK_S1) {
        // Randomly decide the starting y-coordinate for Ground

        // Fill with Ground from ground_start_y to the bottom
        world.set_yrange(col, (dx, dz), 0, 1 as usize, Blocks::ground());
    }
}

fn pos_to_range(pos: ColPos) -> [RangeInclusive<i32>; 2] {
    let x = pos.z * CHUNK_S1I;
    let y = pos.x * CHUNK_S1I;
    [x..=(x + CHUNK_S1I - 1), y..=(y + CHUNK_S1I - 1)]
}
