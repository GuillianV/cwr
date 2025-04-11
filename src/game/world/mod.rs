use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod generation;
pub mod systems;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::GridCellLoadedEvent>()
            .add_event::<events::ChunkFullfilledEvent>()
            .add_plugins(generation::WorldGenerationPlugin)
            .add_systems(Startup, systems::init_world);
    }
}
