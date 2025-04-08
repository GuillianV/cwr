use bevy::prelude::*;

mod components;
mod plugins;
mod resouces;
mod systems;
mod utils;

use crate::plugins::plugins::AppPlugin;
use crate::resouces::movement_settings::MovementSettings;
use crate::systems::{
    player::{init_player, player_set_camera_movement, player_set_movement, player_apply_movement},
    world::init_world
};

fn main() {
    App::new().init_resource::<MovementSettings>()
    .add_plugins(AppPlugin)
    .add_systems(Startup,init_world)
    .add_systems(Startup,init_player)
    .add_systems(Update,player_set_camera_movement)
    .add_systems(Update,player_set_movement)
    .add_systems(Update,player_apply_movement)
    
    .run();
}
