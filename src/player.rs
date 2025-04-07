use bevy::{input::keyboard, prelude::*};

#[derive(Component)]
pub struct Player {
    position: Vec3,
    speed: f32,
}

pub fn init_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Player {
            position: Vec3::new(0.0, 0.0, 0.0),
            speed: 10.0,
        },
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn player_movement(
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut player)) = player_query.get_single_mut() {
        let old_position: Vec3 = transform.translation;

        let mut direction = Vec2::ZERO;

        direction.y = if keyboard_input.pressed(KeyCode::KeyW) {
            1.0
        } else if keyboard_input.pressed(KeyCode::KeyS) {
            -1.0
        } else {
            0.0
        };

        direction.x = if keyboard_input.pressed(KeyCode::KeyA) {
            1.0
        } else if keyboard_input.pressed(KeyCode::KeyD) {
            -1.0
        } else {
            0.0
        };

        let new_position: Vec3 = Vec3::new(
            old_position.x + direction.x * player.speed * time.delta_secs(),
            0.0,
            old_position.z + direction.y * player.speed * time.delta_secs(),
        );
        transform.translation = new_position;
        player.position = new_position;
    }
}
