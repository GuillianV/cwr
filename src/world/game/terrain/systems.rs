
use bevy::prelude::*;

use crate::world::events::ChunkFullfilledEvent;

pub fn init_terrain(
    mut chunk_fullfilled_event_reader: EventReader<ChunkFullfilledEvent>,
) {

    for chunk_fullfilled_event in chunk_fullfilled_event_reader.read() {
        println!("Chunk fullfilled {0}", chunk_fullfilled_event.chunk.cells.len());
    }

}