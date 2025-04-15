use bevy::prelude::*;
use systems::setup_gen_thread;

pub mod chunks;
pub mod constants;
pub mod pos;
pub mod systems;
pub mod terrain;

use crate::states::LoadingState;

pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(chunks::WorldGenerationChunksPlugin)
            .add_systems(OnEnter(LoadingState::LoadingMesh), setup_gen_thread);
    }
}
