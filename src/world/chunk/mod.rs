use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct WorldChunkPlugin;

impl Plugin for WorldChunkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::init_chunks)
            .add_systems(Update, systems::update_chunks);
    }
}
