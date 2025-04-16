use bevy::{
    ecs::{
        query::{Changed, With},
        system::Query,
    },
    math::Vec3A,
    render::{
        camera::Camera,
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

pub fn chunk_culling(
    view_query: Query<(&Frustum, &Camera, &GlobalTransform), Changed<Frustum>>,
    q_render: Query<&RenderDistance, With<Player>>,
    mut chunk_query: Query<(&mut Visibility, &Transform, &Face, &Aabb), With<NoFrustumCulling>>,
) {
    for (frustum, camera, gtransform) in view_query.iter() {
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
            let (mut visibility, coord, face, aabb) = item;
            let center = Vec3A::from(coord.translation) + aabb.center;
            let world_sphere = Sphere {
                center,
                radius: aabb.half_extents.length(),
            };
            let world_aabb = Aabb {
                center,
                half_extents: aabb.half_extents,
            };
            total.fetch_add(1, Ordering::AcqRel);

            let distance_vec = (&chunk_cam_pos - chunk_pos(coord.translation));
            let distance = distance_vec.x.abs() + distance_vec.z.abs();

            *visibility = if !face_visible(&chunk_cam_pos, chunk_pos(coord.translation), face)
                || !frustum.intersects_sphere(&world_sphere, false)
                || !intersects_aabb(frustum, &world_aabb)
                || distance > render_dist.0.into()
            {
                Visibility::Hidden
            } else {
                visible.fetch_add(1, Ordering::AcqRel);
                Visibility::Visible
            };
        });
    }
}
