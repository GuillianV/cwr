
use bevy::prelude::*;

/// Noise map settings
#[derive(Resource)]
pub struct NoiseMapSettings {
    pub seed: u32,
    pub x_size: usize,
    pub y_size: usize,
}

impl Default for NoiseMapSettings {
    fn default() -> Self {
        Self {
           seed: 42,
           x_size: 1024,
           y_size: 1024,
        }
    }
}

