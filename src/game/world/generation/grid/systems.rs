use bevy::prelude::*;

use super::components::{Coords, Grid};
use crate::{events::EntityMovedEvent, game::world::events::GridCellLoadedEvent};

pub fn init_grid(
    mut commands: Commands,
    mut world_grid_new_cell_event_writer: EventWriter<GridCellLoadedEvent>,
) {
    let mut world_grid = Grid::default();
    let new_cells = world_grid.initialize(Vec3::new(0.0, 0.0, 0.0));

    commands.spawn(world_grid);

    for new_cell in new_cells {
        
        world_grid_new_cell_event_writer.send(GridCellLoadedEvent { coords: new_cell });
    }
}

pub fn update_grid(
    mut q_world_grid: Query<&mut Grid>,
    mut entity_moved_event_reader: EventReader<EntityMovedEvent>,
    mut world_grid_new_cell_event_writer: EventWriter<GridCellLoadedEvent>,
) {
    let mut world_grid = q_world_grid.single_mut();

    let mut new_cells = Vec::<Coords>::new();
    for entity_moved_event in entity_moved_event_reader.read() {
        if entity_moved_event.entity == "player" {
            new_cells = world_grid.update(entity_moved_event.position);
        }
    }

    for new_cell in new_cells {
        world_grid_new_cell_event_writer.send(GridCellLoadedEvent { coords: new_cell });
    }
}
