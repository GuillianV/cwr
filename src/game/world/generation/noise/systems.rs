use bevy::prelude::*;
use bracket_noise::prelude::FastNoise;
use crate::game::world::generation::noise::resources::NoiseMapSettings;

use super::resources::ArcFastNoise;

pub fn init_noise_map(mut commands: Commands, noise_map_settings: Res<NoiseMapSettings>) {

    let mut fast_noise = FastNoise::seeded(noise_map_settings.seed.into());
    fast_noise.set_frequency(0.005);

    commands.insert_resource(ArcFastNoise::new(
        fast_noise,
    ));
}
