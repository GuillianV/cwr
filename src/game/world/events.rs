use bevy::prelude::*;

use super::{generation::chunk::components::Chunk, generation::grid::components::Coords};

#[derive(Event)]
pub struct GridCellLoadedEvent {
    pub coords: Coords,
}


#[derive(Event)]
pub struct ChunkFullfilledEvent {
    pub chunk: Chunk,
}
