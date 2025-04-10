use bevy::prelude::*;
#[derive(Component, Clone)]
pub struct World {
    pub light: f32,
}

#[derive(Component, Clone)]
pub struct Voxel {
    pub position: Vec3,
    pub size: Vec3,
    pub color: Color,
}
