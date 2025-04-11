use bevy::prelude::*;

pub mod chunk;
pub mod grid;
pub mod noise;
pub mod terrain;

use super::generation::{
    chunk::systems::{init_chunks, update_chunks},
    grid::systems::{init_grid, update_grid},
    noise::{resources::NoiseMapSettings, systems::init_noise_map},
    terrain::systems::update_terrain,
};

pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NoiseMapSettings>()
            .add_systems(Startup, (init_noise_map, init_grid, init_chunks))
            .add_systems(Update, (update_chunks, update_grid, update_terrain));
    }
}
