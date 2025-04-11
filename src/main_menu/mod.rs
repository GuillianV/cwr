use bevy::prelude::*;

use crate::states::AppState;
pub mod systems;
pub mod styles;
pub mod components;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            systems::layout::spawn_layout.run_if(in_state(AppState::MainMenu)),
        ).add_systems(OnExit(AppState::MainMenu), systems::layout::despawn_layout);
    }
}
