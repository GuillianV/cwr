use bevy::prelude::*;

pub struct AppPlugin;

// use crate::dev::DevPlugin;
use crate::entity::EntityPlugin;
use crate::world::WorldPlugin;
use crate::events::EntityMovedEvent;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<EntityMovedEvent>()
        .add_plugins(DefaultPlugins)
            // .add_plugins(DevPlugin)
            .add_plugins(WorldPlugin)
            .add_plugins(EntityPlugin);
    }
}
