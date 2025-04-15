use bevy::prelude::*;

use crate::states::{AppState, LoadingState};

pub mod area;
pub mod components;
pub mod resources;
pub mod systems;

pub struct EntityPlayerPlugin;

impl Plugin for EntityPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::MovementSettings>()
            .add_plugins(area::EntityPlayerAreaPlugin)
            .add_systems(Startup, systems::init_player)
            .add_systems(
                Update,
                (
                    systems::player_set_movement,
                    systems::player_set_camera_movement,
                    systems::player_apply_movement,
                )
                    .run_if(in_state(AppState::Game)),
            );
    }
}
