
use bevy::prelude::*;

use crate::game::world::events::ChunkFullfilledEvent;

pub fn update_terrain(
    mut chunk_fullfilled_event_reader: EventReader<ChunkFullfilledEvent>,
) {

    for chunk_fullfilled_event in chunk_fullfilled_event_reader.read() {
        println!("Chunk fullfilled {0}", chunk_fullfilled_event.chunk.cells.len());
    }

}