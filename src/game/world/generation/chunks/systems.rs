use bevy::{
    ecs::{
        query::{Changed, With},
        system::Query,
    },
    math::Vec3A,
    render::{
        camera::Camera,
        experimental::occlusion_culling::OcclusionCulling,
        primitives::{Aabb, Frustum, Sphere},
        view::{NoFrustumCulling, Visibility},
    },
    transform::components::{GlobalTransform, Transform},
};
use std::sync::atomic::{AtomicU32, Ordering};

use crate::game::{
    entity::player::{area::components::RenderDistance, components::Player},
    world::{block::components::Face, generation::pos::chunk_pos},
};

use super::components::{face_visible, intersects_aabb};

pub fn chunk_culling_render_distance(
    view_query: Query<(&Camera, &GlobalTransform), Changed<Frustum>>,
    q_render: Query<&RenderDistance, With<Player>>,
    mut chunk_query: Query<(&mut Visibility, &Transform), With<OcclusionCulling>>,
) {
    for (camera, gtransform) in view_query.iter() {
        if !camera.is_active {
            continue;
        }

        let chunk_cam_pos = chunk_pos(gtransform.translation());

        let total = AtomicU32::new(0);
        let visible = AtomicU32::new(0);

        let Ok(render_dist) = q_render.get_single() else {
            return;
        };

        // TODO: make it less dumb when having multiple cameras
        chunk_query.par_iter_mut().for_each(|item| {
            let (mut visibility, coord) = item;

            total.fetch_add(1, Ordering::AcqRel);

            let distance_vec = (&chunk_cam_pos - chunk_pos(coord.translation));
            let distance = distance_vec.x.abs() + distance_vec.z.abs();

            *visibility = if distance > (render_dist.0 as f32  * 1.44) as i64  {
                Visibility::Hidden
            } else {
                visible.fetch_add(1, Ordering::AcqRel);
                Visibility::Visible
            };
        });
    }
}
