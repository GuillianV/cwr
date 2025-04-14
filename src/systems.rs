use bevy::prelude::*;
use bevy::{
    prelude::*,
    render::{
        RenderPlugin,
        settings::{RenderCreation, WgpuFeatures, WgpuSettings},
    },
};

pub struct AppPlugin;

// use crate::dev::DevPlugin;
use crate::events::EntityMovedEvent;
use crate::game::GamePlugin;
use crate::main_menu::MainMenuPlugin;
use crate::render::ChunkRenderPlugin;
use crate::states::{AppState, LoadingState};

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                // WARN this is a native only feature. It will not work with webgl or webgpu
                features: WgpuFeatures::POLYGON_MODE_LINE,
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .init_state::<LoadingState>()
        .add_event::<EntityMovedEvent>()
        .add_plugins(ChunkRenderPlugin)
        // .add_plugins(DevPlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin);
    }
}
