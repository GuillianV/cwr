use bevy::prelude::*;
use resources::VoxelWorld;

pub mod resources;


pub struct WorldVoxelPlugin;

impl Plugin for WorldVoxelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VoxelWorld::new());
    }
}
