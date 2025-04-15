

use std::sync::atomic::{AtomicU32, Ordering};
use bevy::{
    ecs::{query::{Changed, With}, system::Query}, 
    math::{I64Vec3, Vec3A}, render::{camera::Camera, primitives::{Aabb, Frustum, Sphere}, 
    view::{NoFrustumCulling, Visibility}}, 
    transform::components::{GlobalTransform, Transform}
};

use crate::game::world::{block::components::Face, generation::pos::chunk_pos};

use super::components::{face_visible, intersects_aabb};

pub fn chunk_culling(
    view_query: Query<(&Frustum, &Camera, &GlobalTransform), Changed<Frustum>>,
    mut chunk_query: Query<
        (&mut Visibility, &Transform, &Face, &Aabb),
        With<NoFrustumCulling>,
    >,
) {
    for (frustum, camera, gtransform) in view_query.iter() {
        if !camera.is_active {
            continue;
        }

        let chunk_cam_pos = chunk_pos(gtransform.translation());

        let total = AtomicU32::new(0);
        let visible = AtomicU32::new(0);

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
            *visibility = if !face_visible(&chunk_cam_pos, chunk_pos(coord.translation), face)
                || !frustum.intersects_sphere(&world_sphere, false)
                || !intersects_aabb(frustum, &world_aabb)
            {
                Visibility::Hidden
            } else {
                visible.fetch_add(1, Ordering::AcqRel);
                Visibility::Visible
            };
        });
    }
}
