use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub position: Vec3,
    pub speed: f32,
    pub inertia: f32,
}



#[derive(Component)]
pub struct PlayerCamera {
    pub position: Vec3,
    pub rotation: Quat,
    pub pitch: f32,
    pub yaw: f32,
    pub yaw_euler: Vec2,
}
