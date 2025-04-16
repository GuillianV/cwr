use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use bevy::tasks::futures_lite::future::yield_now;
use std::sync::Arc;

use crate::game::world::generation::terrain::components::gen_terrain;
use crate::game::world::voxel::resources::VoxelWorld;

use super::chunks::loader::resources::LoadOrders;
use super::noise::resources::ArcFastNoise;

pub fn setup_gen_thread(
    blocks: Res<VoxelWorld>,
    perlin_noise_map: Res<ArcFastNoise>,
    /* world_rng: Res<WorldRng>, */ load_orders: Res<LoadOrders>,
) {
    let thread_pool = AsyncComputeTaskPool::get();
    let chunks = Arc::clone(&blocks.chunks);
    // let seed_value = world_rng.seed;
    let load_orders = Arc::clone(&load_orders.to_generate);
    let perlin_noise_map = Arc::clone(&perlin_noise_map.fast_noise);

    thread_pool
        .spawn(async move {
            let world = VoxelWorld::new_with(chunks);
            loop {
                let Some((col_pos, _)) = load_orders.try_write_arc().and_then(|mut ld| ld.pop())
                else {
                    yield_now();
                    continue;
                };

                gen_terrain(&world, col_pos, &perlin_noise_map);
                world.mark_change_col(col_pos);
            }
        })
        .detach();
}
