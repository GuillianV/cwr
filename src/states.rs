use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Loading,
    Game,
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum LoadingState {
    #[default]
    LoadingGrid,
    LoadingChunk,
    LoadingTerrain,
}
