
use bevy::prelude::*;
use bevy_dev_tools::DevToolsPlugin;
pub mod debug;

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {

        #[cfg(debug_assertions)]
        app.add_plugins((
            debug::plugin,
            DevToolsPlugin,
        ));
    }
}

