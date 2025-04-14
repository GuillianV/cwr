use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod resources;
pub mod area;

pub struct EntityPlayerPlugin;

impl Plugin for EntityPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::MovementSettings>()
            .add_plugins(area::EntityPlayerAreaPlugin)
            .add_systems(Startup, systems::init_player)
            .add_systems(Update, systems::player_set_camera_movement)
            .add_systems(Update, systems::player_set_movement)
            .add_systems(Update, systems::player_apply_movement);
    }
}
