use bevy::prelude::*;

pub mod entity;
pub mod world;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(entity::EntityPlugin).add_plugins(world::WorldPlugin);
    }
}
