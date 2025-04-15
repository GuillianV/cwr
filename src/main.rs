use bevy::prelude::*;

mod dev;
mod events;
mod game;
mod main_menu;
mod render;
mod states;
mod systems;
mod util;

use systems::*;

fn main() {
    App::new().add_plugins(AppPlugin).run();
}
