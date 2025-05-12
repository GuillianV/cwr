use bevy::prelude::*;

use crate::{game::world::{generation::{chunks::loader::resources::LoadOrders, pos::ColPos}, realm::resources::Realm}, states::LoadingState};

use super::{components::RenderDistance, resources::PlayerArea};


pub fn assign_load_area(
    mut commands: Commands,
    mut col_orders: ResMut<LoadOrders>,
    mut query: Query<(Entity, &Transform, &Realm, &RenderDistance)>,
    mut loading_state_next_state: ResMut<NextState<LoadingState>>,
) {
    let Ok((player, transform, realm, render_dist)) = query.single_mut() else { return; };
    let col = ColPos::from((transform.translation, *realm));
    let old_load_area = PlayerArea::empty();
    let new_load_area = PlayerArea::new(col, *render_dist);
    col_orders.on_load_area_change(player.index(), &old_load_area, &new_load_area);
    commands.insert_resource(new_load_area.clone());
   
    loading_state_next_state.set(LoadingState::LoadingSharedLoadArea);
}


pub fn update_load_area(
    mut query: Query<(Entity, &Transform, &Realm, &RenderDistance)>,
    mut col_orders: ResMut<LoadOrders>,
    mut load_area: ResMut<PlayerArea>,
) {
    for (player, transform, realm, render_dist) in query.iter_mut() {
        let col = ColPos::from((transform.translation, *realm));
        // we're checking before modifying to avoid triggering unnecessary Change detection
        if col != load_area.center {
            let new_load_area = PlayerArea::new(col, *render_dist);
            col_orders.on_load_area_change(player.index(), &load_area, &new_load_area);
            *load_area = new_load_area;
        }
    }
}