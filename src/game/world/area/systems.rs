use std::sync::Arc;

use bevy::prelude::*;
use parking_lot::RwLock;

use crate::{game::entity::player::area::resources::PlayerArea, states::{AppState, LoadingState}};

use super::resources::SharedLoadArea;

pub fn setup_shared_load_area(mut commands: Commands, load_area: Res<PlayerArea>,
    mut loading_state_next_state: ResMut<NextState<LoadingState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    commands.insert_resource(SharedLoadArea(Arc::new(RwLock::new(load_area.clone()))));
    loading_state_next_state.set(LoadingState::LoadingMesh);

    println!("Game")
}   

pub fn update_shared_load_area(load_area: Res<PlayerArea>, shared_load_area: Res<SharedLoadArea>) {
    if !load_area.is_changed() {
        return;
    }
    *shared_load_area.0.write() = load_area.clone();
}
