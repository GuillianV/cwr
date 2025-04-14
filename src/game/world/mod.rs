use bevy::prelude::*;

pub mod block;
pub mod components;
pub mod events;
pub mod generation;
pub mod realm;
pub mod systems;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(generation::WorldGenerationPlugin)
            .add_systems(Startup, systems::init_world);
    }
}
