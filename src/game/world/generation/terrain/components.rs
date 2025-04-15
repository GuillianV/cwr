use bevy::log::info_span;
use bevy::prelude::*;
use itertools::iproduct;
use noise::{NoiseFn, Perlin};
use std::ops::RangeInclusive;

use crate::game::world::{
    block::components::{Block, BlockFamily, Blocks},
    generation::{
        constants::{CHUNK_S1, CHUNK_S1I, MAX_GEN_HEIGHT},
        noise::resources::PerlinNoiseMap,
        pos::ColPos,
    },
    voxel::resources::VoxelWorld,
};

pub fn gen_terrain(world: &VoxelWorld, col: ColPos, perlin_noise_map: &PerlinNoiseMap) {
    //let landratio = self.config.get("land_ratio").copied().unwrap_or(0.4);
    let range = pos_to_range(col);
    let gen_span = info_span!("noise gen", name = "noise gen").entered();

    for (dx, dz) in iproduct!(0..CHUNK_S1, 0..CHUNK_S1) {
        // Randomly decide the starting y-coordinate for Ground

        let mut y = perlin_noise_map.get_value(dx, dz);
        y = y.clamp(0.0, 1.0);
        world.set_yrange(col, (dx, dz), (y as f32 * MAX_GEN_HEIGHT  as f32) as i32 , 1 as usize, Blocks::ground());
        // Fill with Ground from ground_start_y to the bottom
    }
}

fn pos_to_range(pos: ColPos) -> [RangeInclusive<i32>; 2] {
    let x = pos.z * CHUNK_S1I;
    let y = pos.x * CHUNK_S1I;
    [x..=(x + CHUNK_S1I - 1), y..=(y + CHUNK_S1I - 1)]
}

fn generate_multi_perlin_noise(
    width: usize,
    height: usize,
    seed: u32,
    octaves: usize,
    persistence: f64,
    lacunarity: f64,
) -> Vec<f64> {
    // Créez une instance de Perlin avec une graine
    let perlin = Perlin::new(seed);

    // Générer le bruit
    let mut noise_values = Vec::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            let mut value = 0.0;
            let mut frequency = 1.0;
            let mut amplitude = 1.0;
            let mut max_value = 0.0;

            // Somme des octaves
            for _ in 0..octaves {
                value += perlin.get([x as f64 * frequency, y as f64 * frequency]) * amplitude;
                max_value += amplitude;
                amplitude *= persistence;
                frequency *= lacunarity;
            }

            // Normaliser la valeur
            value /= max_value;
            noise_values.push(value);
        }
    }

    noise_values
}
