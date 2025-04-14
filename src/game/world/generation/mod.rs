use bevy::prelude::*;
use systems::setup_gen_thread;

pub mod chunks;
pub mod constants;
pub mod noise;
pub mod pos;
pub mod systems;

use crate::states::AppState;

use super::generation::noise::{resources::NoiseMapSettings, systems::init_noise_map};

pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NoiseMapSettings>()
        .add_plugins(chunks::WorldGenerationChunksPlugin)
        .add_systems(
            OnEnter(AppState::Loading),
            (init_noise_map, setup_gen_thread),
        );
    }
}
