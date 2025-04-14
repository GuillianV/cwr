use bevy::prelude::*;
use resources::LoadOrders;

pub mod resources;

pub struct WorldGenerationChunksLoaderPlugin;

impl Plugin for WorldGenerationChunksLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadOrders::new());
    }
}
