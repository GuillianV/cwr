use bevy::prelude::*;

use crate::states::LoadingState;

pub mod components;
pub mod resources;
pub mod systems;

pub struct MeshRenderPlugin;

impl Plugin for MeshRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LoadingState::LoadingMesh),
            (systems::setup_mesh_thread,),
        )
        .add_systems(Update, systems::pull_meshes.run_if(in_state(LoadingState::LoadingRender)));
    }
}
