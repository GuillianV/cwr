// use binary_greedy_meshing as bgm;
// use std::collections::BTreeSet;
// use std::collections::HashSet;
// use std::sync::Arc;

// use bevy::{
//     asset::RenderAssetUsages,
//     prelude::*,
//     render::{
//         mesh::{Indices, MeshVertexAttribute, PrimitiveTopology, VertexAttributeValues},
//         primitives::Aabb,
//         render_resource::VertexFormat,
//     },
// };

// use crate::game::world::block::components::Face;
// use crate::render::constants::ATTRIBUTE_VOXEL_DATA;
// use crate::render::resources::BlockTextureArray;
// use crate::{game::world::components::Voxel, states::AppState};

// use super::chunks::loader::resources::LoadOrders;

// pub fn init_generation(
//     mut commands: Commands,
//     mut meshess: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut app_state_next_state: ResMut<NextState<AppState>>,
//     block_tex_array: Res<BlockTextureArray>,
//     load_orders: Res<LoadOrders>,
// ) {
//     app_state_next_state.set(AppState::Loading);

//     let load_orders = Arc::clone(&load_orders.to_generate);
//     loop {
//         let Some((col_pos, _)) = load_orders.try_write_arc().and_then(|mut ld| ld.pop()) else {
//             continue;
//         };

//         println!("loading col_pos: {:?}", col_pos);
//     }


//     let mut voxels = [0; bgm::CS_P3];
//     // Add 2 voxels at position 0;0;0 and 0;1;0

//     voxels[bgm::pad_linearize(61, 0, 0)] = 1;
//     voxels[bgm::pad_linearize(0, 1, 0)] = 1;
//     voxels[bgm::pad_linearize(1, 1, 0)] = 3;
//     // Contain useful buffers that can be cached and cleared
//     // with mesh_data.clear() to avoid re-allocation
//     let mut mesh_data = bgm::MeshData::new();
//     // Does the meshing, mesh_data.quads is the output
//     // transparent block values are signaled by putting them in the BTreeSet
//     bgm::mesh(&voxels, &mut mesh_data, BTreeSet::default());

//     println!("{:?}", mesh_data.quads);

//     let mut meshes: [Option<bevy::prelude::Mesh>; 6] = core::array::from_fn(|_| None);

//     for (face_n, quads) in mesh_data.quads.iter().enumerate() {
//         let mut voxel_data: Vec<[u32; 2]> = Vec::with_capacity(quads.len() * 4);
//         let indices = bgm::indices(quads.len());
//         let face: Face = face_n.into();
//         for quad in quads {
//             let voxel_i = (quad >> 32) as usize;
//             let w = MASK_6 & (quad >> 18);
//             let h = MASK_6 & (quad >> 24);
//             let xyz = MASK_XYZ & quad;
//             // let block = self.palette[voxel_i];
//             let layer = 0; //texture_map.get_texture_index(block, face) as u32;
//             let color = match voxel_i {
//                 1 => 0b011_111_001,
//                 2 => 0b110_011_001,
//                 3 => 0b010_101_001,
//                 _ => 0b111_111_111,
//             };
//             let vertices = face.vertices_packed(xyz as u32, w as u32, h as u32, 1 as u32);
//             let quad_info = (layer << 12) | (color << 3) | face_n as u32;
//             voxel_data.extend_from_slice(&[
//                 [vertices[0], quad_info],
//                 [vertices[1], quad_info],
//                 [vertices[2], quad_info],
//                 [vertices[3], quad_info],
//             ]);
//         }
//         meshes[face_n] = Some(
//             Mesh::new(
//                 PrimitiveTopology::TriangleList,
//                 RenderAssetUsages::RENDER_WORLD,
//             )
//             .with_inserted_attribute(ATTRIBUTE_VOXEL_DATA, voxel_data)
//             .with_inserted_indices(Indices::U32(indices)),
//         )
//     }

//     for (i, mesh) in meshes.iter().enumerate() {
//         if mesh.is_some() {
//             let mesh = mesh.as_ref().unwrap();

//             let _ = commands
//                 .spawn((
//                     Mesh3d(meshess.add(mesh.clone())),
//                     MeshMaterial3d(block_tex_array.0.clone_weak()),
//                     Transform::from_translation(Vec3::ZERO),
//                 ))
//                 .id();
//         }
//     }

//     app_state_next_state.set(AppState::Game);
// }

use std::sync::Arc;
use bevy::prelude::*;
use bevy::tasks::futures_lite::future::yield_now;
use bevy::tasks::AsyncComputeTaskPool;

use super::chunks::loader::resources::LoadOrders;

pub fn setup_gen_thread(/*blocks: Res<VoxelWorld>, world_rng: Res<WorldRng>, */load_orders: Res<LoadOrders>) {
    let thread_pool = AsyncComputeTaskPool::get();
    // let chunks = Arc::clone(&blocks.chunks);
    // let seed_value = world_rng.seed;
    let load_orders = Arc::clone(&load_orders.to_generate);
    thread_pool.spawn(
        async move {
            // let gen = Earth::new(seed_value as u32, HashMap::new());
            //let world = VoxelWorld::new_with(chunks);
            loop {
                let Some((col_pos, _)) = load_orders.try_write_arc().and_then(|mut ld| ld.pop()) else {
                    yield_now();
                    continue;
                };

                println!("loading col_pos: {:?}", col_pos);
                // gen.gen(&world, col_pos);
                // world.mark_change_col(col_pos);
            }
        }
    ).detach();
}