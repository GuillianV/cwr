use std::sync::Arc;

use bevy::prelude::*;
use bracket_noise::prelude::FastNoise;

/// Noise map settings
#[derive(Resource)]
pub struct NoiseMapSettings {
    pub seed: u32,
}

impl Default for NoiseMapSettings {
    fn default() -> Self {
        Self { seed: 42 }
    }
}

/// Noise map settings
#[derive(Resource)]

pub struct ArcNoises {
    pub noises: Arc<NoisesList>,
}

pub struct NoisesList {
    pub fast_noise: FastNoise,
    pub continental_noise: FastNoise,
    pub erosion_noise: FastNoise,
}

impl ArcNoises {
    pub fn new(
        fast_noise: FastNoise,
        continental_noise: FastNoise,
        erosion_noise: FastNoise,
    ) -> Self {
        Self {
            noises: Arc::new(NoisesList {
                fast_noise,
                continental_noise,
                erosion_noise,
            }),
        }
    }
}
