
use std::sync::Arc;

use bevy::prelude::*;
use parking_lot::RwLock;

use crate::game::entity::player::area::resources::PlayerArea;


#[derive(Resource)]
pub struct SharedLoadArea(pub Arc<RwLock<PlayerArea>>);
