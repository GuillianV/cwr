use bevy::prelude::*;

use super::components::Chunks;
use crate::game::world::events::{ChunkFullfilledEvent, GridCellLoadedEvent};

pub fn update_chunks(
    mut q_chunks: Query<&mut Chunks>,
    mut grid_cell_loaded_event_reader: EventReader<GridCellLoadedEvent>,
    mut chunk_fullfilled_event_writer: EventWriter<ChunkFullfilledEvent>,
) {
    let mut chunks = q_chunks.single_mut();

    for grid_cell_loaded_event in grid_cell_loaded_event_reader.read() {
        let coords = &grid_cell_loaded_event.coords;

        for chunk in chunks.update_grid(coords) {
            println!("Chunk fullfilledaa {0}", chunk.cells.len());
            chunk_fullfilled_event_writer.send(ChunkFullfilledEvent { chunk });
        }
    }
}
