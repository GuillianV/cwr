use crate::game::world::components::World;
use bevy::prelude::*;

pub fn init_world(mut commands: Commands) {
    let world_data = World { light: 400.5 };
    let world_data_clone = world_data.clone();

    commands.spawn(world_data);
    commands.insert_resource(AmbientLight {
        color: Color::WHITE.into(),
        brightness: world_data_clone.light,
    });
}
