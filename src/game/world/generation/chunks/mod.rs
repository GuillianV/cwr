
use bevy::prelude::*;
use resources::ChunkEntities;

pub mod loader;
pub mod components;
pub mod  resources;
pub mod systems;

pub struct WorldGenerationChunksPlugin;

impl Plugin for WorldGenerationChunksPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkEntities::new()).add_plugins(loader::WorldGenerationChunksLoaderPlugin).add_systems(Update, systems::chunk_culling_render_distance);
    }
}
