use bevy::prelude::*;

#[derive(Component)]
pub struct WorldNoiseMap {
    pub map: Vec<f64>,
    pub width: usize,
    pub height: usize,
}

impl WorldNoiseMap {
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
