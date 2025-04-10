use crate::world::components::World;
use bevy::prelude::*;

pub fn init_world(mut commands: Commands) {
    let world_data = World { light: 400.5 };
    let world_data_clone = world_data.clone();

    commands.spawn(world_data);
    commands.insert_resource(AmbientLight {
        color: Color::WHITE.into(),
        brightness: world_data_clone.light,
    });
}

// pub fn event_world_grid(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut world_grid_new_cell_event_reader: EventReader<WorldGridNewCellEvent>,
// ) {

//     for entity_moved_event in world_grid_new_cell_event_reader.read() {

//         let coord = &entity_moved_event.coords;
//         let voxel_data = Voxel {
//             position: Vec3::new(coord.x as f32, 0.0, coord.z as f32),
//             size: Vec3::new(1.0, 1.0, 1.0),
//             color: Color::srgb(0.1, 0.1, 0.1),
//         };

//         let voxel_data_clone = voxel_data.clone();

//         commands.spawn((
//             voxel_data,
//             Mesh3d(meshes.add(Cuboid::from_size(voxel_data_clone.size))),
//             MeshMaterial3d(materials.add(voxel_data_clone.color)),
//             Transform::from_translation(voxel_data_clone.position),
//         ));
//     }
// }
