use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod resources;

pub struct EntityPlayerAreaPlugin;

impl Plugin for EntityPlayerAreaPlugin {
    fn build(&self, app: &mut App) {
       app.add_systems(PostStartup, systems::assign_load_area).add_systems(Update, systems::update_load_area);
    }
}
