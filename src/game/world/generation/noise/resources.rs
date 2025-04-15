use std::sync::Arc;

use bevy::prelude::*;
use noise::utils::NoiseMap;

/// Noise map settings
#[derive(Resource)]
pub struct NoiseMapSettings {
    pub seed: u32,
    pub x_size: usize,
    pub y_size: usize,
    pub octaves: usize,
    pub persistence: f64,
    pub lacunarity: f64,
}

impl Default for NoiseMapSettings {
    fn default() -> Self {
        Self {
            seed: 42,
            x_size: 1024,
            y_size: 1024,
            octaves: 6,
            persistence: 0.5,
            lacunarity: 2.0,
        }
    }
}

/// Noise map settings
#[derive(Resource)]


pub struct ArcPerlinNoiseMap {

    pub map: Arc<PerlinNoiseMap>,
}

impl ArcPerlinNoiseMap {
    pub fn new(map: PerlinNoiseMap) -> Self {
        Self { map: Arc::new(map) }
    }
}


pub struct PerlinNoiseMap {
    
    pub map: Vec<f64>,
    pub width: usize,
    pub height: usize,
}

impl PerlinNoiseMap {
    pub fn new(map: Vec<f64>, width: usize, height: usize) -> Self {
        Self { map, width, height }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn get_value(&self, x: usize, y: usize) -> f64 {
        let (width, height) = self.size();

        if x < width && y < height {
            self.map[x + y * width]
        } else {
            0.0
        }
    }
}
