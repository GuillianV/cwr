use bevy::prelude::*;

mod dev;
mod entity;
mod events;
mod systems;
mod util;
mod states;
mod world;

use systems::*;

fn main() {
    App::new()
        .init_state::<states::AppState>()
        .add_plugins(AppPlugin)
        .run();
}
