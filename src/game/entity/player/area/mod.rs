use bevy::prelude::*;

use crate::states::{AppState, LoadingState};

pub mod components;
pub mod resources;
pub mod systems;

pub struct EntityPlayerAreaPlugin;

impl Plugin for EntityPlayerAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LoadingState::LoadingPlayerArea),
            systems::assign_load_area,
        )
        .add_systems(
            Update,
            systems::update_load_area.run_if(in_state(AppState::Game)),
        );
    }
}
