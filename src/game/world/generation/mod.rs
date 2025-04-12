use bevy::prelude::*;

pub mod chunk;
pub mod grid;
pub mod noise;
pub mod systems;
pub mod terrain;

use crate::states::AppState;

use super::generation::{
    chunk::systems::update_chunks,
    grid::systems::update_grid,
    noise::{resources::NoiseMapSettings, systems::init_noise_map},
    systems::init_generation,
    terrain::systems::update_terrain,
};

pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NoiseMapSettings>()
            .add_systems(
                OnEnter(AppState::Loading),
                (init_generation, init_noise_map),
            )
            .add_systems(
                Update,
                (update_chunks, update_grid, update_terrain).run_if(in_state(AppState::Game)),
            );
    }
}
