use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct WorldGridPlugin;

impl Plugin for WorldGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::init_grid)
            .add_systems(Update, systems::update_grid);
    }
}
