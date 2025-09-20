// Follow the bevy Aabb2d API: https://docs.rs/bevy_math/latest/bevy_math/bounding/struct.Aabb2d.html

use crate::math::vec2::Vec2;

pub struct Aabb2d {
    pub min: Vec2,
    pub max: Vec2,
}