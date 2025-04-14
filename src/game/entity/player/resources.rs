use bevy::prelude::*;

/// Mouse sensitivity and movement speed
#[derive(Resource)]
pub struct MovementSettings {
    pub sensitivity: f32,
    pub camera_rotation_speed: f32,
    pub camera_translation_speed: f32,
    pub camera_height: f32,
    pub camera_distance: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            camera_rotation_speed: 15.,
            camera_translation_speed: 15.,
            camera_height: 3.,
            camera_distance: 5.,
        }
    }
}
