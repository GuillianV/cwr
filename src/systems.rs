use bevy::prelude::*;

pub struct AppPlugin;

// use crate::dev::DevPlugin;
use crate::events::EntityMovedEvent;
use crate::game::GamePlugin;
use crate::main_menu::MainMenuPlugin;
use crate::states::AppState;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .init_state::<AppState>()
            .add_event::<EntityMovedEvent>()
            // .add_plugins(DevPlugin)
            .add_plugins(MainMenuPlugin)
            .add_plugins(GamePlugin);
    }
}
