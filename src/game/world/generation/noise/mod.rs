use bevy::prelude::*;
use resources::NoiseMapSettings;

pub mod resources;
pub mod systems;

pub struct WorldGenerationNoisePlugin;

impl Plugin for WorldGenerationNoisePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NoiseMapSettings>().add_systems(Startup, systems::init_noise_map);
    }
}
