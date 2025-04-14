use bevy::prelude::*;
use crossbeam::channel::Receiver;

use crate::game::world::{block::components::Face, generation::pos::ChunkPos};

use super::components::LOD;

#[derive(Resource)]
pub struct MeshReciever(pub Receiver<(Option<Mesh>, ChunkPos, Face, LOD)>);
