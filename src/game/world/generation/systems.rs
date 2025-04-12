use std::collections::HashSet;
use binary_greedy_meshing as bgm;
use std::collections::BTreeSet;

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{
        mesh::{Indices, MeshVertexAttribute, PrimitiveTopology, VertexAttributeValues},
        primitives::Aabb,
        render_resource::VertexFormat,
    },
};

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

    let mut voxels = [0; bgm::CS_P3];
    // Add 2 voxels at position 0;0;0 and 0;1;0
    voxels[bgm::pad_linearize(0, 0, 0)] = 1;
    voxels[bgm::pad_linearize(0, 1, 0)] = 1;
    // Contain useful buffers that can be cached and cleared 
    // with mesh_data.clear() to avoid re-allocation
    let mut mesh_data = bgm::MeshData::new();
    // Does the meshing, mesh_data.quads is the output
    // transparent block values are signaled by putting them in the BTreeSet
    bgm::mesh(&voxels, &mut mesh_data, BTreeSet::default());

    println!("{:?}", mesh_data.quads);

    // //Test multimesh
    // let mesh = meshes.add();
    // let material = materials.add(StandardMaterial {
    //     base_color: Color::rgb(0.8, 0.7, 0.6),
    //     ..Default::default()
    // });

    // spawn_voxel(
    //     &mut commands,
    //     mesh.clone(),
    //     material.clone(),
    //     &Coords::new(0, 0),
    // );

    app_state_next_state.set(AppState::Game);
}



pub fn spawn_voxel(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    coord: &Coords,
) {
    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(Vec3::new(coord.x as f32, 0.0, coord.z as f32)),
    ));
}
