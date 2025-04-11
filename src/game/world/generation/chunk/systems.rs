use bevy::prelude::*;

use super::components::{Chunk, Chunks};
use crate::game::world::events::{ChunkFullfilledEvent, GridCellLoadedEvent};

pub fn init_chunks(mut commands: Commands) {
    let chunks = Chunks::default();
    commands.spawn(chunks);
}

pub fn update_chunks(
    mut q_chunks: Query<&mut Chunks>,
    mut grid_cell_loaded_event_reader: EventReader<GridCellLoadedEvent>,
    mut chunk_fullfilled_event_writer: EventWriter<ChunkFullfilledEvent>,
) {
    let mut chunks = q_chunks.single_mut();
    let mut fullfilled_chunks = Vec::<Chunk>::new();

    for grid_cell_loaded_event in grid_cell_loaded_event_reader.read() {
        let coords = &grid_cell_loaded_event.coords;
        fullfilled_chunks = chunks.update_grid(coords);
    }

    for chunk in fullfilled_chunks {
        chunk_fullfilled_event_writer.send(ChunkFullfilledEvent { chunk });
    }
}
