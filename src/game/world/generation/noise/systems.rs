use bevy::prelude::*;
use noise::utils::PlaneMapBuilder;
use noise::{core::perlin::perlin_2d, permutationtable::PermutationTable};

use crate::game::world::generation::noise::components::WorldNoiseMap;
use crate::game::world::generation::noise::resources::NoiseMapSettings;

pub fn init_noise_map(mut commands: Commands, noise_map_settings: Res<NoiseMapSettings>) {
    let hasher = PermutationTable::new(noise_map_settings.seed);
    let pmb = &PlaneMapBuilder::new_fn(|point| perlin_2d(point.into(), &hasher))
        .set_size(noise_map_settings.x_size, noise_map_settings.y_size)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build();

    let map: Vec<f64> = pmb.iter().map(|x| *x).collect();
    commands.spawn(WorldNoiseMap::new(
        map,
        noise_map_settings.x_size,
        noise_map_settings.y_size,
    ));
}

