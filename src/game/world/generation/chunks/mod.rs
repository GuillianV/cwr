
use bevy::prelude::*;
use resources::ChunkEntities;

pub mod loader;
pub mod components;
pub mod  resources;

pub struct WorldGenerationChunksPlugin;

impl Plugin for WorldGenerationChunksPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkEntities::new()).add_plugins(loader::WorldGenerationChunksLoaderPlugin);
    }
}
