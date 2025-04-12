
use bevy::prelude::*;

use crate::game::world::{components::Voxel, events::ChunkFullfilledEvent, generation::grid::components::Coords};

pub fn update_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunk_fullfilled_event_reader: EventReader<ChunkFullfilledEvent>,
) {

    for chunk_fullfilled_event in chunk_fullfilled_event_reader.read() {
            for coord in chunk_fullfilled_event.chunk.clone().cells {
                spawn_voxel(&mut commands, &mut meshes, &mut materials, &coord);
            }
      

    }

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
