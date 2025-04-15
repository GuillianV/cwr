use std::sync::Arc;

use bevy::{
    prelude::*,
    render::{primitives::Aabb, view::NoFrustumCulling},
    tasks::{AsyncComputeTaskPool, futures_lite::future::yield_now},
};
use crossbeam::channel::{Receiver, unbounded};
use itertools::Itertools;

use crate::{
    game::{
        entity::player::area::resources::PlayerArea,
        world::{
            area::resources::SharedLoadArea,
            block::components::Face,
            generation::{chunks::resources::ChunkEntities, constants::CHUNK_S1, pos::ChunkPos},
            voxel::resources::VoxelWorld,
        },
    },
    render::resources::BlockTextureArray, states::{AppState, LoadingState},
};

use super::{components::LOD, resources::MeshReciever};

pub fn setup_mesh_thread(
    mut commands: Commands,
    blocks: Res<VoxelWorld>,
    shared_load_area: Res<SharedLoadArea>,
    mut loading_state_next_state: ResMut<NextState<LoadingState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    let thread_pool = AsyncComputeTaskPool::get();
    let chunks = Arc::clone(&blocks.chunks);
    let (mesh_sender, mesh_reciever) = unbounded();
    commands.insert_resource(MeshReciever(mesh_reciever));
    let shared_load_area = Arc::clone(&shared_load_area.0);
    thread_pool
        .spawn(async move {
            loop {
                let Some((chunk_pos, dist)) = shared_load_area.read().pop_closest_change(&chunks)
                else {
                    yield_now();
                    continue;
                };

                let lod = 1;
                let Some(chunk) = chunks.get(&chunk_pos) else {
                    continue;
                };

           
                let face_meshes = chunk.create_face_meshes(lod);
                for (i, face_mesh) in face_meshes.into_iter().enumerate() {
                    let face: Face = i.into();
                    if mesh_sender
                        .send((face_mesh, chunk_pos, face, LOD(lod)))
                        .is_err()
                    {
                        println!("mesh for {:?} couldn't be sent", chunk_pos)
                    };
                }
            }
        })
        .detach();

    loading_state_next_state.set(LoadingState::LoadingRender);
    app_state_next_state.set(AppState::Game);

}

pub fn pull_meshes(
    mut commands: Commands,
    mesh_reciever: Res<MeshReciever>,
    mut mesh_query: Query<(&mut Mesh3d, &mut LOD)>,
    mut chunk_ents: ResMut<ChunkEntities>,
    mut meshes: ResMut<Assets<Mesh>>,
    block_tex_array: Res<BlockTextureArray>,
    load_area: Res<PlayerArea>,
    blocks: Res<VoxelWorld>,
) {
    let received_meshes: Vec<_> = mesh_reciever
        .0
        .try_iter()
        .filter(|(_, chunk_pos, _, _)| load_area.col_dists.contains_key(&(*chunk_pos).into()))
        .collect();
    for (mesh_opt, chunk_pos, face, lod) in received_meshes
        .into_iter()
        .rev()
        .unique_by(|(_, pos, face, _)| (*pos, *face))
    {
        let Some(mesh) = mesh_opt else {
            if let Some(ent) = chunk_ents.0.remove(&(chunk_pos, face)) {
                commands.entity(ent).despawn();
            }
            continue;
        };
        let chunk_aabb = Aabb::from_min_max(Vec3::ZERO, Vec3::splat(CHUNK_S1 as f32));
        if let Some(ent) = chunk_ents.0.get(&(chunk_pos, face)) {
            if let Ok((mut handle, mut old_lod)) = mesh_query.get_mut(*ent) {
                handle.0 = meshes.add(mesh);
                *old_lod = lod;
            } else {
                // the entity is not instanciated yet, we put it back
                println!("entity wasn't ready to recieve updated mesh");
            }
        } else if blocks.chunks.contains_key(&chunk_pos) {

            let ent = commands
                .spawn((
                    Mesh3d(meshes.add(mesh)),
                    MeshMaterial3d(block_tex_array.0.clone_weak()),
                    Transform::from_translation(
                        Vec3::new(chunk_pos.x as f32, chunk_pos.y as f32, chunk_pos.z as f32)
                            * CHUNK_S1 as f32,
                    ),
                    NoFrustumCulling,
                    chunk_aabb,
                    lod,
                    face,
                ))
                .id();
            chunk_ents.0.insert((chunk_pos, face), ent);
        }
    }
}
