use bevy::prelude::*;

use super::{chunk::components::Chunk, grid::components::Coords};

#[derive(Event)]
pub struct GridCellLoadedEvent {
    pub coords: Coords,
}


#[derive(Event)]
pub struct ChunkFullfilledEvent {
    pub chunk: Chunk,
}
