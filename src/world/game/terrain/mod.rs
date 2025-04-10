use bevy::prelude::*;

pub mod systems;
pub mod components;

pub struct WorldGameTerrainPlugin;

impl Plugin for WorldGameTerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,systems::init_terrain);
      
    }
}
