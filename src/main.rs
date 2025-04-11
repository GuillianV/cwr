use bevy::prelude::*;

mod dev;
mod events;
mod systems;
mod util;
mod states;
mod game;

use systems::*;

fn main() {
    App::new()
        .init_state::<states::AppState>()
        .add_plugins(AppPlugin)
        .run();
}
