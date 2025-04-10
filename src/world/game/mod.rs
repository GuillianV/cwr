use bevy::prelude::*;
pub mod terrain;

pub struct WorldGamePlugin;

impl Plugin for WorldGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(terrain::WorldGameTerrainPlugin);
    }
}
