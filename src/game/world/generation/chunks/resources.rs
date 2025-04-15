use bevy::{prelude::*, utils::HashMap};

use crate::game::world::{block::components::Face, generation::pos::ChunkPos};

#[derive(Resource)]
pub struct ChunkEntities(pub HashMap<(ChunkPos, Face), Entity>);

impl ChunkEntities {
    pub fn new() -> Self {
        ChunkEntities(HashMap::new())
    }
}