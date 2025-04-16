use bevy::log::info_span;
use bevy::prelude::*;
use bracket_noise::prelude::FastNoise;
use itertools::iproduct;
use std::ops::RangeInclusive;

use crate::game::world::{
    block::components::{Block, BlockFamily, Blocks},
    generation::{
        constants::{CHUNK_S1, CHUNK_S1I, MAX_GEN_HEIGHT},
        pos::ColPos,
    },
    voxel::resources::VoxelWorld,
};

pub fn gen_terrain(world: &VoxelWorld, col: ColPos, fast_noise: &FastNoise) {
    //let landratio = self.config.get("land_ratio").copied().unwrap_or(0.4);
    let ranges = pos_to_range(col);
    let offsets = ranges
        .clone()
        .map(|range| (range.start() / 1 as i32) as f32 / 1. as f32);

    let gen_span = info_span!("noise gen", name = "noise gen").entered();

    for (dx, dz) in iproduct!(0..CHUNK_S1, 0..CHUNK_S1) {
        // Randomly decide the starting y-coordinate for Ground

        let mut y = fast_noise.get_noise(offsets[1] + dx as f32, offsets[0] + dz as f32);
        y = (y + 1.) / 2.;
        let y = y * MAX_GEN_HEIGHT  as f32 / 3.;

        // Placer les blocs dans le monde
        world.set_yrange(col, (dx, dz), y as i32, 4, Blocks::dirt());
        world.set_yrange(col, (dx, dz), y as i32 - 4, 2, Blocks::stone());
        world.set_yrange(
            col,
            (dx, dz),
            y as i32 - 6,
            MAX_GEN_HEIGHT,
            Blocks::deepslate(),
        );

        // Fill with Ground from ground_start_y to the bottom
    }
}

fn pos_to_range(pos: ColPos) -> [RangeInclusive<i32>; 2] {
    let x = pos.z * CHUNK_S1I;
    let y = pos.x * CHUNK_S1I;
    [x..=(x + CHUNK_S1I - 1), y..=(y + CHUNK_S1I - 1)]
}
