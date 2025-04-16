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
        Self {
            seed: 42,
        }
    }
}

/// Noise map settings
#[derive(Resource)]


pub struct ArcFastNoise {

    pub fast_noise: Arc<FastNoise>,
}

impl ArcFastNoise {
    pub fn new(fast_noise: FastNoise) -> Self {
        Self { fast_noise: Arc::new(fast_noise) }
    }
}

