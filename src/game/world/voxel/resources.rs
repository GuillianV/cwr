use std::sync::Arc;

use bevy::prelude::*;
use dashmap::DashMap;

use crate::game::world::{
    block::components::Block,
    generation::{
        chunks::components::TrackedChunk,
        constants::CHUNK_S1,
        pos::{ChunkPos, ColPos, ColedPos, chunked, pos2d::chunks_in_col},
    },
};

#[derive(Resource)]
pub struct VoxelWorld {
    pub chunks: Arc<DashMap<ChunkPos, TrackedChunk>>,
}

impl VoxelWorld {
    pub fn new() -> Self {
        VoxelWorld {
            chunks: Arc::new(DashMap::new()),
        }
    }

    pub fn new_with(chunks: Arc<DashMap<ChunkPos, TrackedChunk>>) -> Self {
        VoxelWorld { chunks }
    }

    pub fn set_yrange(
        &self,
        col_pos: ColPos,
        (x, z): ColedPos,
        top: i32,
        mut height: usize,
        block: Block,
    ) {
        // USED BY TERRAIN GENERATION - bypasses change detection for efficiency
        let (mut cy, mut dy) = chunked(top);
        while height > 0 && cy >= 0 {
            let chunk_pos = ChunkPos {
                x: col_pos.x,
                y: cy,
                z: col_pos.z,
                realm: col_pos.realm,
            };
            let h = height.min(dy);
            self.chunks
                .entry(chunk_pos)
                .or_insert_with(|| TrackedChunk::new())
                .set_yrange((x, dy, z), h, block);
            height -= h;
            cy -= 1;
            dy = CHUNK_S1 - 1;
        }
    }

    pub fn mark_change_col(&self, col_pos: ColPos) {
        // USE BY TERRAIN GEN to mass mark change on chunks for efficiency
        for chunk_pos in chunks_in_col(&col_pos) {
            self.mark_change_single(chunk_pos);
        }
    }

    pub fn mark_change_single(&self, chunk_pos: ChunkPos) {
        if let Some(mut chunk) = self.chunks.get_mut(&chunk_pos) {
            chunk.changed = true;
        }
    }
}
