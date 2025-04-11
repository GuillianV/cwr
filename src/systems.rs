use bevy::prelude::*;

pub struct AppPlugin;

// use crate::dev::DevPlugin;
use crate::game::GamePlugin;
use crate::events::EntityMovedEvent;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<EntityMovedEvent>()
        .add_plugins(DefaultPlugins)
            // .add_plugins(DevPlugin)
            .add_plugins(GamePlugin);
    }
}
