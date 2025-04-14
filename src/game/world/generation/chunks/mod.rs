
use bevy::prelude::*;

pub mod loader;
pub mod components;

pub struct WorldGenerationChunksPlugin;

impl Plugin for WorldGenerationChunksPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(loader::WorldGenerationChunksLoaderPlugin);
    }
}
