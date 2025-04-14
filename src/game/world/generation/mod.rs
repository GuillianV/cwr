use bevy::prelude::*;
use systems::setup_gen_thread;

pub mod chunks;
pub mod constants;
pub mod noise;
pub mod pos;
pub mod systems;
pub mod terrain;

use crate::states::{AppState, LoadingState};

use super::generation::noise::{resources::NoiseMapSettings, systems::init_noise_map};

pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NoiseMapSettings>()
        .add_plugins(chunks::WorldGenerationChunksPlugin)
        .add_systems(
            OnEnter(LoadingState::LoadingMesh),
            (init_noise_map, setup_gen_thread),
        );
    }
}
