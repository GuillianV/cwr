use bevy::prelude::*;

#[derive(Event)]
pub struct EntityMovedEvent {
    pub entity: String,
    pub position: Vec3,
}
