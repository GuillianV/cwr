use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::game::entity::player::{
    components::{Player, PlayerCamera},
    resources::MovementSettings,
};
use crate::game::world::realm::resources::Realm;
use crate::util::rotations::{combine_direction_with_rotation_to_eulers, vec2_to_degrees};

use super::area::components::RenderDistance;

pub fn init_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Player {
            position: Vec3::new(0.0, 2.0, 0.0),
            speed: 75.0,
            inertia: 50.,
        },
        Realm::Overworld,
        RenderDistance(64),
        Mesh3d(meshes.add(Capsule3d::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.4, 0.2, 0.3))),
        Transform::default(),
    ));

    commands.spawn((
        PlayerCamera {
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::from_rotation_y(0.0),
            pitch: 0.0,
            yaw: 0.0,
            yaw_euler: Vec2::ZERO,
        },
        Camera3d::default(),
        Transform::default(),
    ));
}

pub fn player_set_camera_movement(
    settings: Res<MovementSettings>,
    primary_window: Query<&Window>,
    mut state: EventReader<MouseMotion>,
    mut q_camera_data: Query<&mut PlayerCamera, With<PlayerCamera>>,
    mut q_player_data: Query<&mut Player, With<Player>>,
) {
    if let Ok(window) = primary_window.get_single() {
        for mut camera_data in q_camera_data.iter_mut() {
            for ev in state.read() {
                let (mut yaw, mut pitch, _) = camera_data.rotation.to_euler(EulerRot::YXZ);
                // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                let window_scale = window.height().min(window.width());
                pitch -= (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                yaw -= (settings.sensitivity * ev.delta.x * window_scale).to_radians();

                pitch = pitch.clamp(-1.54, 1.54);

                camera_data.pitch = pitch;
                camera_data.yaw = yaw;

                // Order is important to prevent unintended roll
                camera_data.rotation =
                    Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }

            let x: f32 = camera_data.yaw.sin() * settings.camera_distance;
            let z = camera_data.yaw.cos() * settings.camera_distance;

            camera_data.yaw_euler = Vec2::new(x, z);

            if let Ok(player) = q_player_data.get_single_mut() {
                let position = Vec3::new(
                    player.position.x + x,
                    settings.camera_height,
                    player.position.z + z,
                );
                camera_data.position = position;
            }
        }
    }
}

pub fn player_set_movement(
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
    mut q_camera_data: Query<&mut PlayerCamera, With<PlayerCamera>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut player)) = player_query.get_single_mut() {
        let old_position: Vec3 = transform.translation;
        let camera_data = q_camera_data.get_single_mut().unwrap();

        let yaw = camera_data.yaw;

        let x = yaw.sin();
        let z = yaw.cos();

        let camera_direction = Vec2::new(x, z) * -1.;

        //Works in desktop but with game pad
        let forward = KeyCode::KeyW;
        let backward = KeyCode::KeyS;
        let left = KeyCode::KeyA;
        let right = KeyCode::KeyD;

        let forward_pressed = keyboard_input.pressed(forward);
        let backward_pressed = keyboard_input.pressed(backward);
        let left_pressed = keyboard_input.pressed(left);
        let right_pressed = keyboard_input.pressed(right);

        let input_direction = if forward_pressed && left_pressed {
            Vec2::new(1., -1.)
        } else if forward_pressed && right_pressed {
            Vec2::new(1., 1.)
        } else if backward_pressed && left_pressed {
            Vec2::new(-1., -1.)
        } else if backward_pressed && right_pressed {
            Vec2::new(-1., 1.)
        } else if forward_pressed {
            Vec2::new(1., 0.)
        } else if backward_pressed {
            Vec2::new(-1., 0.)
        } else if left_pressed {
            Vec2::new(0., -1.)
        } else if right_pressed {
            Vec2::new(0., 1.)
        } else {
            Vec2::ZERO
        };

        let normalized_input_direction = input_direction.normalize_or_zero();
        let deg_input_direction = vec2_to_degrees(normalized_input_direction);

        let combined_direction = if normalized_input_direction != Vec2::ZERO {
            combine_direction_with_rotation_to_eulers(
                camera_direction.x,
                camera_direction.y,
                deg_input_direction,
            )
        } else {
            (0.0, 0.0)
        };

        let new_position: Vec3 = Vec3::new(
            old_position.x + combined_direction.0 * player.speed * time.delta_secs(),
            player.position.y,
            old_position.z + combined_direction.1 * player.speed * time.delta_secs(),
        );


        transform.translation = transform
            .translation
            .lerp(new_position, player.inertia * time.delta_secs());
        player.position = transform.translation;
    }
}

pub fn player_apply_movement(
    mut q_camera: Query<(&mut PlayerCamera, &mut Transform), With<PlayerCamera>>,
    settings: Res<MovementSettings>,
    time: Res<Time>,
) {
    for (camera_data, mut transform) in q_camera.iter_mut() {
        transform.translation = transform.translation.lerp(
            camera_data.position,
            settings.camera_translation_speed * time.delta_secs(),
        );
        transform.rotation = transform.rotation.lerp(
            camera_data.rotation,
            settings.camera_rotation_speed * time.delta_secs(),
        );
    }
}
