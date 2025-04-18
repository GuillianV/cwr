use crate::game::world::generation::noise::resources::NoiseMapSettings;
use bevy::prelude::*;
use bracket_noise::prelude::{FastNoise, FractalType};

use super::resources::ArcNoises;



pub fn init_noise_map(mut commands: Commands, noise_map_settings: Res<NoiseMapSettings>) {
    let base_frequency = 0.005 as f32;
    let base_octaves = 2;
    let base_gain = 0.5;
    let base_lacunarity = 2.0;

    let mut fast_noise = FastNoise::seeded(noise_map_settings.seed.into());
    fast_noise.set_frequency(base_frequency);
    fast_noise.set_fractal_octaves(base_octaves);
    fast_noise.set_fractal_gain(base_gain);
    fast_noise.set_fractal_lacunarity(base_lacunarity);
    
    let mut continental_noise = FastNoise::seeded(noise_map_settings.seed.into());
    continental_noise.set_frequency( 0.01);
    continental_noise.set_fractal_octaves(base_octaves + 1);
    fast_noise.set_fractal_gain(base_gain+1.);
    continental_noise.set_fractal_lacunarity(base_lacunarity.powf(2.));

    let mut erosion_noise = FastNoise::seeded(noise_map_settings.seed.into());
    erosion_noise.set_frequency(0.03);
    erosion_noise.set_fractal_octaves(base_octaves + 2);
    fast_noise.set_fractal_gain(base_gain+2.);
    erosion_noise.set_fractal_lacunarity(base_lacunarity.powf(3.));

    commands.insert_resource(ArcNoises::new(fast_noise, continental_noise, erosion_noise));
}
