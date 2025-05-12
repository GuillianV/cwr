use bevy::prelude::*;

use crate::{
    main_menu::{components::PlayButton, styles::*},
    states::{AppState, LoadingState},
};

pub fn on_click(
    click: Trigger<Pointer<Click>>,
    mut button_query: Query<&mut BackgroundColor, With<PlayButton>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut loading_state_next_state: ResMut<NextState<LoadingState>>,
) {
    if let Ok(mut color) = button_query.get_mut(click.target()) {
        *color = PRESSED_BUTTON.into();
        app_state_next_state.set(AppState::Loading);
        loading_state_next_state.set(LoadingState::LoadingPlayerArea);
    }
}

pub fn on_hover(
    hover: Trigger<Pointer<Over>>,
    mut button_query: Query<&mut BackgroundColor, With<PlayButton>>,
) {
    if let Ok(mut color) = button_query.get_mut(hover.target()) {
        *color = HOVERED_BUTTON.into();
    }
}

pub fn on_hover_out(
    hover: Trigger<Pointer<Out>>,
    mut button_query: Query<&mut BackgroundColor, With<PlayButton>>,
) {
    if let Ok(mut color) = button_query.get_mut(hover.target()) {
        *color = NORMAL_BUTTON.into();
    }
}
