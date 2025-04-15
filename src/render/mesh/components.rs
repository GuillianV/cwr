use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct LOD(pub usize);

pub fn choose_lod_level(chunk_dist: u32) -> usize {
    if chunk_dist < 16 {
        return 1;
    }
    return 2;
}