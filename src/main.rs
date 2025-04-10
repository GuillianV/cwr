use bevy::prelude::*;

mod dev;
mod entity;
mod util;
mod world;
mod systems;
mod events;

use systems::*;

fn main() {
    App::new()
        .add_plugins(AppPlugin)
        .run();
}
