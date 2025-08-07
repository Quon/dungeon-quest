use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::math::{Vec2, Vec3, Vec3Swizzles};

pub fn collide(a_pos :Vec3,
           a_size :Vec2,
           b_pos :Vec3,
           b_size :Vec2 ) -> bool {
    let collision = Aabb2d::new(a_pos.truncate().trunc(), a_size / 2.)
        .intersects(&Aabb2d::new(b_pos.truncate().trunc(), b_size / 2.));
    collision
}
