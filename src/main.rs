use bevy::{
    prelude::*,
    render::{
        RenderPlugin,
        settings::{RenderCreation, WgpuFeatures, WgpuSettings},
    },
};

mod render;
mod dev;
mod events;
mod game;
mod main_menu;
mod states;
mod systems;
mod util;

use systems::*;

fn main() {
    App::new().add_plugins(AppPlugin).run();
}
