use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod noise;
pub mod systems;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::WorldGridNewCellEvent>()
            .add_plugins(noise::WorldNoisePlugin)
            .add_systems(Startup, systems::init_world)
            .add_systems(Update, systems::update_world)
            .add_systems(Update, systems::event_world_grid);
    }
}
