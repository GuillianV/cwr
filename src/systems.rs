use bevy::core_pipeline::CorePipelinePlugin;
use bevy::core_pipeline::smaa::SmaaPlugin;
use bevy::prelude::*;
use bevy::render::settings::{Backends, PowerPreference, WgpuSettingsPriority};
use bevy::render::{
    RenderPlugin,
    settings::{RenderCreation, WgpuFeatures, WgpuSettings},
};
use bevy::tasks::available_parallelism;
use bevy::window::WindowTheme;

pub struct AppPlugin;

// use crate::dev::DevPlugin;
use crate::game::GamePlugin;
use crate::main_menu::MainMenuPlugin;
use crate::render::ChunkRenderPlugin;
use crate::states::{AppState, LoadingState};

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        features: WgpuFeatures::POLYGON_MODE_LINE,
                        power_preference: PowerPreference::HighPerformance,
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(TaskPoolPlugin {
                    task_pool_options: TaskPoolOptions {
                        min_total_threads: 1,
                        max_total_threads: available_parallelism(), // unlimited threads
                        ..Default::default()
                    },
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: bevy::window::PresentMode::Immediate,
                        // Tells Wasm to resize the window according to the available canvas
                        fit_canvas_to_parent: true,
                        // Tells Wasm not to override default event handling, like F5, Ctrl+R etc.
                        prevent_default_event_handling: false,
                        window_theme: Some(WindowTheme::Dark),
                        enabled_buttons: bevy::window::EnabledButtons {
                            maximize: false,
                            ..Default::default()
                        },
                        visible: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .init_state::<AppState>()
        .init_state::<LoadingState>()
        .add_plugins(ChunkRenderPlugin)
        // .add_plugins(DevPlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin);
    }
}
