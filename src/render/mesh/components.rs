use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct LOD(pub f32);

pub fn choose_lod_level(chunk_dist: u32) -> f32 {
    if chunk_dist < 16 {
        return 1.;
    } else if chunk_dist < 32 {
        return 2.;
    } else if chunk_dist < 64 {
        return 6.2;
    }
    return 12.4;
}
