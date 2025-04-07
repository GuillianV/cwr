use bevy::prelude::*;
use bevy_dev_tools::DevToolsPlugin;

mod debug;
pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins,
        ));

        #[cfg(debug_assertions)]
        app.add_plugins((
            debug::plugin,
            DevToolsPlugin,
        ));
    }
}

