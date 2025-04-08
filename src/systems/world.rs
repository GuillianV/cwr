use bevy::prelude::*;
use std::f32::consts::PI;

use crate::components::world::{Voxel, World};

pub fn init_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let world_data = World { light: 400.5 };
    let world_data_clone = world_data.clone();

    commands.spawn(world_data);
    commands.insert_resource(AmbientLight {
        color: Color::WHITE.into(),
        brightness: world_data_clone.light,
    });

    let voxel_data = Voxel {
        position: Vec3::new(0.0, 0.0, 0.0),
        size: Vec3::new(1.0, 1.0, 1.0),
        color: Color::srgb(0.1, 0.9, 0.1),
    };
    // Clonez voxel_data avant de l'utiliser
    let voxel_data_clone = voxel_data.clone();

    commands.spawn((
        voxel_data,
        Mesh3d(meshes.add(Cuboid::from_size(voxel_data_clone.size))),
        MeshMaterial3d(materials.add(voxel_data_clone.color)),
        Transform::from_translation(voxel_data_clone.position),
    ));

    //test
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(100.))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-PI / 2.)),
    ));
}
