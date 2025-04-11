use bevy::prelude::*;

use crate::states::AppState;

use super::{
    chunk::components::{Chunk, Chunks},
    grid::components::Grid,
};

pub fn init_generation(
    mut commands: Commands,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    app_state_next_state.set(AppState::Loading);

    let mut world_grid = Grid::default();
    let new_cells = world_grid.initialize(Vec3::new(0.0, 0.0, 0.0));

    commands.spawn(world_grid);

    let mut chunks = Chunks::default();
    let mut fullfilled_chunks = Vec::<Chunk>::new();
    for new_cell in new_cells {
        let local_fullfilled_chunks = chunks.update_grid(&new_cell);
        for chunk in local_fullfilled_chunks {
            fullfilled_chunks.push(chunk);
        }
    }

    commands.spawn(chunks);

    for chunk in fullfilled_chunks {
        println!("Chunk fullfilled {0}", chunk.cells.len());
    }
}
