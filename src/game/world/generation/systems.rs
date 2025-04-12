use bevy::prelude::*;

use crate::{game::world::components::Voxel, states::AppState};

use super::{
    chunk::components::{Chunk, Chunks},
    grid::components::{Coords, Grid},
};

pub fn init_generation(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
        for coord in chunk.cells {
            spawn_voxel(&mut commands, &mut meshes, &mut materials, &coord);
        }
    }

    app_state_next_state.set(AppState::Game);
}

pub fn spawn_voxel(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    coord: &Coords,
) {
    let voxel_data = Voxel {
        position: Vec3::new(coord.x as f32, 0.0, coord.z as f32),
        size: Vec3::new(1.0, 1.0, 1.0),
        color: Color::srgb(0.1, 0.1, 0.1),
    };

    let voxel_data_clone = voxel_data.clone();

    commands.spawn((
        voxel_data,
        Mesh3d(meshes.add(Cuboid::from_size(voxel_data_clone.size))),
        MeshMaterial3d(materials.add(voxel_data_clone.color)),
        Transform::from_translation(voxel_data_clone.position),
    ));
}
