use bevy::prelude::*;

pub mod chunk;
pub mod components;
pub mod events;
pub mod game;
pub mod grid;
pub mod noise;
pub mod systems;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::GridCellLoadedEvent>()
            .add_event::<events::ChunkFullfilledEvent>()
            .add_plugins(game::WorldGamePlugin)
            .add_plugins(grid::WorldGridPlugin)
            .add_plugins(chunk::WorldChunkPlugin)
            .add_plugins(noise::WorldNoisePlugin)
            .add_systems(Startup, systems::init_world);
    }
}
