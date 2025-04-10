use bevy::prelude::*;

pub mod resources;
pub mod systems;
pub mod components;

pub struct WorldNoisePlugin;

impl Plugin for WorldNoisePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::NoiseMapSettings>().add_systems(Startup, systems::init_noise_map);
    }
}
