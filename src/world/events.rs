
use bevy::prelude::*;

use super::components::Coords;

#[derive(Event)]
pub struct WorldGridNewCellEvent{
    pub coords : Coords
}