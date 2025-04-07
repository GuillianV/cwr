use bevy::prelude::*;
use cwr::AppPlugin;

mod player;

fn main() {
    App::new().add_plugins(AppPlugin).add_systems(Startup, player::init_player).add_systems(Update, player::player_movement).run();
}
