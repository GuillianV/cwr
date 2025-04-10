use bevy::prelude::*;

pub mod player;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((player::EntityPlayerPlugin,));
    }
}
