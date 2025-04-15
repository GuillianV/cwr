use bevy::prelude::*;

use crate::states::{AppState, LoadingState};

pub mod resources;
pub mod systems;

pub struct WorldAreaPlugin;

impl Plugin for WorldAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LoadingState::LoadingSharedLoadArea),
            systems::setup_shared_load_area,
        )
        .add_systems(
            Update,
            systems::update_shared_load_area.run_if(in_state(AppState::Game)),
        );
    }
}
